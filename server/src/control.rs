use log::error;
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
        match msg {
            Palette(m) => {
                let control_result = self.palette.control(m);
                if let PaletteStateChange::Set(ref colors) = control_result {
                    self.subs.send_palette(&colors, &self.osc_sender);
                }
                self.send_to_clients(StateChange::Palette(control_result))
            }
            Subscriber(m) => {
                let control_result = self.subs.control(m);
                if let SubscriberStateChange::Added(ref sub) = control_result {
                    self.subs
                        .send_palette_to(sub.id, self.palette.colors(), &self.osc_sender);
                }
                self.send_to_clients(StateChange::Subscriber(control_result))
            }
            Refresh => {
                self.send_to_clients(StateChange::Palette(self.palette.current_state()));
                for sc in self.subs.current_state() {
                    self.send_to_clients(StateChange::Subscriber(sc));
                }
            }
        };
    }

    fn send_to_clients(&self, sc: StateChange) {
        if let Err(e) = self.clients.send_state_update(&sc) {
            error!(
                "Failed to send state update to clients: {}.\nMissed update:\n{:?}",
                e, sc
            );
        }
    }
}
