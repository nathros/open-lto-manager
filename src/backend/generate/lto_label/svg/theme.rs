use std::{collections::HashMap, sync::LazyLock};

use crate::shared::models::database::label_preset::model_label_preset::LabelTheme;

pub static CODE_39_BARCODE_THEMES: LazyLock<HashMap<LabelTheme, HashMap<char, &str>>> =
    LazyLock::new(|| {
        HashMap::from([
            (
                LabelTheme::Standard,
                HashMap::from([
                    ('A', "#DD001B"),
                    ('B', "#EBE541"),
                    ('C', "#A4CB3E"),
                    ('D', "#39B4E4"),
                    ('E', "#B4B6B5"),
                    ('F', "#E7751E"),
                    ('G', "#E96195"),
                    ('H', "#33AA43"),
                    ('I', "#F3B732"),
                    ('J', "#745D9C"),
                    ('K', "#DD001B"),
                    ('L', "#EBE541"),
                    ('M', "#A4CB3E"),
                    ('N', "#39B4E4"),
                    ('O', "#B4B6B5"),
                    ('P', "#E7751E"),
                    ('Q', "#E96195"),
                    ('R', "#33AA43"),
                    ('S', "#F3B732"),
                    ('T', "#745D9C"),
                    ('U', "#DD001B"),
                    ('V', "#EBE541"),
                    ('W', "#A4CB3E"),
                    ('X', "#39B4E4"),
                    ('Y', "#B4B6B5"),
                    ('Z', "#E7751E"),
                    ('0', "#DD001B"),
                    ('1', "#EBE541"),
                    ('2', "#A4CB3E"),
                    ('3', "#39B4E4"),
                    ('4', "#B4B6B5"),
                    ('5', "#E7751E"),
                    ('6', "#E96195"),
                    ('7', "#33AA43"),
                    ('8', "#F3B732"),
                    ('9', "#745D9C"),
                    (' ', "#FFF"),
                    ('-', "#FFF"),
                    ('$', "#FFF"),
                    ('%', "#FFF"),
                    ('.', "#FFF"),
                    ('/', "#FFF"),
                    ('+', "#FFF"),
                    ('*', "#FFF"),
                ]),
            ),
            (
                LabelTheme::Warm,
                HashMap::from([
                    ('A', "#FF8881"),
                    ('B', "#FFFD93"),
                    ('C', "#B4FD4C"),
                    ('D', "#84DFFD"),
                    ('E', "#C4C4C4"),
                    ('F', "#FF864D"),
                    ('G', "#FFB7DD"),
                    ('H', "#8DE05F"),
                    ('I', "#FFB459"),
                    ('J', "#B2B6FA"),
                    ('K', "#FF8881"),
                    ('L', "#FFFD93"),
                    ('M', "#B4FD4C"),
                    ('N', "#84DFFD"),
                    ('O', "#C4C4C4"),
                    ('P', "#FF864D"),
                    ('Q', "#FFB7DD"),
                    ('R', "#8DE05F"),
                    ('S', "#FFB459"),
                    ('T', "#B2B6FA"),
                    ('U', "#FF8881"),
                    ('V', "#FFFD93"),
                    ('W', "#B4FD4C"),
                    ('X', "#84DFFD"),
                    ('Y', "#C4C4C4"),
                    ('Z', "#FF864D"),
                    ('0', "#FF8881"),
                    ('1', "#FFFD93"),
                    ('2', "#B4FD4C"),
                    ('3', "#84DFFD"),
                    ('4', "#C4C4C4"),
                    ('5', "#FF864D"),
                    ('6', "#FFB7DD"),
                    ('7', "#8DE05F"),
                    ('8', "#FFB459"),
                    ('9', "#B2B6FA"),
                    (' ', "#FFF"),
                    ('-', "#FFF"),
                    ('$', "#FFF"),
                    ('%', "#FFF"),
                    ('.', "#FFF"),
                    ('/', "#FFF"),
                    ('+', "#FFF"),
                    ('*', "#FFF"),
                ]),
            ),
            (
                LabelTheme::Greyscale,
                HashMap::from([
                    ('A', "#707070"),
                    ('B', "#E1E1E1"),
                    ('C', "#BEBEBE"),
                    ('D', "#A6A6A6"),
                    ('E', "#B5B5B5"),
                    ('F', "#969696"),
                    ('G', "#919191"),
                    ('H', "#959595"),
                    ('I', "#C2C2C2"),
                    ('J', "#686868"),
                    ('K', "#707070"),
                    ('L', "#E1E1E1"),
                    ('M', "#BEBEBE"),
                    ('N', "#A6A6A6"),
                    ('O', "#B5B5B5"),
                    ('P', "#969696"),
                    ('Q', "#919191"),
                    ('R', "#959595"),
                    ('S', "#C2C2C2"),
                    ('T', "#686868"),
                    ('U', "#707070"),
                    ('V', "#E1E1E1"),
                    ('W', "#BEBEBE"),
                    ('X', "#A6A6A6"),
                    ('Y', "#B5B5B5"),
                    ('Z', "#969696"),
                    ('0', "#707070"),
                    ('1', "#E1E1E1"),
                    ('2', "#BEBEBE"),
                    ('3', "#A6A6A6"),
                    ('4', "#B5B5B5"),
                    ('5', "#969696"),
                    ('6', "#919191"),
                    ('7', "#959595"),
                    ('8', "#C2C2C2"),
                    ('9', "#686868"),
                    (' ', "#FFF"),
                    ('-', "#FFF"),
                    ('$', "#FFF"),
                    ('%', "#FFF"),
                    ('.', "#FFF"),
                    ('/', "#FFF"),
                    ('+', "#FFF"),
                    ('*', "#FFF"),
                ]),
            ),
        ])
    });

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use enum_iterator::{all, cardinality};

    use crate::backend::generate::lto_label::svg::{
        code_39::BARCODE_VALID_CHARS,
        theme::{CODE_39_BARCODE_THEMES, LabelTheme},
    };

    #[test]
    fn check() {
        let themes: HashSet<LabelTheme> =
            HashSet::from_iter(all::<LabelTheme>().collect::<Vec<_>>());
        assert_eq!(
            themes.len(),
            cardinality::<LabelTheme>(),
            "Check all themes are added"
        );
        for theme in themes {
            let check_theme = CODE_39_BARCODE_THEMES.get(&theme);
            assert!(check_theme.is_some(), "Check all themes added to list");

            // Check current theme has all supported characters
            if let Some(check_theme) = check_theme {
                assert_eq!(
                    check_theme.len(),
                    BARCODE_VALID_CHARS.len(),
                    "Theme should have same length as supported characters list"
                );

                for char in BARCODE_VALID_CHARS.chars() {
                    assert!(
                        check_theme.get(&char).is_some(),
                        "Theme is missing character"
                    );
                }
            } else {
                unreachable!();
            }
        }
    }
}
