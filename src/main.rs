mod app;
pub use app::MyApp;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Code Editor",
        options,
        Box::new(|cc| Box::new(app::MyApp::new(cc))),
    )   
}