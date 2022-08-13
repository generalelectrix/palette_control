use crate::{color::Color, osc::OscSender};

pub struct Palette(Vec<Color>);

impl Palette {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn control<E: EmitStateChange>(&mut self, msg: ControlMessage, emitter: &mut E) {
        match msg {
            ControlMessage::Set(colors) => {
                self.0.clear();
                self.0.extend_from_slice(&colors);
                emitter.emit_palette_state_change(StateChange::Set(colors));
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

pub trait EmitStateChange {
    fn emit_palette_state_change(&mut self, sc: StateChange);
}
