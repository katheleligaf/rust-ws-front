use ewebsock::{ WsEvent, WsMessage, WsReceiver, WsSender };
use serde::{ Deserialize, Serialize };

pub struct MitiWs {
    ws_sender: WsSender,
    ws_receiver: WsReceiver,
}

impl MitiWs {
    pub fn new(url: String, ctx: eframe::egui::Context) -> Self {
        let wakeup = move || ctx.request_repaint();
        match ewebsock::connect_with_wakeup(url, wakeup) {
            Ok((wsender, wreceiver)) => {
                Self {
                    ws_sender: wsender,
                    ws_receiver: wreceiver,
                }
            }
            Err(error) => {
                panic!("{}", error);
            }
        }
    }
    pub fn try_receive(&mut self) -> Option<MitiTrace> {
        let opt_event = self.ws_receiver.try_recv();
        match opt_event {
            Some(event) => {
                match event {
                    WsEvent::Message(msg) => {
                        match msg {
                            WsMessage::Text(txt) => {
                                let resp: MitiTrace = serde_json::from_str(&txt).unwrap();
                                return Some(resp);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            None => {
                return None;
            }
        }
        return None;
    }
    pub fn send(&mut self, msg: &MitiTrace) {
        self.ws_sender.send(WsMessage::Text(serde_json::to_string(msg).unwrap()));
    }
}

#[derive(Serialize, Deserialize)]
pub struct MitiTrace {
    direction: String,
    rate: u16,
    text: char,
    roaming: bool,
}
