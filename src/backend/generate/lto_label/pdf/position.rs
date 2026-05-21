use super::page::PDFPageConfig;

pub struct PDFLabelPosition<'life> {
    pub x: f32,
    pub y: f32,

    label_count_page: usize, // Number of labels currently on this page
    current_column: usize,

    config: &'life PDFPageConfig,
}

impl PDFLabelPosition<'_> {
    pub fn new(config: &'_ PDFPageConfig) -> PDFLabelPosition<'_> {
        PDFLabelPosition {
            x: config.start_x,
            y: config.start_y,
            label_count_page: 0,
            current_column: 0,
            config,
        }
    }

    // Updates state and returns true if new page is needed
    pub fn next(&mut self) -> bool {
        self.label_count_page += 1;
        if self.label_count_page >= self.config.count_label {
            // Move to new page and reset
            self.x = self.config.start_x;
            self.y = self.config.start_y;

            self.label_count_page = 0;
            self.current_column = 0;
            return true;
        }

        self.current_column += 1;
        if self.current_column >= self.config.count_column {
            // Move to next row
            self.current_column = 0;
            self.x = self.config.start_x;
            self.y += self.config.increment_y;
        } else {
            self.x += self.config.increment_x;
        }
        false
    }
}

#[cfg(test)]
mod tests {

    use std::collections::{HashMap, HashSet};

    use crate::backend::generate::lto_label::pdf::page::PDFPageType;

    use super::PDFLabelPosition;

    #[test]
    fn label_positions() {
        let page_types = HashSet::from([PDFPageType::A4, PDFPageType::Letter]);

        for page_type in page_types {
            let page_config = page_type.get_config();
            let mut position = PDFLabelPosition::new(page_config);

            type XY = (f32, f32);
            let mut history_all: HashMap<i32, Vec<XY>> = HashMap::new();

            // Create 3 pages
            for pg in 0..3 {
                let mut history_current: Vec<XY> = vec![];

                // Check reset for new page
                assert_eq!(position.current_column, 0);
                assert_eq!(position.label_count_page, 0);
                assert_eq!(position.x, page_config.start_x);
                assert_eq!(position.y, page_config.start_y);

                // Loop through all labels for 1 page except last
                for i in 0..(page_config.count_label - 1) {
                    //println!(
                    //    "{:?} {}[{:3}] x={:5}, y={:5}",
                    //    page_type, pg, position.label_count_page, position.x, position.y
                    //);
                    let row_index = position.label_count_page / page_config.count_column;

                    // X position correct
                    assert_eq!(
                        position.x,
                        page_config.start_x
                            + (page_config.increment_x * (i % page_config.count_column) as f32)
                    );

                    // Y position correct
                    assert_eq!(
                        position.y,
                        page_config.start_y + (page_config.increment_y * row_index as f32)
                    );

                    history_current.push((position.x, position.y)); // Add current X and Y positions
                    assert!(!position.next()); // No new page
                }
                assert_eq!(position.label_count_page, page_config.count_label - 1); // Now at penultimate label
                assert!(position.next(), "Expected new page"); // True for new page

                // Check reset after new page
                assert_eq!(position.current_column, 0);
                assert_eq!(position.label_count_page, 0);
                assert_eq!(position.x, page_config.start_x);
                assert_eq!(position.y, page_config.start_y);
                history_all.insert(pg, history_current); // Add all X and Y positions of this page
            }

            // Make sure all pages are the same
            let first: Vec<XY> = history_all.values().next().unwrap().clone();
            for hist in history_all {
                assert_eq!(first, hist.1);
            }
        }
    }
}
