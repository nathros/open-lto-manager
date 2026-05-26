use std::{collections::HashMap, sync::LazyLock};

use enum_iterator::Sequence;

#[derive(Debug, PartialEq, Eq, Hash, Sequence)]
pub enum PDFPageType {
    A4,
    Letter,
}

pub struct PDFPageConfig {
    pub width: f32,  // As PostScript point
    pub height: f32, // As PostScript point

    pub count_label: usize,  // Number of labels for a page
    pub count_column: usize, // Number of columns for a page

    pub start_x: f32, // Start X position for row
    pub start_y: f32, // Start Y position for page

    pub increment_x: f32, // Add this to start_x to move to next column
    pub increment_y: f32, // Add this to start_y to move to next row
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
                    width: 595.0,  // 210mm
                    height: 842.0, // 297mm
                    count_label: 32,
                    count_column: 2,
                    start_x: 69.3,
                    start_y: 3.4,
                    increment_x: 228.2, // 80.5mm
                    increment_y: 52.5,  // 18.5mm
                },
            ),
            (
                PDFPageType::Letter,
                PDFPageConfig {
                    width: 612.0,  // 215.9mm
                    height: 792.0, // 279.4mm
                    count_label: 30,
                    count_column: 2,
                    start_x: 0.0,
                    start_y: 0.0,
                    increment_x: 228.2, // 80.5mm
                    increment_y: 52.5,  // 18.5mm
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
