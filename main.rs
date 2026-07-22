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

#[derive(Default)]
struct ApacheApp;

impl eframe::App for ApacheApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(15.0);
                ui.heading("10");
            });
        });
    }
}
