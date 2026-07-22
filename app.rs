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
        
    if let Some((x, y)) = crate::platform::cursor::position() {    

         ctx.send_viewport_cmd(
                   egui::ViewportCommand::OuterPosition(
                       egui::pos2( x as f32 + 5.0, y as f32 - 5.0,),
            ),
          );     
        }
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
