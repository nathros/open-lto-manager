use std::{collections::HashMap, sync::LazyLock};

use crate::shared::models::database::label_preset::model_label_preset::PDFPageType;

pub struct PDFPageConfig {
    pub width_pt: f32,  // As unit
    pub height_pt: f32, // As unit
    pub width_mm: f32,  // As unit
    pub height_mm: f32, // As unit

    pub label_width: f32,  // mm
    pub label_height: f32, // mm

    pub count_label: usize,  // Number of labels for a page
    pub count_column: usize, // Number of columns for a page

    pub start_x: f32, // Start X position for row in mm
    pub start_y: f32, // Start Y position for page in mm

    pub increment_x: f32, // Add this to start_x to move to next column in mm
    pub increment_y: f32, // Add this to start_y to move to next row in mm
}

impl PDFPageType {
    pub fn get_config(&self) -> &'static PDFPageConfig {
        match PDF_PAGE_DIMENSIONS.get(self) {
            Some(p) => p,
            None => match PDF_PAGE_DIMENSIONS.get(&PDFPageType::A4) {
                Some(p) => p,
                None => unreachable!("A4 should exist for PDF_PAGE_DIMENSIONS"),
            },
        }
    }
}

impl PDFPageConfig {
    fn empty() -> PDFPageConfig {
        PDFPageConfig {
            width_pt: 0.0,
            height_pt: 0.0,
            width_mm: 0.0,
            height_mm: 0.0,
            label_width: 0.0,
            label_height: 0.0,
            count_label: 0,
            count_column: 0,
            start_x: 0.0,
            start_y: 0.0,
            increment_x: 0.0,
            increment_y: 0.0,
        }
    }

    fn base_a4() -> PDFPageConfig {
        PDFPageConfig {
            width_pt: 595.0,
            height_pt: 842.0,
            width_mm: 210.0,
            height_mm: 297.0,
            ..PDFPageConfig::empty()
        }
    }

    fn base_letter() -> PDFPageConfig {
        PDFPageConfig {
            width_pt: 612.0,
            height_pt: 792.0,
            width_mm: 215.9,
            height_mm: 279.4,
            ..PDFPageConfig::empty()
        }
    }
}

pub static PDF_PAGE_DIMENSIONS: LazyLock<HashMap<PDFPageType, PDFPageConfig>> =
    LazyLock::new(|| {
        HashMap::from([
            (
                PDFPageType::A4,
                PDFPageConfig {
                    label_width: 80.5,
                    label_height: 18.5,
                    count_label: 32,
                    count_column: 2,
                    start_x: 26.5,
                    start_y: 16.5,
                    increment_x: 78.5,
                    increment_y: 16.5,
                    ..PDFPageConfig::base_a4()
                },
            ),
            (
                PDFPageType::Letter,
                PDFPageConfig {
                    label_width: 80.5,
                    label_height: 18.5,
                    count_label: 30,
                    count_column: 2,
                    start_x: 29.45,
                    start_y: 15.95,
                    increment_x: 78.5,
                    increment_y: 16.5,
                    ..PDFPageConfig::base_letter()
                },
            ),
            (
                PDFPageType::Avery6571, // https://www.avery.com/templates/6571
                PDFPageConfig {
                    label_width: 77.2,    // 3 Inch + 1mm
                    label_height: 16.875, // 5/8 Inch + 1mm
                    count_label: 32,
                    count_column: 2,
                    start_x: 20.93,
                    start_y: 12.2,
                    increment_x: 96.855,
                    increment_y: 15.875,
                    ..PDFPageConfig::base_letter()
                },
            ),
        ])
    });

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use enum_iterator::{all, cardinality};

    use crate::backend::generate::lto_label::pdf::page::PDF_PAGE_DIMENSIONS;

    use super::PDFPageType;

    #[test]
    fn check() {
        // TODO replace with std::mem::variant_count when stable
        let options: HashSet<PDFPageType> =
            HashSet::from_iter(all::<PDFPageType>().collect::<Vec<_>>());
        assert_eq!(cardinality::<PDFPageType>(), options.len());

        assert_eq!(PDF_PAGE_DIMENSIONS.len(), options.len());

        for option in options {
            assert!(PDF_PAGE_DIMENSIONS.get(&option).is_some())
        }
    }
}
