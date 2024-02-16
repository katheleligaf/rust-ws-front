use eframe::egui::{ self, Event };
use ewebsock::{ WsEvent, WsMessage, WsReceiver, WsSender };

pub struct ExampleApp {
    url: String,
    error: String,
    frontend: Option<FrontEnd>,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            url: "ws://ws.ceyraud.com:80".to_owned(),
            error: Default::default(),
            frontend: None,
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
            });
        }

        egui::TopBottomPanel::top("server").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Connect to server").clicked() {
                    self.connect(ctx.clone());
                }
            });
        });

        if !self.error.is_empty() {
            egui::TopBottomPanel::top("error").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Error:");
                    ui.colored_label(egui::Color32::RED, &self.error);
                });
            });
        }

        if let Some(frontend) = &mut self.frontend {
            frontend.ui(ctx);
        }
    }
}

impl ExampleApp {
    fn connect(&mut self, ctx: egui::Context) {
        let wakeup = move || ctx.request_repaint(); // wake up UI thread on new message
        match ewebsock::connect_with_wakeup(&self.url, wakeup) {
            Ok((ws_sender, ws_receiver)) => {
                self.frontend = Some(FrontEnd::new(ws_sender, ws_receiver));
                self.error.clear();
            }
            Err(error) => {
                log::error!("Failed to connect to {:?}: {}", &self.url, error);
                self.error = error;
            }
        }
    }
}

// ----------------------------------------------------------------------------

struct FrontEnd {
    ws_sender: WsSender,
    ws_receiver: WsReceiver,
    events: Vec<MitiIn>,
}

impl FrontEnd {
    fn new(ws_sender: WsSender, ws_receiver: WsReceiver) -> Self {
        Self {
            ws_sender,
            ws_receiver,
            events: Default::default(),
        }
    }

    fn ui(&mut self, ctx: &egui::Context) {
        while let Some(event) = self.ws_receiver.try_recv() {
            match event {
                WsEvent::Message(msg) => {
                    match msg {
                        WsMessage::Text(txt) => {
                            let resp: MitiIn = serde_json::from_str(&txt).unwrap();
                            self.events.push(resp);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Send a query").clicked() {
                    self.ws_sender.send(
                        WsMessage::Text(serde_json::to_string(&MitiOut::new()).unwrap())
                    );
                }
            });
            ui.separator();
            ui.heading("Received events:");

            for event in &self.events {
                ui.label(format!("{:?}", event));
            }
        });
    }
}

//--------

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct MitiIn {
    direction: String,
    rate: u16,
    text: char,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct MitiOut {
    request: String,
    index: u16,
}

impl MitiOut {
    // Custom constructor to create a default Resp instance
    pub fn new() -> Self {
        Self {
            request: "data".to_owned(),
            index: 12,
        }
    }
}
