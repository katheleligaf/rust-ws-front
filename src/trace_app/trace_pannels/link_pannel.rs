use eframe::egui::{ self };

use super::TracePannel;

pub struct LinkPannel {
    direction: Option<String>,
}

impl TracePannel for LinkPannel {
    fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("LinkPannel").show(ctx, |ui| {
            match &self.direction {
                Some(a) => {
                    ui.label(a);
                }
                None => {
                    ui.label("nothing");
                }
            }
        });
    }
}

impl Default for LinkPannel {
    fn default() -> Self {
        Self {
            direction: None,
        }
    }
}

impl LinkPannel {
    /* 
    pub fn set_direction(&mut self, dir: Option<String>) {
        self.direction = dir;
    }*/
    pub fn new(dir: Option<String>) -> Self {
        Self {
            direction: dir,
        }
    }
}
