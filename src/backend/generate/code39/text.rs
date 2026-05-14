use std::fmt::{Display, Formatter, Result};

#[allow(dead_code)] // TODO remove later
#[derive(PartialEq)]
pub enum LabelTextOrientation {
    Normal,
    Rotate90,
    Rotate180,
    Rotate270,
}

impl Display for LabelTextOrientation {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match *self {
            LabelTextOrientation::Normal => write!(formatter, "0"),
            LabelTextOrientation::Rotate90 => write!(formatter, "90"),
            LabelTextOrientation::Rotate180 => write!(formatter, "180"),
            LabelTextOrientation::Rotate270 => write!(formatter, "270"),
        }
    }
}

#[allow(dead_code)] // TODO remove later
#[derive(PartialEq)]
pub enum LabelTextDirection {
    Normal,
    Reversed,
}
