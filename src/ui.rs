use eframe::egui;
use rfd;
use std::process::Command;

pub struct App {
    input_path: String,
    output_format: &'static str,
    default_save_path: String,
    status_text: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            input_path: String::new(),
            output_format: "MP3",
            default_save_path: String::new(),
            status_text: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lirel");
            ui.label("Конвертер видео в аудио");

            ui.separator();

            ui.heading("Настройки");
            ui.label("Путь по умолчанию для сохранения:");
            ui.text_edit_singleline(&mut self.default_save_path);
            if ui.button("Выбрать папку").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.default_save_path = path.display().to_string();
                }
            }

            ui.separator();

            ui.heading("Конвертация видео в аудио");
            if ui.button("Выбрать файл").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.input_path = path.display().to_string();
                }
            }

            if !self.input_path.is_empty() {
                ui.label(format!("Выбранный файл: {}", self.input_path.split('/').last().unwrap_or("Нет файла")));
            }

            ui.label("Формат конвертации:");
            egui::ComboBox::from_label("Формат")
                .selected_text(self.output_format)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.output_format, "MP3", "MP3");
                    ui.selectable_value(&mut self.output_format, "WAV", "WAV");
                    ui.selectable_value(&mut self.output_format, "FLAC", "FLAC");
                    ui.selectable_value(&mut self.output_format, "OGG", "OGG");
                    ui.selectable_value(&mut self.output_format, "AAC", "AAC");
                });

            if ui.button("Конвертировать").clicked() {
                if !self.input_path.is_empty() {
                    if let Err(e) = self.convert_video_to_audio(&self.input_path, self.output_format, &self.default_save_path) {
                        self.status_text = format!("Ошибка: {}", e);
                    } else {
                        self.status_text = "Конвертация успешно выполнена!".to_string();
                    }
                } else {
                    self.status_text = "Путь к файлу не указан".to_string();
                }
            }

            ui.label(&self.status_text);
        });
    }
}

impl App {
    fn convert_video_to_audio(&self, input_path: &str, output_format: &str, default_save_path: &str) -> Result<(), std::io::Error> {
        let output_path = format!("{}/output.{}", default_save_path, output_format.to_lowercase());

        let status = Command::new("ffmpeg")
            .args(&["-i", input_path, &output_path])
            .status()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Ошибка запуска ffmpeg: {}", e)))?;

        if status.success() {
            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "FFmpeg завершился с ошибкой"))
        }
    }
}
