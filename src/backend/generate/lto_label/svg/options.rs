use std::fmt::{Display, Formatter, Result};

use crate::shared::models::database::label_preset::model_label_preset::{
    LabelOptions, LabelTextOrientation,
};

use super::theme::CODE_39_BARCODE_THEMES;

impl LabelOptions {
    pub fn get_character_colour(&self, char: char) -> &'static str {
        if let Some(theme) = CODE_39_BARCODE_THEMES.get(&self.theme)
            && let Some(colour) = theme.get(&char)
        {
            return colour;
        }

        "transparent"
    }
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
