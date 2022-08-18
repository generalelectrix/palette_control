use std::net::SocketAddr;

use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn as_u8(&self) -> (u8, u8, u8) {
        (
            unipolar_float_to_u8(self.red),
            unipolar_float_to_u8(self.green),
            unipolar_float_to_u8(self.blue),
        )
    }
}

/// Convert a unit float to an 8-bit integer.
/// Uses rounding instead of floor to ensure we divide up the unit range into
/// bins of equal size.
fn unipolar_float_to_u8(f: f32) -> u8 {
    (f * 255.).round() as u8
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlMessage {
    Refresh,
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display, Serialize, Deserialize)]
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
