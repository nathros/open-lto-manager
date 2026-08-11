use crate::backend::generate::lto_label::pdf::page::PDFPageConfig;

pub struct SvgPage {
    buffer: String,
}

impl SvgPage {
    pub fn new(page: &PDFPageConfig) -> SvgPage {
        SvgPage {
            buffer: format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}mm\" height=\"{}mm\" viewBox=\"0 0 {} {}\">\n",
                page.width_mm, page.height_mm, page.width_mm, page.height_mm,
            ),
        }
    }

    pub fn result(&mut self) -> String {
        self.buffer.push_str("</svg>");
        self.buffer.to_owned()
    }

    pub fn add_label(&mut self, label: &str, x: f32, y: f32) {
        self.buffer
            .push_str(format!("<g transform=\"translate({}, {})\">\n", x, y).as_str());
        self.buffer.push_str(label);
        self.buffer.push_str("\n</g>\n");
    }
}

#[cfg(test)]
mod tests {
    use enum_iterator::all;

    use crate::{
        backend::generate::lto_label::svg::{
            generate::{
                generate_lto_label_svg_pages, generate_lto_label_svg_single, tests::test_file,
            },
            svg_page::SvgPage,
        },
        shared::models::database::label_preset::model_label_preset::{LabelOptions, PDFPageType},
    };

    #[test]
    fn svg_empty_page_generate() {
        for page_type in all::<PDFPageType>() {
            let page_config = page_type.get_config();
            let svg_page_str = SvgPage::new(page_config).result();
            //std::fs::write(format!("{:?}.svg", page_type), svg_page.result());
            assert_eq!(
                test_file(format!("page/empty-{:?}.svg", page_type).as_str()),
                svg_page_str
            );
        }
    }

    #[test]
    fn svg_page_generate() {
        let label_1 =
            generate_lto_label_svg_single("TEST00..".to_string(), LabelOptions::default()).unwrap();
        let label_2 =
            generate_lto_label_svg_single("SECOND++".to_string(), LabelOptions::default()).unwrap();
        let page_config = PDFPageType::A4.get_config();

        let mut page = SvgPage::new(page_config);
        page.add_label(label_1.as_str(), 32_f32, 64_f32);
        page.add_label(label_2.as_str(), 64_f32, 128_f32);

        assert_eq!(test_file("page/test-2-labels.svg"), page.result());
    }

    #[test]
    fn svg_full_page_generate() {
        const DIR: &str = "page/align/";

        let test_data = [
            LabelOptions {
                designation: "ZZ".to_string(),
                page: PDFPageType::Avery3420,
                quantity: 51, // Max per page as above
                ..LabelOptions::default()
            },
            LabelOptions {
                designation: "ZZ".to_string(),
                page: PDFPageType::Avery5366,
                quantity: 30, // Max per page as above
                ..LabelOptions::default()
            },
            LabelOptions {
                designation: "ZZ".to_string(),
                page: PDFPageType::Avery6571_6577,
                quantity: 32, // Max per page as above
                ..LabelOptions::default()
            },
            LabelOptions {
                designation: "ZZ".to_string(),
                page: PDFPageType::AveryL7162,
                quantity: 32, // Max per page as above
                ..LabelOptions::default()
            },
        ];

        for mut options in test_data {
            options.switch_page(options.page);
            options.stroke_outer = LabelOptions::default().stroke_outer;

            let page = generate_lto_label_svg_pages(&options); // Render page

            assert!(page.len() == 1); // Expect only 1 page for this test

            //std::fs::write("test.svg", page.first().unwrap());
            assert_eq!(
                test_file(format!("{}{:?}_render.svg", DIR, options.page).as_str()),
                *page.first().unwrap()
            );
        }
    }
}
