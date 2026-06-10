use std::collections::HashSet;

use crate::shared::{
    error::ErrorStr,
    models::database::label_preset::model_label_preset::{
        LabelOptions, LabelTextDirection, LabelTextOrientation,
    },
};

use super::{
    code_39::{BARCODE_VALID_CHARS, CODE_39_BARCODE_SEGMENTS},
    svg_label::SvgLabel,
};

const TEXT_BOX_ID: &str = "t";
const BACKGROUND_ID: &str = "b";

pub fn generate_lto_label_svg(
    mut barcode: String,
    options: LabelOptions,
) -> Result<String, ErrorStr> {
    barcode = format!(
        "*{}{}*",
        barcode,
        (0..(8 - barcode.len())).map(|_| " ").collect::<String>() // Pad empty with space
    );

    const BARCODE_LEN: usize = 10;
    if barcode.len() != BARCODE_LEN {
        return Err("Barcode not correct length".to_string());
    }

    let mut svg = SvgLabel::new(&options);
    //svg.append_line(0, format!("<!--{}-->", barcode).as_str());

    let mut unique_characters: HashSet<char> = HashSet::new();
    for char in barcode.chars() {
        if !BARCODE_VALID_CHARS.contains(char) {
            return Err(format!("Invalid character: {}", char));
        }
        unique_characters.insert(char);
    }

    let segment_height_str = format!("{}", 11.7_f64 / options.barcode_scale);
    svg.append_group(
        1,
        "defs",
        Box::new(move |tab_index: i32, svg: &mut SvgLabel| {
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
                TEXT_BOX_ID, options.text_box_width, options.text_box_height, options.radius_inner, options.radius_inner, options.stroke_inner).as_str()
            );
            svg.append_line(tab_index, format!("<rect id=\"{}\" width=\"{}\" height=\"{}\" x=\"1\" y=\"1\" rx=\"{}\" ry=\"{}\" />",
            BACKGROUND_ID, options.width - 2_f64, options.height - 2_f64, options.radius_outer, options.radius_outer).as_str());
        }),
    );

    if let Some(col) = options.background_colour.as_ref() {
        svg.append_line(
            1, // Background colour
            format!("<use href=\"#{}\" fill=\"{}\"/>", BACKGROUND_ID, col).as_str(),
        );
    }

    let shift_x = 6.588 * options.barcode_scale;
    let total_barcode_width = shift_x * BARCODE_LEN as f64; // Extra space needed per segment

    let mut translate_x = options.width - 2_f64; // Total usable space
    translate_x -= total_barcode_width; // Calculate free space
    translate_x = (translate_x / 2_f64) + 1_f64; // Divide by 2 to centre + 1

    // Add barcode vertical lines
    for char in barcode.chars() {
        svg.append_line(
            1,
            format!(
                "<use href=\"#{}\" transform=\"translate({:.3} 5.8) scale({})\"/>",
                char as i32, translate_x, options.barcode_scale
            )
            .as_str(),
        );
        translate_x += shift_x;
    }

    translate_x = options.width - 2_f64; // Total usable space
    translate_x -= options.text_box_width * 7_f64; // Calculate free space, for 7 text boxes
    translate_x = (translate_x / 2_f64) + 1_f64; // Divide by 2 to centre + 1

    let text_rotation = format!("{}", options.text_orientation);
    let text_x = format!("{}", options.text_box_width / 2_f64);
    let y_offset = match options.text_orientation {
        LabelTextOrientation::Normal => 0.5_f64,
        _ => 0.0_f64,
    };
    let text_y = format!("{}", (options.text_box_height / 2_f64) + y_offset);

    // Add barcode text box, skip first and last '*'
    let barcode_text_actions: [&str; 7] = match options.text_direction {
        LabelTextDirection::Normal => [
            &barcode[1..2],
            &barcode[2..3],
            &barcode[3..4],
            &barcode[4..5],
            &barcode[5..6],
            &barcode[6..7],
            &barcode[7..9], // Designation at end
        ],
        LabelTextDirection::Reversed => [
            &barcode[7..9], // Designation at start
            &barcode[6..7],
            &barcode[5..6],
            &barcode[4..5],
            &barcode[3..4],
            &barcode[2..3],
            &barcode[1..2],
        ],
    };
    for str in barcode_text_actions {
        barcode_text(
            &mut svg,
            &options,
            translate_x,
            str,
            text_rotation.as_str(),
            text_x.as_str(),
            text_y.as_str(),
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

fn barcode_text(
    svg: &mut SvgLabel,
    options: &LabelOptions,
    translate_x: f64,
    text: &str,
    rotate: &str,
    text_x: &str,
    text_y: &str,
) {
    let (font_size, colour) = if text.len() > 1 {
        ("4", options.get_character_colour('*')) // Is tape designation
    } else {
        (
            // Is single character
            "5",
            options.get_character_colour(text.chars().next().unwrap_or('*')),
        )
    };

    // svg.append_line(0, format!("<!--{}-->", text).as_str());
    svg.append_line(
        1,
        format!("<g transform=\"translate({:.3} 1)\">", translate_x).as_str(),
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
