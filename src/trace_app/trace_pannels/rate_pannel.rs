use eframe::egui::{ self };

use super::TracePannel;

pub struct RatePannel {
    rate: Option<u16>,
}

impl TracePannel for RatePannel {
    fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("RatePannel").show(ctx, |ui| {
            match &self.rate {
                Some(a) => {
                    ui.label(a.to_string());
                }
                None => {
                    ui.label("nothing");
                }
            }
        });
    }
}

impl Default for RatePannel {
    fn default() -> Self {
        Self {
            rate: None, //On defini l'url du serveur ws ici, on peut le modifier ici, ou faire un input text sur le front pour le modifier
        }
    }
}

impl RatePannel {
    /* 
    pub fn set_rate(&mut self, r: Option<u16>) {
        self.rate = r;
    }*/
    pub fn new(r: Option<u16>) -> Self {
        Self {
            rate: r,
        }
    }
}
