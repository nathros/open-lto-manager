use std::{collections::HashMap, sync::LazyLock};

use crate::shared::models::database::label_preset::model_label_preset::PDFPageType;

#[derive(Clone)]
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
        let mut result = HashMap::from([
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
                PDFPageType::Avery3420, // https://labelsmerge.com/assets/labels/pdf/avery-3420-template-google-docs.pdf
                PDFPageConfig {
                    label_width: 71.0, // Label: 70 x 16.9mm
                    label_height: 17.9,
                    count_label: 51,
                    count_column: 3,
                    start_x: -0.5,
                    start_y: 4.5,
                    increment_x: 70.0,
                    increment_y: 16.9,
                    ..PDFPageConfig::base_a4()
                },
            ),
            (
                PDFPageType::Avery5366, // https://www.avery.com/templates/5366
                PDFPageConfig {
                    label_width: 80.5, // Label: 3-7/16" x 2/3"
                    label_height: 17.93,
                    count_label: 30,
                    count_column: 2,
                    start_x: 16.8895,
                    start_y: 12.2,
                    increment_x: 101.6,
                    increment_y: 16.91,
                    ..PDFPageConfig::base_letter()
                },
            ),
            (
                PDFPageType::Avery6571_6577, // https://www.avery.com/templates/6571, https://www.avery.com/templates/6577
                PDFPageConfig {
                    label_width: 77.2, // Label: 3" x 5/8"
                    label_height: 16.875,
                    count_label: 32,
                    count_column: 2,
                    start_x: 20.93,
                    start_y: 12.2,
                    increment_x: 96.855,
                    increment_y: 15.875,
                    ..PDFPageConfig::base_letter()
                },
            ),
            (
                PDFPageType::AveryL7162, // https://www.avery.co.uk/template-l7162
                PDFPageConfig {
                    label_width: 80.5, // Label: 99.06mm x 33.87mm
                    label_height: 18.0,
                    count_label: 32,
                    count_column: 2,
                    start_x: 13.954,
                    start_y: 12.5,
                    increment_x: 101.6,
                    increment_y: 16.935,
                    ..PDFPageConfig::base_a4()
                },
            ),
            (
                PDFPageType::NetCllc749303_12301, // https://label.tec-it.com/en/Group/LTO/LTO%20NetC
                PDFPageConfig {
                    label_width: 79.74, // Label: 3.1" x 0.66"
                    label_height: 17.764,
                    count_label: 20,
                    count_column: 2,
                    start_x: 19.250,
                    start_y: 20.9,
                    increment_x: 98.425,
                    increment_y: 24.549,
                    ..PDFPageConfig::base_letter()
                },
            ),
        ]);
        if let Some(find) = result.get(&PDFPageType::Avery3420)
            && let copy = find.clone()
        {
            result.insert(PDFPageType::Herma4459, copy.clone());
            result.insert(PDFPageType::Herma4611, copy);
        }
        if let Some(find) = result.get(&PDFPageType::Avery6571_6577)
            && let copy = find.clone()
        {
            result.insert(PDFPageType::WorldLabelWl173, copy.clone());
            result.insert(PDFPageType::Avery94214, copy.clone());
            result.insert(PDFPageType::OnlineLabelsOL173, copy);
        }
        result
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
