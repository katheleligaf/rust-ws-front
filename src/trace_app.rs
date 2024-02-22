use eframe::egui::{ self };
mod trace_pannels;
mod miti_ws;
use trace_pannels::TracePannel;
use trace_pannels::TracePannels;
use link_pannel::LinkPannel;
use rate_pannel::RatePannel;

use self::miti_ws::MitiWs;
use self::trace_pannels::link_pannel;
use self::trace_pannels::rate_pannel;

pub struct TraceFront {
    url: String,
    pannels: TracePannels,
    mitiws: Option<MitiWs>,
}

const DEFAULT_URL: &str = "ws://127.0.0.1:8002";
impl Default for TraceFront {
    fn default() -> Self {
        Self {
            url: DEFAULT_URL.to_string(), //On defini l'url du serveur ws ici, on peut le modifier ici, ou faire un input text sur le front pour le modifier
            pannels: Default::default(),
            mitiws: None,
        }
    }
}

impl eframe::App for TraceFront {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close); //On ferme l'app
                    }
                });
                ui.menu_button("Pannels", |ui| {
                    //TODO Ugly if chain, to fix but need to understand how to do that ...
                    if self.pannels.is_set_lpannel() {
                        if ui.button("Remove LinkPannel").clicked() {
                            self.pannels.set_lpannel(None);
                        }
                    } else {
                        if ui.button("Add LinkPannel").clicked() {
                            self.pannels.set_lpannel(
                                Some(LinkPannel::new(Some("Test".to_owned())))
                            );
                        }
                    }
                    if self.pannels.is_set_rpannel() {
                        if ui.button("Remove RatePannel").clicked() {
                            self.pannels.set_rpannel(None);
                        }
                    } else {
                        if ui.button("Add RatePannel").clicked() {
                            self.pannels.set_rpannel(Some(RatePannel::new(Some(12))));
                        }
                    }
                });
                match &self.mitiws {
                    Some(_) => {}
                    None => {}
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |_| {});
        self.pannels.ui(ctx);
    }
}
