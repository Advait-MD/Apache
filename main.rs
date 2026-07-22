mod app;
mod hud;
mod metrics;
mod platform;

use app::ApacheApp;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Apache")
            .with_inner_size([120.0, 60.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Apache",
        options,
        Box::new(|_cc| Ok(Box::new(ApacheApp::default()))),
    )
}
