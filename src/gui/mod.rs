use crate::{
    backend::{
        convert::{self, ConvertError},
        tools::{Tool, CATEGORIES, L},
    },
    utils::{config::Config, i18n, system},
    AppState, AppWindow, CategoryData, FileItem, OptionGroup, TileData,
};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel, Weak};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};

// ---------- помощники ----------

fn lt(lang: i32, l: L) -> SharedString {
    SharedString::from(if lang == 0 { l.0 } else { l.1 })
}

fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut v = bytes as f64;
    let mut i = 0;
    while v >= 1024.0 && i < UNITS.len() - 1 {
        v /= 1024.0;
        i += 1;
    }
    if i == 0 { format!("{bytes} B") } else { format!("{v:.1} {}", UNITS[i]) }
}

fn make_item(path: &Path) -> FileItem {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let ext: String = path
        .extension()
        .map(|e| e.to_string_lossy().to_uppercase())
        .unwrap_or_default()
        .chars()
        .take(4)
        .collect();
    let size = fs::metadata(path).map(|m| human_size(m.len())).unwrap_or_default();

    FileItem {
        name: name.into(),
        path: path.display().to_string().into(),
        ext: ext.into(),
        size: size.into(),
        state: 0,
        progress: 0.0,
        note: "".into(),
    }
}

/// Доступ к VecModel со списком файлов (только из потока интерфейса).
fn with_files<R>(ui: &AppWindow, f: impl FnOnce(&VecModel<FileItem>) -> R) -> Option<R> {
    let model = ui.global::<AppState>().get_files();
    model.as_any().downcast_ref::<VecModel<FileItem>>().map(f)
}

/// Обновить одну строку списка из любого потока.
fn update_row(weak: &Weak<AppWindow>, i: usize, f: impl FnOnce(&mut FileItem) + Send + 'static) {
    let _ = weak.upgrade_in_event_loop(move |ui| {
        with_files(&ui, |m| {
            if let Some(mut row) = m.row_data(i) {
                f(&mut row);
                m.set_row_data(i, row);
            }
        });
    });
}

/// Подпись в большой карточке: имя файла / «Выбрано: N файлов».
fn refresh_summary(ui: &AppWindow) {
    let st = ui.global::<AppState>();
    let lang = st.get_language();
    let files = st.get_files();
    let text = match files.row_count() {
        0 => i18n::summary_none(lang),
        1 => files.row_data(0).map(|f| f.name.to_string()).unwrap_or_default(),
        n => i18n::summary_many(lang, n),
    };
    st.set_summary(text.into());
}

/// Главная: разделы (Фото / Видео / Аудио) с плитками. Пересобирается при смене языка.
fn refresh_tiles(ui: &AppWindow) {
    let st = ui.global::<AppState>();
    let lang = st.get_language();

    let categories: Vec<CategoryData> = CATEGORIES
        .iter()
        .enumerate()
        .map(|(ci, title)| {
            let tiles: Vec<TileData> = Tool::ALL
                .iter()
                .filter(|t| t.category() == ci)
                .map(|t| {
                    let (src, dst) = t.tile();
                    TileData {
                        src: src.into(),
                        dst: dst.into(),
                        title: lt(lang, t.title()),
                        available: true,
                        tool: *t as i32,
                    }
                })
                .collect();
            CategoryData {
                title: lt(lang, *title),
                tiles: ModelRc::new(VecModel::from(tiles)),
            }
        })
        .collect();

    st.set_categories(ModelRc::new(VecModel::from(categories)));
}

/// Загрузить форматы и настройки конвертора на экран.
/// `reset = true` — сбросить выбор на значения по умолчанию, `false` — сохранить (смена языка).
fn load_tool(ui: &AppWindow, tool: Tool, reset: bool) {
    let st = ui.global::<AppState>();
    let lang = st.get_language();

    st.set_tool(tool as i32);
    st.set_tool_title(lt(lang, tool.title()));

    if reset {
        let formats: Vec<SharedString> = tool.formats().iter().map(|f| (*f).into()).collect();
        st.set_formats(ModelRc::new(VecModel::from(formats)));
        st.set_format_index(0);
    }

    let old: Vec<i32> = if reset {
        Vec::new()
    } else {
        st.get_options().iter().map(|g| g.selected).collect()
    };

    let groups: Vec<OptionGroup> = tool
        .options()
        .iter()
        .enumerate()
        .map(|(i, o)| {
            let choices: Vec<SharedString> = o.choices.iter().map(|c| lt(lang, *c)).collect();
            OptionGroup {
                title: lt(lang, o.title),
                choices: ModelRc::new(VecModel::from(choices)),
                selected: old.get(i).copied().unwrap_or(o.default as i32),
            }
        })
        .collect();
    st.set_options(ModelRc::new(VecModel::from(groups)));
}

// ---------- настройка обработчиков ----------

pub fn setup(ui: &AppWindow) {
    let cfg = Arc::new(Mutex::new(Config::load()));
    let cancel = Arc::new(AtomicBool::new(false));
    let state = ui.global::<AppState>();

    state.set_files(ModelRc::new(VecModel::<FileItem>::default()));

    {
        let c = cfg.lock().unwrap();
        state.set_language(c.language_index());
        state.set_theme(c.theme_index());
        state.set_accent_index(c.accent.clamp(0, 7));
        state.set_open_when_done(c.open_when_done);
        state.set_output_dir(c.output_dir.as_str().into());
    }
    i18n::apply(ui, state.get_language());
    refresh_tiles(ui);
    load_tool(ui, Tool::VideoAudio, true);
    refresh_summary(ui);

    // --- язык ---
    state.on_language_changed({
        let weak = ui.as_weak();
        let cfg = cfg.clone();
        move |idx| {
            cfg.lock().unwrap().set_language(idx);
            if let Some(ui) = weak.upgrade() {
                i18n::apply(&ui, idx);
                refresh_tiles(&ui);
                let tool = Tool::from_index(ui.global::<AppState>().get_tool());
                load_tool(&ui, tool, false);
                refresh_summary(&ui);
            }
        }
    });

    // --- тема ---
    state.on_theme_changed({
        let cfg = cfg.clone();
        move |idx| cfg.lock().unwrap().set_theme(idx)
    });

    // --- акцентный цвет ---
    state.on_accent_changed({
        let cfg = cfg.clone();
        move |idx| cfg.lock().unwrap().set_accent(idx)
    });

    // --- «открывать папку после конвертации» ---
    state.on_open_when_done_changed({
        let cfg = cfg.clone();
        move |on| cfg.lock().unwrap().set_open_when_done(on)
    });

    // --- открыть конвертор ---
    state.on_open_tool({
        let weak = ui.as_weak();
        move |idx| {
            let Some(ui) = weak.upgrade() else { return };
            let st = ui.global::<AppState>();
            if st.get_busy() {
                return;
            }
            // Тот же конвертор — оставляем список файлов как был.
            if st.get_tool() != idx {
                with_files(&ui, |m| m.set_vec(Vec::new()));
                st.set_status("".into());
                st.set_status_error(false);
                st.set_done(false);
                load_tool(&ui, Tool::from_index(idx), true);
                refresh_summary(&ui);
            }
            st.set_page(2);
        }
    });

    // --- выбор пункта в настройках конвертора ---
    state.on_option_selected({
        let weak = ui.as_weak();
        move |group, choice| {
            let Some(ui) = weak.upgrade() else { return };
            let options = ui.global::<AppState>().get_options();
            if let Some(model) = options.as_any().downcast_ref::<VecModel<OptionGroup>>() {
                let g = group.max(0) as usize;
                if let Some(mut row) = model.row_data(g) {
                    row.selected = choice;
                    model.set_row_data(g, row);
                }
            }
        }
    });

    // --- выбор файлов (можно несколько), фильтр зависит от конвертора ---
    state.on_pick_input({
        let weak = ui.as_weak();
        move || {
            let Some(ui) = weak.upgrade() else { return };
            let exts = Tool::from_index(ui.global::<AppState>().get_tool()).input_exts();
            let weak = weak.clone();
            thread::spawn(move || {
                let Some(paths) = rfd::FileDialog::new().add_filter("Files", exts).pick_files()
                else {
                    return;
                };
                let _ = weak.upgrade_in_event_loop(move |ui| {
                    with_files(&ui, |m| {
                        let existing: Vec<String> = (0..m.row_count())
                            .filter_map(|i| m.row_data(i))
                            .map(|f| f.path.to_string())
                            .collect();
                        for p in &paths {
                            if !existing.contains(&p.display().to_string()) {
                                m.push(make_item(p));
                            }
                        }
                    });
                    let st = ui.global::<AppState>();
                    st.set_status("".into());
                    st.set_status_error(false);
                    st.set_done(false);
                    refresh_summary(&ui);
                });
            });
        }
    });

    // --- убрать один файл из списка ---
    state.on_remove_file({
        let weak = ui.as_weak();
        move |idx| {
            let Some(ui) = weak.upgrade() else { return };
            if ui.global::<AppState>().get_busy() {
                return;
            }
            with_files(&ui, |m| {
                if idx >= 0 && (idx as usize) < m.row_count() {
                    m.remove(idx as usize);
                }
            });
            refresh_summary(&ui);
        }
    });

    // --- очистить список ---
    state.on_clear_files({
        let weak = ui.as_weak();
        move || {
            let Some(ui) = weak.upgrade() else { return };
            let st = ui.global::<AppState>();
            if st.get_busy() {
                return;
            }
            with_files(&ui, |m| m.set_vec(Vec::new()));
            st.set_status("".into());
            st.set_status_error(false);
            st.set_done(false);
            refresh_summary(&ui);
        }
    });

    // --- выбор папки сохранения ---
    state.on_pick_output_dir({
        let weak = ui.as_weak();
        let cfg = cfg.clone();
        move || {
            let weak = weak.clone();
            let cfg = cfg.clone();
            thread::spawn(move || {
                if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                    let dir = dir.display().to_string();
                    cfg.lock().unwrap().set_output_dir(dir.clone());
                    let _ = weak.upgrade_in_event_loop(move |ui| {
                        ui.global::<AppState>().set_output_dir(dir.into());
                    });
                }
            });
        }
    });

    // --- открыть папку с результатом ---
    state.on_open_folder({
        let weak = ui.as_weak();
        move || {
            if let Some(ui) = weak.upgrade() {
                let dir = ui.global::<AppState>().get_output_dir().to_string();
                system::open_dir(Path::new(&dir));
            }
        }
    });

    // --- отмена ---
    state.on_cancel({
        let cancel = cancel.clone();
        move || cancel.store(true, Ordering::Relaxed)
    });

    // --- конвертация всех файлов по очереди ---
    state.on_convert({
        let weak = ui.as_weak();
        let cancel = cancel.clone();
        move || {
            let Some(ui) = weak.upgrade() else { return };
            let st = ui.global::<AppState>();
            if st.get_busy() {
                return;
            }
            let lang = st.get_language();

            // Берём только те файлы, которые ещё не готовы.
            let files = st.get_files();
            let todo: Vec<(usize, PathBuf)> = (0..files.row_count())
                .filter_map(|i| files.row_data(i).map(|f| (i, f)))
                .filter(|(_, f)| f.state != 2)
                .map(|(i, f)| (i, PathBuf::from(f.path.as_str())))
                .collect();
            if todo.is_empty() {
                return;
            }

            // Сбрасываем ошибки у тех, что пойдут в работу.
            with_files(&ui, |m| {
                for (i, _) in &todo {
                    if let Some(mut r) = m.row_data(*i) {
                        r.state = 0;
                        r.progress = 0.0;
                        r.note = "".into();
                        m.set_row_data(*i, r);
                    }
                }
            });

            // План ffmpeg собираем из выбранного конвертора, формата и настроек.
            let tool = Tool::from_index(st.get_tool());
            let opts: Vec<usize> = st
                .get_options()
                .iter()
                .map(|g| g.selected.max(0) as usize)
                .collect();
            let plan = tool.plan(st.get_format_index().max(0) as usize, &opts);
            let out_dir = PathBuf::from(st.get_output_dir().to_string());
            let out_dir_final = out_dir.clone();

            cancel.store(false, Ordering::Relaxed);
            st.set_busy(true);
            st.set_done(false);
            st.set_status("".into());
            st.set_status_error(false);

            let weak = weak.clone();
            let cancel = cancel.clone();
            thread::spawn(move || {
                let mut ok = 0usize;
                let mut failed = 0usize;
                let mut cancelled = false;

                for (i, input) in todo {
                    update_row(&weak, i, |r: &mut FileItem| {
                        r.state = 1;
                        r.progress = 0.0;
                    });

                    let w = weak.clone();
                    let result = convert::run(&input, &out_dir, &plan, &cancel, move |p| {
                        update_row(&w, i, move |r: &mut FileItem| r.progress = p);
                    });

                    match result {
                        Ok(_) => {
                            ok += 1;
                            update_row(&weak, i, |r: &mut FileItem| {
                                r.state = 2;
                                r.progress = 1.0;
                            });
                        }
                        Err(ConvertError::Cancelled) => {
                            cancelled = true;
                            update_row(&weak, i, |r: &mut FileItem| {
                                r.state = 0;
                                r.progress = 0.0;
                            });
                            break;
                        }
                        Err(ConvertError::FfmpegMissing) => {
                            failed += 1;
                            let note = i18n::msg_no_ffmpeg(lang);
                            update_row(&weak, i, move |r: &mut FileItem| {
                                r.state = 3;
                                r.note = note.into();
                            });
                            break;
                        }
                        Err(ConvertError::Failed(d)) => {
                            failed += 1;
                            let last = d.lines().last().unwrap_or("").to_string();
                            let note = i18n::msg_failed(lang, &last);
                            update_row(&weak, i, move |r: &mut FileItem| {
                                r.state = 3;
                                r.note = note.into();
                            });
                        }
                    }
                }

                let _ = weak.upgrade_in_event_loop(move |ui| {
                    let st = ui.global::<AppState>();
                    let lang = st.get_language();
                    st.set_busy(false);
                    st.set_done(ok > 0);
                    st.set_status_error(!cancelled && failed > 0 && ok == 0);
                    let msg = if cancelled {
                        i18n::msg_cancelled(lang)
                    } else {
                        i18n::msg_batch(lang, ok, failed)
                    };
                    st.set_status(msg.into());
                    if ok > 0 && !cancelled && st.get_open_when_done() {
                        system::open_dir(&out_dir_final);
                    }
                });
            });
        }
    });
}