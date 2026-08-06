use std::{collections::HashMap, sync::LazyLock};

use crate::shared::models::database::label_preset::model_label_preset::PDFPageType;

pub struct PDFPageConfig {
    pub width_pt: f32,  // As unit
    pub height_pt: f32, // As unit
    pub width_mm: f32,  // As unit
    pub height_mm: f32, // As unit

    pub count_label: usize,  // Number of labels for a page
    pub count_column: usize, // Number of columns for a page

    pub start_x: f32, // Start X position for row in mm
    pub start_y: f32, // Start Y position for page in mm

    pub increment_x: f32, // Add this to start_x to move to next column in mm
    pub increment_y: f32, // Add this to start_y to move to next row in mm
}

impl PDFPageType {
    pub fn get_config(&self) -> &PDFPageConfig {
        match PDF_PAGE_DIMENSIONS.get(self) {
            Some(p) => p,
            None => match PDF_PAGE_DIMENSIONS.get(&PDFPageType::A4) {
                Some(p) => p,
                None => unreachable!("A4 should exist for PDF_PAGE_DIMENSIONS"),
            },
        }
    }
}

pub static PDF_PAGE_DIMENSIONS: LazyLock<HashMap<PDFPageType, PDFPageConfig>> =
    LazyLock::new(|| {
        HashMap::from([
            (
                PDFPageType::A4,
                PDFPageConfig {
                    width_pt: 595.0,
                    height_pt: 842.0,
                    width_mm: 210.0,
                    height_mm: 297.0,
                    count_label: 32,
                    count_column: 2,
                    start_x: 25.0,
                    start_y: 3.4,
                    increment_x: 80.5,
                    increment_y: 18.5,
                },
            ),
            (
                PDFPageType::Letter,
                PDFPageConfig {
                    width_pt: 612.0,
                    height_pt: 792.0,
                    width_mm: 215.9,
                    height_mm: 279.4,
                    count_label: 30,
                    count_column: 2,
                    start_x: 25.0,
                    start_y: 3.4,
                    increment_x: 80.5,
                    increment_y: 18.5,
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
