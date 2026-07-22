use eframe::egui;

use crate::hud::Hud;

pub struct ApacheApp{
    hud: Hud,
}
impl Default for ApacheApp {
    fn default() -> Self {
        Self {
            hud: Hud::new(),
        }
    }
}

impl eframe::App for ApacheApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

           self.hud.update();          
      
          egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(15.0);
                ui.heading(self.hud.text());
            });
        });

       ctx.request_repaint();

    }
}
