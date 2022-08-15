use log::error;
use serde::{Deserialize, Serialize};
use shared::{ControlMessage, PaletteStateChange, StateChange, SubscriberStateChange};

use crate::client::Clients;
use crate::osc::OscSender;
use crate::palette::Palette;
use crate::subscriber::Subscribers;

pub struct Dispatcher {
    osc_sender: OscSender,
    palette: Palette,
    subs: Subscribers,
    clients: Clients,
}

impl Dispatcher {
    pub fn new(osc_sender: OscSender, palette: Palette, clients: Clients) -> Self {
        Self {
            osc_sender,
            palette,
            subs: Subscribers::new(),
            clients,
        }
    }

    pub fn control(&mut self, msg: ControlMessage) {
        use ControlMessage::*;
        let state_change = match msg {
            Palette(m) => {
                let control_result = self.palette.control(m);
                if let PaletteStateChange::Set(ref colors) = control_result {
                    self.subs.send_palette(&colors, &self.osc_sender);
                }
                StateChange::Palette(control_result)
            }
            Subscriber(m) => {
                let control_result = self.subs.control(m);
                if let SubscriberStateChange::Added(ref sub) = control_result {
                    self.subs
                        .send_palette_to(sub.id, self.palette.colors(), &self.osc_sender);
                }
                StateChange::Subscriber(control_result)
            }
        };
        if let Err(e) = self.clients.send_state_update(&state_change) {
            error!(
                "Failed to send state update to clients: {}.\nMissed update:\n{:?}",
                e, state_change
            );
        }
    }
}
