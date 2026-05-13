use super::theme::{CODE_39_BARCODE_THEMES, LabelTheme};

pub struct LabelOptions {
    theme_colour: LabelTheme,
}

impl Default for LabelOptions {
    fn default() -> Self {
        Self {
            theme_colour: LabelTheme::Standard,
        }
    }
}

impl LabelOptions {
    pub fn get_character_colour(&self, char: char) -> &'static str {
        if let Some(theme) = CODE_39_BARCODE_THEMES.get(&self.theme_colour)
            && let Some(colour) = theme.get(&char)
        {
            return colour;
        }

        "transparent"
    }
}
