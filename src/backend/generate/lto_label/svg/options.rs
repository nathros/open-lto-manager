use std::{
    cmp::min,
    fmt::{Display, Formatter, Result},
};

use crate::shared::{
    r#const::Const,
    models::database::label_preset::model_label_preset::{LabelOptions, LabelTextOrientation},
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

    pub fn combine_label(barcode: &String, designation: &String) -> String {
        format!(
            "{:w1$}{:w2$}",
            barcode,
            designation,
            w1 = Const::CODE_39_LTO_USABLE_LEN,
            w2 = Const::CODE_39_LTO_DESIGNATION_LEN
        )
    }

    pub fn generate_barcode(&self, index: usize) -> String {
        let size = self.prefix.len() + self.postfix.len();
        let mut tmp = format!(
            "{}{:0>width$}{}",
            self.prefix,
            self.start_index + index,
            self.postfix,
            width = Const::CODE_39_LTO_USABLE_LEN - min(Const::CODE_39_LTO_USABLE_LEN, size),
        );
        while tmp.len() > Const::CODE_39_LTO_USABLE_LEN {
            tmp.pop();
        }
        tmp.push_str(self.designation.as_str());
        tmp
    }

    pub fn generate_barcodes(&self) -> Vec<String> {
        let mut labels = vec![];
        (0..self.quantity).for_each(|i| {
            labels.push(self.generate_barcode(i));
        });
        labels
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

#[cfg(test)]
mod tests {
    use crate::shared::models::database::label_preset::model_label_preset::LabelOptions;

    #[test]
    fn barcode_padding() {
        assert_eq!(
            "        ",
            LabelOptions::combine_label(&"".to_string(), &"".to_string())
        );
        assert_eq!(
            "12      ",
            LabelOptions::combine_label(&"12".to_string(), &"".to_string())
        );
        assert_eq!(
            "    F   ",
            LabelOptions::combine_label(&"    F".to_string(), &"".to_string())
        );
        assert_eq!(
            "123456  ",
            LabelOptions::combine_label(&"123456".to_string(), &"".to_string())
        );
        assert_eq!(
            "123456A ",
            LabelOptions::combine_label(&"123456".to_string(), &"A".to_string())
        );
        assert_eq!(
            "123456AB",
            LabelOptions::combine_label(&"123456".to_string(), &"AB".to_string())
        );
        assert_eq!(
            "      A ",
            LabelOptions::combine_label(&"".to_string(), &"A".to_string())
        );
    }

    #[test]
    fn barcode_generate_ok() {
        let start_index = 3;
        let quantity: usize = 5;
        let prefix = "A".to_string();
        let postfix = "BB".to_string();
        let designation = "XY".to_string();

        let options = LabelOptions {
            start_index,
            quantity,
            designation,
            prefix: prefix.clone(),
            postfix: postfix.clone(),
            ..Default::default()
        };

        let test_data_ok = ["A003BBXY", "A004BBXY", "A005BBXY", "A006BBXY", "A007BBXY"];
        assert_eq!(test_data_ok.len(), quantity);

        // Test generate single
        (0..quantity).for_each(|i| {
            let label = options.generate_barcode(i);
            assert_eq!(test_data_ok[i], label);
        });

        // Test generate all
        let barcodes = options.generate_barcodes();
        for (i, barcode) in barcodes.iter().enumerate() {
            assert_eq!(test_data_ok[i], barcode);
        }
    }

    #[test]
    fn barcode_generate_errors() {
        assert_eq!(
            LabelOptions {
                start_index: 5,
                quantity: 1,
                designation: "XX".to_string(),
                prefix: "ABCDEF".to_string(), // Prefix + postfix = overflow
                postfix: "XYZ".to_string(),
                ..Default::default()
            }
            .generate_barcode(0),
            "ABCDEFXX".to_string()
        );
        assert_eq!(
            LabelOptions {
                start_index: 5,
                quantity: 1,
                designation: "XX".to_string(),
                prefix: "".to_string(), // Prefix + postfix = full
                postfix: "ABCDEF".to_string(),
                ..Default::default()
            }
            .generate_barcode(0),
            "5ABCDEXX".to_string()
        );
        assert_eq!(
            LabelOptions {
                start_index: 5,
                quantity: 1,
                designation: "XX".to_string(),
                prefix: "A".to_string(), // Prefix + postfix = full
                postfix: "ABCDEF".to_string(),
                ..Default::default()
            }
            .generate_barcode(0),
            "A5ABCDXX".to_string()
        );
    }
}
