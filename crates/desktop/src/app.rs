//! Main application state

use egui::Context;

/// Main application state
pub struct AnalyzerApp {
    // TODO: Add application state
}

impl AnalyzerApp {
    /// Create a new application
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for AnalyzerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PoE Item Analyzer");
            ui.separator();
            ui.label("Welcome! Phase 1 complete - basic structure is ready.");
            ui.label("Next: Implement LUT data loading and analysis logic.");
        });
    }
}
