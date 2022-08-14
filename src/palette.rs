use crate::color::Color;

pub struct Palette(Vec<Color>);

impl Palette {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn colors(&self) -> &[Color] {
        &self.0
    }

    pub fn control(&mut self, msg: ControlMessage) -> StateChange {
        match msg {
            ControlMessage::Set(colors) => {
                self.0.clear();
                self.0.extend_from_slice(&colors);
                StateChange::Set(colors)
            }
        }
    }
}

pub enum ControlMessage {
    Set(Vec<Color>),
}

pub enum StateChange {
    Set(Vec<Color>),
}
