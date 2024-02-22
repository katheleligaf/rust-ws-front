pub mod link_pannel;
pub mod rate_pannel;
pub use link_pannel::LinkPannel;
pub use rate_pannel::RatePannel;
use eframe::egui::{ self };

use super::miti_ws::MitiTrace;
pub trait TracePannel: Sized {
    fn ui(&mut self, ctx: &egui::Context);
}

pub struct TracePannels {
    lpannel: Option<LinkPannel>,
    rpannel: Option<RatePannel>,
    trace: Option<MitiTrace>,
}
impl TracePannels {
    pub fn set_lpannel(&mut self, lpannel: Option<LinkPannel>) {
        self.lpannel = lpannel;
    }
    pub fn set_rpannel(&mut self, rpannel: Option<RatePannel>) {
        self.rpannel = rpannel;
    }
    pub fn is_set_lpannel(&mut self) -> bool {
        !self.lpannel.is_none()
    }
    pub fn is_set_rpannel(&mut self) -> bool {
        !self.rpannel.is_none()
    }
}
impl Default for TracePannels {
    fn default() -> Self {
        Self {
            lpannel: None,
            rpannel: None,
            trace: None,
        }
    }
}

impl TracePannel for TracePannels {
    fn ui(&mut self, ctx: &egui::Context) {
        match &mut self.lpannel {
            Some(pan) => {
                pan.ui(ctx);
            }
            _ => {}
        }
        match &mut self.rpannel {
            Some(pan) => {
                pan.ui(ctx);
            }
            _ => {}
        }
    }
}
