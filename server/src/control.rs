use serde::{Deserialize, Serialize};

use crate::osc::OscSender;
use crate::palette::{
    ControlMessage as PaletteControlMessage, Palette, StateChange as PaletteStateChange,
};
use crate::subscriber::{
    ControlMessage as SubscriberControlMessage, StateChange as SubscriberStateChange, Subscribers,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlMessage {
    Palette(PaletteControlMessage),
    Subscriber(SubscriberControlMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateChange {
    Palette(PaletteStateChange),
    Subscriber(SubscriberStateChange),
}

pub struct Dispatcher {
    osc_sender: OscSender,
    palette: Palette,
    subs: Subscribers,
}

impl Dispatcher {
    pub fn new(osc_sender: OscSender, palette: Palette) -> Self {
        Self {
            osc_sender,
            palette,
            subs: Subscribers::new(),
        }
    }

    pub fn control(&mut self, msg: ControlMessage) {
        use ControlMessage::*;
        match msg {
            Palette(m) => {
                match self.palette.control(m) {
                    PaletteStateChange::Set(colors) => {
                        self.subs.send_palette(&colors, &self.osc_sender);
                        // TODO: send palette update to all controllers
                    }
                }
            }
            Subscriber(m) => match self.subs.control(m) {
                SubscriberStateChange::Added(sub) => {
                    self.subs
                        .send_palette_to(sub.id(), self.palette.colors(), &self.osc_sender);
                    // TODO: send added subscriber to controllers
                }
                SubscriberStateChange::Removed(_) => {
                    // TODO: send state change to controllers
                }
            },
        }
    }
}
