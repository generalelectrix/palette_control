use std::{net::SocketAddr, sync::Arc};

use log::error;
use rosc::{encoder, OscMessage, OscType};

use crate::{color::Color, osc::OscSender};

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

    pub fn control<E: EmitStateChange>(&mut self, msg: ControlMessage, emitter: &mut E) {
        match msg {
            ControlMessage::Add(cfg) => {
                let id = self.next_id;
                self.next_id.0 += 1;
                let sub = Subscriber { id, cfg };
                self.subs.push(sub.clone());
                emitter.emit_subscriber_state_change(StateChange::Added(sub));
            }
            ControlMessage::Remove(id) => {
                self.subs.retain(|sub| sub.id != id);
                emitter.emit_subscriber_state_change(StateChange::Removed(id));
            }
        }
    }

    /// Send the provided palette to all subscribers.
    /// Logs errors.
    pub fn send_palette(&self, colors: &[Color], osc_sender: &OscSender) {
        use SubscriberConfig::*;
        // Produce encoded messages only once.
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
        let osc_encoded = Arc::new(match encoder::encode(&rosc::OscPacket::Message(osc_msg)) {
            Ok(msg) => msg,
            Err(e) => {
                error!("Unable to encode OSC message: {}.", e);
                return;
            }
        });
        for sub in self.subs.iter() {
            match sub.cfg {
                Osc(addr) => {
                    osc_sender.send(addr, osc_encoded.clone());
                }
            }
        }
    }
}

pub enum ControlMessage {
    Add(SubscriberConfig),
    Remove(SubscriberId),
}

pub enum StateChange {
    Added(Subscriber),
    Removed(SubscriberId),
}

pub trait EmitStateChange {
    fn emit_subscriber_state_change(&mut self, sc: StateChange);
}

/// A unique ID assigned to each subscriber when it is added.
/// Clients can refer to subscribers by this ID.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SubscriberId(u64);

#[derive(Debug, Clone)]
pub struct Subscriber {
    id: SubscriberId,
    cfg: SubscriberConfig,
}

#[derive(Debug, Clone)]
pub enum SubscriberConfig {
    Osc(SocketAddr),
}
