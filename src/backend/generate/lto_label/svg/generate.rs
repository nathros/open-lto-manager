use std::collections::BTreeSet;

use tracing::error;

use crate::{
    backend::generate::lto_label::svg::{
        code_39::Code39Segment, position::PDFLabelPosition, svg_page::SvgPage,
    },
    shared::{
        r#const::Const,
        error::ErrorStr,
        models::database::label_preset::model_label_preset::{
            LabelCheckDigit, LabelOptions, LabelTextDirection, LabelTextOrientation,
        },
    },
};

use super::{code_39::CODE_39_BARCODE_SEGMENTS, svg_label::SvgLabel};

const TEXT_BOX_ID: &str = "t";
const BACKGROUND_ID: &str = "b";

pub fn generate_lto_label_svg_single(
    mut barcode: String,
    options: &LabelOptions,
) -> Result<String, ErrorStr> {
    let label = format!(
        "{}{}",
        barcode,
        (0..(Const::CODE_39_LTO_MAIN_LEN - barcode.len()))
            .map(|_| " ")
            .collect::<String>() // Pad empty with space
    );

    if label.len() != Const::CODE_39_LTO_MAIN_LEN {
        return Err("Barcode not correct length".to_string());
    }

    barcode = format!(
        "*{}{}*",
        label,
        match options.check_digit {
            LabelCheckDigit::None => "".to_string(),
            LabelCheckDigit::Modulo10 => Code39Segment::create_check_digit_mod_10(label.as_str()),
            LabelCheckDigit::Modulo43 => Code39Segment::create_check_digit_mod_43(label.as_str()),
        }
    );

    let page_config = options.page.get_config();
    let mut svg = SvgLabel::new(options, page_config);
    //svg.append_line(0, format!("<!--{}-->", barcode).as_str());

    let mut unique_characters: BTreeSet<char> = BTreeSet::new(); // Maintain insertion order
    for char in barcode.chars() {
        if !Const::BARCODE_VALID_CHARS.contains(char) {
            return Err(format!("Invalid character: {}", char));
        }
        unique_characters.insert(char);
    }

    let scale = if options.check_digit == LabelCheckDigit::None {
        options.barcode_scale
    } else {
        // Check digit has added extra character, scale barcode to add extra segment
        options.barcode_scale
            * (Const::CODE_39_BARCODE_LEN as f64 / (Const::CODE_39_BARCODE_LEN + 1) as f64)
    };

    let segment_height_str = format!("{:.1}", (page_config.label_height as f64 - 6.8) / scale);
    svg.append_group(
        1,
        "defs",
        options,
        Box::new(move |tab_index: i32, svg: &mut SvgLabel, options: &LabelOptions| {
            for index in unique_characters.iter() {
                if let Some(segment_gen) = CODE_39_BARCODE_SEGMENTS.get(index) {
                    svg.append_line(
                        tab_index,
                        format!(
                            "<svg id=\"{}\" width=\"6.588mm\" height=\"{}mm\">",
                            *index as u8, segment_height_str
                        )
                        .as_str(),
                    );
                    for segment in segment_gen.create_segment(segment_height_str.as_str()) {
                        svg.append_line(tab_index + 1, segment.as_str());
                    }
                    svg.append_line(tab_index, "</svg>");
                }
            }
            svg.append_line(
                tab_index,
                format!("<rect id=\"{}\" width=\"{}\" height=\"{}\" x=\"0\" y=\"0\" rx=\"{}\" ry=\"{}\" stroke=\"#000\" stroke-width=\"{}\" />",
                TEXT_BOX_ID, options.text_box_width, options.text_box_height, options.text_box_radius, options.text_box_radius, options.text_box_stroke).as_str()
            );
            svg.append_line(tab_index, format!("<rect id=\"{}\" width=\"{}\" height=\"{}\" x=\"1\" y=\"1\" rx=\"{}\" ry=\"{}\" />",
            BACKGROUND_ID, page_config.label_width - 2.0, page_config.label_height - 2.0, options.radius_outer, options.radius_outer).as_str());
        }),
    );

    if let Some(col) = options.background_colour.as_ref() {
        svg.append_line(
            1, // Background colour
            format!("<use href=\"#{}\" fill=\"{}\"/>", BACKGROUND_ID, col).as_str(),
        );
    }

    let shift_x = 6.588 * scale;
    let total_barcode_width = shift_x * barcode.len() as f64; // Extra space needed per segment

    let mut translate_x = page_config.label_width as f64 - 2_f64; // Total usable space
    translate_x -= total_barcode_width; // Calculate free space
    translate_x = (translate_x / 2_f64) + 1_f64; // Divide by 2 to centre + 1

    // Add barcode vertical lines
    for char in barcode.chars() {
        svg.append_line(
            1,
            format!(
                "<use href=\"#{}\" transform=\"translate({:.3} 5.8) scale({})\"/>",
                char as i32, translate_x, scale
            )
            .as_str(),
        );
        translate_x += shift_x;
    }

    translate_x = page_config.label_width as f64 - 2_f64; // Total usable space
    translate_x -= options.text_box_width * 7_f64; // Calculate free space, for 7 text boxes
    translate_x = (translate_x / 2_f64) + 1_f64; // Divide by 2 to centre + 1

    let text_rotation = format!("{}", options.text_orientation);
    let text_x = format!("{}", options.text_box_width / 2_f64);
    let y_offset = match options.text_orientation {
        LabelTextOrientation::Normal => 0.5_f64,
        _ => 0.0_f64,
    };
    let text_y = format!("{}", (options.text_box_height / 2_f64) + y_offset);

    // Add label text box
    let barcode_text_actions: [&str; 7] = match options.text_direction {
        LabelTextDirection::Normal => [
            &label[0..1],
            &label[1..2],
            &label[2..3],
            &label[3..4],
            &label[4..5],
            &label[5..6],
            &label[6..8], // Designation at end
        ],
        LabelTextDirection::Reversed => [
            &label[6..8], // Designation at start
            &label[5..6],
            &label[4..5],
            &label[3..4],
            &label[2..3],
            &label[1..2],
            &label[0..1],
        ],
    };
    let font_size_major = options.text_box_font_size.to_string();
    let font_size_minor = (options.text_box_font_size * 0.8).to_string();
    for str in barcode_text_actions {
        barcode_text(
            &mut svg,
            options,
            translate_x + options.text_box_x_offset,
            1.0 + options.text_box_y_offset,
            str,
            &text_rotation,
            &text_x,
            &text_y,
            &font_size_major,
            &font_size_minor,
        );
        translate_x += options.text_box_width;
    }

    svg.append_line(
        1, // Background outline
        format!(
            "<use href=\"#{}\" fill=\"none\" stroke=\"#000\" stroke-width=\"{}\"/>",
            BACKGROUND_ID, options.stroke_outer
        )
        .as_str(),
    );

    let result = svg.result();
    //let _ = std::fs::write("test.svg", &result);

    Ok(result)
}

fn generate_lto_label_svg_multiple(options: &LabelOptions) -> Vec<String> {
    let barcodes: Vec<String> = options.generate_barcodes();

    let mut svg: Vec<String> = vec![];
    for barcode in barcodes {
        match generate_lto_label_svg_single(barcode, options) {
            Ok(s) => {
                svg.push(s);
            }
            Err(e) => {
                error!("Failed to generate label {}", e)
            }
        }
    }
    svg
}

pub fn generate_lto_label_svg_pages(options: &LabelOptions) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let page_config = options.page.get_config();

    let svg_labels_str = generate_lto_label_svg_multiple(options);

    let mut svg_page = SvgPage::new(page_config);
    let mut position = PDFLabelPosition::new(page_config);

    for (index, label) in svg_labels_str.iter().enumerate() {
        let gap_multiplier_x = (index % page_config.count_column) as f32;
        let gap_multiplier_y = (index / page_config.count_column) as f32;
        svg_page.add_label(
            label.as_str(),
            position.x + options.page_x_offset + (gap_multiplier_x * options.page_inner_x_gap),
            position.y + options.page_y_offset + (gap_multiplier_y * options.page_inner_y_gap),
        );

        // Advance position and create new page if returns true, do not create new page for last label
        if position.next() && index + 1 != svg_labels_str.len() {
            result.push(svg_page.result());
            svg_page = SvgPage::new(page_config); // Move to next page
        }
    }
    result.push(svg_page.result());

    result
}

#[allow(clippy::too_many_arguments)]
fn barcode_text(
    svg: &mut SvgLabel,
    options: &LabelOptions,
    translate_x: f64,
    translate_y: f64,
    text: &str,
    rotate: &str,
    text_x: &str,
    text_y: &str,
    font_size_major: &str,
    font_size_minor: &str,
) {
    let (font_size, colour) = if text.len() > 1 {
        (font_size_minor, options.get_character_colour('*')) // Is tape designation
    } else {
        (
            // Is single character
            font_size_major,
            options.get_character_colour(text.chars().next().unwrap_or('*')),
        )
    };

    // svg.append_line(0, format!("<!--{}-->", text).as_str());
    svg.append_line(
        1,
        format!(
            "<g transform=\"translate({:.3} {})\">",
            translate_x, translate_y
        )
        .as_str(),
    );
    svg.append_line(
        2,
        format!("<use href=\"#{}\" fill=\"{}\" />", TEXT_BOX_ID, colour).as_str(),
    );
    svg.append_line(
        2,
        format!(
            "<text x=\"{}\" y=\"{}\" dominant-baseline=\"middle\" text-anchor=\"middle\" transform=\"rotate({} {} {})\" font-size=\"{}\">{}</text>",
            text_x, text_y, rotate, text_x, text_y, font_size, text
        )
        .as_str(),
    );
    svg.append_line(1, "</g>");
}

#[cfg(test)]
pub mod tests {
    use crate::{
        backend::generate::lto_label::svg::generate::generate_lto_label_svg_single,
        shared::models::database::label_preset::model_label_preset::{
            LabelCheckDigit, LabelOptions, LabelTheme,
        },
    };

    pub fn test_file(test: &str) -> String {
        std::fs::read_to_string(format!("test/labels/{}", test)).unwrap()
    }

    #[test]
    fn single_svg_generate() {
        const DIR: &str = "single/";
        let test_data = [
            (
                "default_preview.svg",
                "ABCDEFXX",
                LabelOptions {
                    ..LabelOptions::default_preview()
                },
            ),
            (
                "default_preview-no_viewbox.svg",
                "ABCDEFXX",
                LabelOptions {
                    include_view_box: false,
                    ..LabelOptions::default_preview()
                },
            ),
            (
                "default_preview-theme_greyscale.svg",
                "ABCDEFXX",
                LabelOptions {
                    theme: LabelTheme::Greyscale,
                    ..LabelOptions::default_preview()
                },
            ),
            (
                "default_preview-check_digit_mod10.svg",
                "ABCDEFXX",
                LabelOptions {
                    check_digit: LabelCheckDigit::Modulo10,
                    ..LabelOptions::default_preview()
                },
            ),
            (
                "default_preview-check_digit_mod43.svg",
                "ABCDEFXX",
                LabelOptions {
                    check_digit: LabelCheckDigit::Modulo43,
                    ..LabelOptions::default_preview()
                },
            ),
            (
                "default_preview-text_box_composite.svg",
                "ABCDEFXX",
                LabelOptions {
                    text_box_width: 7.0,
                    text_box_height: 7.0,
                    text_box_stroke: 2.0,
                    text_box_radius: 4.0,
                    text_box_x_offset: -6.0,
                    text_box_y_offset: 4.0,
                    text_box_font_size: 3.0,
                    ..LabelOptions::default_preview()
                },
            ),
        ];

        let inspector = test_file(format!("{}inspect.preview.html", DIR).as_str());

        test_data
            .iter()
            .for_each(|(test_file_name, barcode, options)| {
                let svg_str = generate_lto_label_svg_single(barcode.to_string(), options).unwrap();

                //std::fs::write(test_file_name, &svg_str);

                assert_eq!(
                    test_file(format!("{}{}", DIR, test_file_name).as_str()),
                    svg_str
                );

                assert!(
                    inspector.find(&format!("\"{}\"", test_file_name)).is_some(),
                    "Did not find reference to test file in single/inspect.preview.html"
                );
            });
    }
}
