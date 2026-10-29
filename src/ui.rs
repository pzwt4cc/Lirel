use eframe::egui;
use ffmpeg_cli_rs::{FFmpeg, FFmpegError};

pub struct App {
    // Здесь можно добавить состояния приложения
}

impl Default for App {
    fn default() -> Self {
        App {}
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

            // Добавляем кнопку для запуска конвертации
            if ui.button("Конвертировать видео в аудио").clicked() {
                if let Err(e) = self.convert_video_to_audio() {
                    ui.label(format!("Ошибка: {}", e));
                } else {
                    ui.label("Конвертация успешно выполнена!");
                }
            }
        });
    }

    fn convert_video_to_audio(&self) -> Result<(), FFmpegError> {
        let ffmpeg = FFmpeg::new();
        ffmpeg
            .input("input.mp4") // Замените на путь к вашему видео файлу
            .output("output.mp3") // Замените на путь к выходному аудио файлу
            .run()
    }
}
