//! PoE Item Analyzer Desktop Application

mod app;
mod ui;

use app::AnalyzerApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("PoE Item Analyzer"),
        ..Default::default()
    };

    eframe::run_native(
        "PoE Item Analyzer",
        options,
        Box::new(|cc| Box::new(AnalyzerApp::new(cc))),
    )
}
