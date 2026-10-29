use eframe::egui;
use std::process::Command;

pub struct App {
    input_path: String,
    status_text: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            input_path: String::new(),
            status_text: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lirel");
            ui.label("Привет! Это базовый интерфейс для вашего проекта.");
            ui.separator();
            ui.heading("Функции:");
            ui.label("1. Конвертация видео в аудио");

            ui.text_edit_singleline(&mut self.input_path);

            if ui.button("Конвертировать видео в аудио").clicked() {
                if !self.input_path.is_empty() {
                    if let Err(e) = self.convert_video_to_audio(&self.input_path) {
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
    fn convert_video_to_audio(&self, input_path: &str) -> Result<(), std::io::Error> {
        let output_path = "output.mp3"; // Замените на путь к выходному аудио файлу

        let status = Command::new("ffmpeg")
            .args(&["-i", input_path, output_path])
            .status()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Ошибка запуска ffmpeg: {}", e)))?;

        if status.success() {
            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "FFmpeg завершился с ошибкой"))
        }
    }
}
