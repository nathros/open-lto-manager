use std::fmt::{Display, Formatter, Result};

#[allow(dead_code)] // TODO remove later
#[derive(Clone)]
pub enum LabelFont {
    SansSerif,
    Serif,
    Monospace,
}

impl Display for LabelFont {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match *self {
            LabelFont::SansSerif => write!(formatter, "sans-serif"),
            LabelFont::Serif => write!(formatter, "serif"),
            LabelFont::Monospace => write!(formatter, "monospace"),
        }
    }
}
