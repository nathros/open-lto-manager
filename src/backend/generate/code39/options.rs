use super::theme::{CODE_39_BARCODE_THEMES, LabelTheme};

pub struct LabelOptions {
    theme_colour: LabelTheme,
    pub stroke_outer: f64,
    pub stroke_inner: f64,
    pub radius_outer: f64,
    pub radius_inner: f64,
    pub width: f64,
    pub height: f64,
    pub barcode_scale: f64,
    pub text_box_width: f64,
    pub text_box_height: f64,
}

impl Default for LabelOptions {
    fn default() -> Self {
        Self {
            theme_colour: LabelTheme::Standard,
            stroke_outer: 0.035,
            stroke_inner: 0.035,
            radius_outer: 1.0,
            radius_inner: 0.0,
            width: 80.5,
            height: 18.5,
            barcode_scale: 1.0,
            text_box_width: 10.0_f64,
            text_box_height: 5.8_f64,
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
