use shared::{Color, PaletteControlMessage, PaletteStateChange};

pub struct Palette(Vec<Color>);

impl Palette {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn colors(&self) -> &[Color] {
        &self.0
    }

    pub fn control(&mut self, msg: PaletteControlMessage) -> PaletteStateChange {
        match msg {
            PaletteControlMessage::Set(colors) => {
                self.0.clear();
                self.0.extend_from_slice(&colors);
                PaletteStateChange::Set(colors)
            }
        }
    }
}
