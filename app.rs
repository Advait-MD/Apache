use eframe::egui;

use crate::hud::Hud;

pub struct ApacheApp{
    hud: Hud,
    x: f32,
    y: f32,
}
impl Default for ApacheApp {
    fn default() -> Self {
        Self {
            hud: Hud::new(),
            x: 100.0,
            y: 100.0,
        }
    }
}

impl eframe::App for ApacheApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

         self.hud.update();          
        
         self.x += 2.0;
         ctx.send_viewport_cmd(
                   egui::ViewportCommand::OuterPosition(
                       egui::pos2(self.x, self.y)
            ),
          );     
        
          egui::CentralPanel::default()
             .frame(egui::Frame::NONE)
             .show(ctx, |ui| {
                  ui.vertical_centered(|ui| {
                    ui.add_space(15.0);
                    ui.heading(self.hud.text());
        });
    });
       ctx.request_repaint();

    }
}
