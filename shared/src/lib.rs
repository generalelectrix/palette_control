use std::net::SocketAddr;

use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaletteControlMessage {
    Set(Vec<Color>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaletteStateChange {
    Set(Vec<Color>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriberControlMessage {
    Add(SubscriberConfig),
    Remove(SubscriberId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriberStateChange {
    Added(Subscriber),
    Removed(SubscriberId),
}

/// A unique ID assigned to each subscriber when it is added.
/// Clients can refer to subscribers by this ID.
#[derive(Debug, Copy, Clone, PartialEq, Display, Serialize, Deserialize)]
pub struct SubscriberId(u64);

impl SubscriberId {
    pub const ZERO: SubscriberId = SubscriberId(0);
    pub fn advance(&mut self) {
        self.0 += 1;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    pub id: SubscriberId,
    pub cfg: SubscriberConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriberConfig {
    Osc(SocketAddr),
}
