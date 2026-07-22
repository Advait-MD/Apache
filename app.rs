use eframe::egui;

use crate::hud::Hud;

pub struct ApacheApp{
    hud: Hud,
    hud_x: f32,
    hud_y: f32,
}
impl Default for ApacheApp {
    fn default() -> Self {
        Self {
            hud: Hud::new(),
            hud_x: 200.0,
            hud_y: 200.0,      
 
       }
    }
}

impl eframe::App for ApacheApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
         
         self.hud.update();          
        
    if let Some((x, y)) = crate::platform::cursor::position() {    
       
        
        let target_x = x as f32 + 1.0;
        let target_y = y as f32 - 1.0;

        self.hud_x += (target_x - self.hud_x) * 0.15;
        self.hud_y += (target_y - self.hud_y) * 0.15; 
        
         ctx.send_viewport_cmd(
                   egui::ViewportCommand::OuterPosition(
                       egui::pos2(self.hud_x, self.hud_y),
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
