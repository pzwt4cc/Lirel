use crate::{AppWindow, Tr};
use slint::{ComponentHandle, SharedString};

/// lang: 0 = русский, 1 = английский
pub fn apply(ui: &AppWindow, lang: i32) {
    let tr = ui.global::<Tr>();
    let t = |ru: &str, en: &str| SharedString::from(if lang == 0 { ru } else { en });

    tr.set_subtitle(t("Свободный и классный конвертер. Выбери, что конвертируем.", "A free and awesome converter. Pick what to convert."));
    tr.set_settings(t("Настройки", "Settings"));
    tr.set_back(t("Назад", "Back"));
    tr.set_soon(t("Скоро", "Soon"));

    tr.set_language(t("Язык", "Language"));
    tr.set_theme(t("Тема", "Theme"));
    tr.set_theme_system(t("Системная", "System"));
    tr.set_theme_light(t("Светлая", "Light"));
    tr.set_theme_dark(t("Тёмная", "Dark"));
    tr.set_accent(t("Цвет приложения", "App color"));
    tr.set_output_dir(t("Папка для готовых файлов", "Output folder"));
    tr.set_choose_folder(t("Выбрать папку", "Choose folder"));
    tr.set_behavior(t("Поведение", "Behavior"));
    tr.set_open_when_done(t("Открывать папку после конвертации", "Open folder when done"));
    tr.set_open_when_done_hint(t("Результат сразу появится в файловом менеджере", "Show the result in your file manager right away"));

    tr.set_choose_files(t("Выбрать файлы", "Choose files"));
    tr.set_add_more(t("Добавить ещё", "Add more"));
    tr.set_clear_all(t("Очистить", "Clear"));
    tr.set_empty_title(t("Выбери один или несколько файлов", "Choose one or more files"));
    tr.set_empty_hint(t("Нажми сюда, чтобы открыть выбор файлов", "Click here to open the file picker"));
    tr.set_format(t("Формат", "Format"));
    tr.set_convert(t("Конвертировать", "Convert"));
    tr.set_converting(t("Конвертирую...", "Converting..."));
    tr.set_cancel(t("Отмена", "Cancel"));
    tr.set_open_folder(t("Открыть папку", "Open folder"));
    tr.set_state_done(t("Готово", "Done"));
}

fn files_word_ru(n: usize) -> &'static str {
    let (m10, m100) = (n % 10, n % 100);
    if m10 == 1 && m100 != 11 {
        "файл"
    } else if (2..=4).contains(&m10) && !(12..=14).contains(&m100) {
        "файла"
    } else {
        "файлов"
    }
}

pub fn summary_none(lang: i32) -> String {
    (if lang == 0 { "Файлы не выбраны" } else { "No files selected" }).into()
}

pub fn summary_many(lang: i32, n: usize) -> String {
    if lang == 0 {
        format!("Выбрано: {n} {}", files_word_ru(n))
    } else {
        format!("{n} files selected")
    }
}

pub fn msg_no_ffmpeg(lang: i32) -> String {
    (if lang == 0 {
        "ffmpeg не найден. Установи его (Arch: sudo pacman -S ffmpeg)"
    } else {
        "ffmpeg not found. Please install it (Arch: sudo pacman -S ffmpeg)"
    })
    .into()
}

pub fn msg_failed(lang: i32, detail: &str) -> String {
    if lang == 0 {
        format!("Ошибка: {detail}")
    } else {
        format!("Error: {detail}")
    }
}

pub fn msg_cancelled(lang: i32) -> String {
    (if lang == 0 { "Отменено" } else { "Cancelled" }).into()
}

pub fn msg_batch(lang: i32, ok: usize, failed: usize) -> String {
    match (lang, failed) {
        (0, 0) => format!("Готово! Сконвертировано: {ok}"),
        (0, _) if ok == 0 => format!("Не удалось сконвертировать файлов: {failed}"),
        (0, _) => format!("Готово: {ok}, с ошибками: {failed}"),
        (_, 0) => format!("Done! Converted: {ok}"),
        (_, _) if ok == 0 => format!("Failed to convert {failed} file(s)"),
        _ => format!("Done: {ok}, failed: {failed}"),
    }
}