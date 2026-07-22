mod app;
mod hud;

use app::ApacheApp;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Apache")
            .with_inner_size([120.0, 60.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Apache",
        options,
        Box::new(|_cc| Ok(Box::new(ApacheApp::default()))),
    )
}
