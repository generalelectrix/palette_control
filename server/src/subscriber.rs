use std::error::Error;
use std::{net::SocketAddr, sync::Arc};

use crate::{color::Color, osc::OscSender};
use derive_more::Display;
use log::error;
use rosc::{encoder, OscMessage, OscType};

/// Maintain the collection of palette subscribers.
pub struct Subscribers {
    subs: Vec<Subscriber>,
    next_id: SubscriberId,
}

impl Subscribers {
    pub fn new() -> Self {
        Self {
            subs: Vec::new(),
            next_id: SubscriberId(0),
        }
    }

    pub fn control(&mut self, msg: ControlMessage) -> StateChange {
        match msg {
            ControlMessage::Add(cfg) => {
                let id = self.next_id;
                self.next_id.0 += 1;
                let sub = Subscriber { id, cfg };
                self.subs.push(sub.clone());
                StateChange::Added(sub)
            }
            ControlMessage::Remove(id) => {
                self.subs.retain(|sub| sub.id != id);
                StateChange::Removed(id)
            }
        }
    }

    /// Send the provided palette to all subscribers.
    /// Logs errors.
    pub fn send_palette(&self, colors: &[Color], osc_sender: &OscSender) {
        let osc_encoded = match prepare_osc_palette(colors) {
            Ok(msg) => msg,
            Err(e) => {
                error!("Unable to encode OSC message: {}.", e);
                return;
            }
        };
        for sub in self.subs.iter() {
            self.send_to(sub, osc_encoded.clone(), osc_sender);
        }
    }

    pub fn send_palette_to(&self, id: SubscriberId, colors: &[Color], osc_sender: &OscSender) {
        // Find the right subscriber.
        let sub = if let Some(sub) = self.subs.iter().filter(|s| s.id == id).next() {
            sub
        } else {
            error!("No subscriber found with ID {}.", id);
            return;
        };
        let osc_encoded = match prepare_osc_palette(colors) {
            Ok(msg) => msg,
            Err(e) => {
                error!("Unable to encode OSC message: {}.", e);
                return;
            }
        };
        self.send_to(sub, osc_encoded, osc_sender);
    }

    fn send_to(&self, sub: &Subscriber, osc_encoded: Arc<Vec<u8>>, osc_sender: &OscSender) {
        use SubscriberConfig::*;
        match sub.cfg {
            Osc(addr) => {
                osc_sender.send(addr, osc_encoded);
            }
        }
    }
}

fn prepare_osc_palette(colors: &[Color]) -> Result<Arc<Vec<u8>>, Box<dyn Error>> {
    let mut osc_args = Vec::with_capacity(colors.len() * 3);
    for color in colors {
        osc_args.push(OscType::Float(color.red));
        osc_args.push(OscType::Float(color.green));
        osc_args.push(OscType::Float(color.blue));
    }
    let osc_msg = OscMessage {
        addr: "/palette".to_string(),
        args: osc_args,
    };
    Ok(Arc::new(encoder::encode(&rosc::OscPacket::Message(
        osc_msg,
    ))?))
}

pub enum ControlMessage {
    Add(SubscriberConfig),
    Remove(SubscriberId),
}

pub enum StateChange {
    Added(Subscriber),
    Removed(SubscriberId),
}

/// A unique ID assigned to each subscriber when it is added.
/// Clients can refer to subscribers by this ID.
#[derive(Debug, Copy, Clone, PartialEq, Display)]
pub struct SubscriberId(u64);

#[derive(Debug, Clone)]
pub struct Subscriber {
    id: SubscriberId,
    cfg: SubscriberConfig,
}

impl Subscriber {
    pub fn id(&self) -> SubscriberId {
        self.id
    }
}

#[derive(Debug, Clone)]
pub enum SubscriberConfig {
    Osc(SocketAddr),
}