use eframe::egui::{self};
use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};

// ExempleApp c'est la Window entière, tout est dedans
pub struct ExampleApp {
    url: String,
    error: String,
    frontsocket: Option<FrontSocket>, //FrontSocket est initialisé quand une socket WS est ouverte
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            url: "ws://ws.ceyraud.com:80".to_owned(), //On defini l'url du serveur ws ici, on peut le modifier ici, ou faire un input text sur le front pour le modifier
            error: Default::default(),
            frontsocket: None, //au depart il n'y a pas de front, je fais peut être changer ça
        }
    }
}
//Ici on implémente le trait "eframe::App", un trait c'est un peu comme surcharger une classe en Java ou Cpp
//En gros ExampleApp est une App eframe, et elle doit renseigner certaines fonctions
impl eframe::App for ExampleApp {
    //La fonction update sert pour mettre a jour l'UI
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx); //Ca pour installer le fait que l'on utilise des images
        {
            //Ici on retrouve un TopPannel, c'est un panneau que l'on greffe au top ou bottom de la page,
            //Ici ce Top Pannel on met une menu bar, avec une option File et le bouton Quit
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    //On defini le bouton au même endroit qu'on defini les actions qui lui sont propres
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            //Ici quand le Quit est cliqué
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            //On ferme l'app
                        }
                    });
                });
            });
        }
        //On remet un Top Pannel, avec le bouton Connect to server, qui quand on clique initialise la
        //connexion websocket

        egui::TopBottomPanel::top("server").show(ctx, |ui| {
            //On crée un rangement horizontal
            ui.horizontal(|ui| {
                //on ajoute un bouton
                if ui.button("Connect to server").clicked() {
                    //qui demarre la connection
                    self.connect(ctx.clone());
                }
            });
        });
        //si une erreur on l'affiche
        if !self.error.is_empty() {
            egui::TopBottomPanel::top("error").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Error:");
                    ui.colored_label(egui::Color32::RED, &self.error);
                });
            });
        }

        //Ici on regarde si la websocket est initialisée, si oui on update l'UI
        if let Some(frontsocket) = &mut self.frontsocket {
            frontsocket.ui(ctx);
        }
    }
}

impl ExampleApp {
    //ici on defini la fonction qui s'occupe de la connexion au serveur websocket
    fn connect(&mut self, ctx: egui::Context) {
        let wakeup = move || ctx.request_repaint(); // wake up UI thread on new message
        match ewebsock::connect_with_wakeup(&self.url, wakeup) {
            Ok((ws_sender, ws_receiver)) => {
                self.frontsocket = Some(FrontSocket::new(ws_sender, ws_receiver));
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
//Cette structure est l'ensemble des windows liés a la connexion websocket
//ici on voit que l'on a la liste des messages recus "MitiIn" dans un vecteur dans events
//On a aussi une Window de type InWindow, c'est la page flottante qui affiche les réponses de la query
struct FrontSocket {
    ws_sender: WsSender,
    ws_receiver: WsReceiver,
    events: Vec<MitiIn>,
    in_window: InWindow,
}

impl FrontSocket {
    // Le constructeur
    fn new(ws_sender: WsSender, ws_receiver: WsReceiver) -> Self {
        Self {
            ws_sender,
            ws_receiver,
            events: Default::default(),
            in_window: Default::default(),
        }
    }
    //La definition de l'UI lié a la connexion websocket
    fn ui(&mut self, ctx: &egui::Context) {
        //on lit les messages reçus par la connexion websocket
        while let Some(event) = self.ws_receiver.try_recv() {
            //un peu comme un switch case
            match event {
                WsEvent::Message(msg) => {
                    match msg {
                        WsMessage::Text(txt) => {
                            //Quand on recoit un message, on le deserialise
                            let resp: MitiIn = serde_json::from_str(&txt).unwrap();
                            //puis on l'ajoute au vec des evenements
                            self.events.push(resp);
                            //On met a jour la donnée dans la window
                            self.in_window
                                .update_in(self.events.last().unwrap().clone());
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        //Un autre toppanelavec une imagebutton
        egui::TopBottomPanel::top("topp_panel").show(ctx, |ui| {
            if ui
                .add(egui::ImageButton::new(egui::include_image!(
                    "../assets/right-arrow.png"
                )))
                .clicked()
            {
                //Si il y a un click sur cette image button, on envoie un message de type MitiOut au serveur
                let m_out = MitiOut::new();
                self.ws_sender
                    .send(WsMessage::Text(serde_json::to_string(&m_out).unwrap()));
            }
        });
        self.in_window.ui_printing_output(ctx);
        self.in_window.ui_load_direction(ctx);
    }
}

//La struct de window d'affichage de la derniere query
struct InWindow {
    m_in: Option<MitiIn>,
}
impl Default for InWindow {
    fn default() -> Self {
        Self { m_in: None }
    }
}
impl InWindow {
    fn update_in(&mut self, msg_in: MitiIn) {
        self.m_in = Some(msg_in);
    }

    fn ui_printing_output(&mut self, ctx: &egui::Context) {
        //on crée une nouvelle window et on y met les valeurs de la derniere query si elle existe
        egui::Window::new("Printing Output").show(ctx, |ui| match &self.m_in {
            Some(msg_in) => {
                ui.label(&msg_in.direction);
                ui.label(&msg_in.rate.to_string());
                ui.label(&msg_in.text.to_string());
                ui.label(&msg_in.roaming.to_string());
            }
            None => {}
        });
    }

    fn ui_load_direction(&mut self, ctx: &egui::Context) {
        let upblack = egui::Image::new(egui::include_image!("../assets/uplink_black.png"))
            .max_size(egui::Vec2::new(100.0, 100.0))
            .maintain_aspect_ratio(true);
        let downblack = egui::Image::new(egui::include_image!("../assets/downlink_black.png"))
            .max_size(egui::Vec2::new(100.0, 100.0))
            .maintain_aspect_ratio(true);
        let downgreen = egui::Image::new(egui::include_image!("../assets/downlink_green.png"))
            .max_size(egui::Vec2::new(100.0, 100.0))
            .maintain_aspect_ratio(true);
        let upblue = egui::Image::new(egui::include_image!("../assets/uplink_blue.png"))
            .max_size(egui::Vec2::new(100.0, 100.0))
            .maintain_aspect_ratio(true);

        egui::Window::new("Direction Window").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
            match &self.m_in {           
            Some(msg_in) => {
                match msg_in.direction.as_str() {
                    "upload" => {
                        ui.add(upblue);
                        ui.add(downblack);
                    }
                    "download" => {
                        ui.add(upblack);
                        ui.add(downgreen);
                    }
                    _ => {
                        ui.add(upblack);
                        ui.add(downblack);}
                }
            }
            None => {
                ui.add(upblack);
                ui.add(downblack);
            }
        }
    });
    });
    }
}

//--------

// les differentes structures envoyé avec le serveur

//Miti In c'est ce que l'on recoit du serveur
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct MitiIn {
    direction: String,
    rate: u16,
    text: char,
    roaming: bool,
}

//C'est ce que l'on envoit au serveur
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct MitiOut {
    request: String,
    index: u16,
    test: String,
}
impl MitiOut {
    // Custom constructor to create a MitiOut instance
    pub fn new() -> Self {
        Self {
            request: "data".to_owned(),
            index: 12,
            test: "Test".to_owned(),
        }
    }
}
