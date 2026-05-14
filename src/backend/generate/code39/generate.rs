use std::collections::HashSet;

use crate::shared::error::ErrorStr;

use super::{
    options::LabelOptions,
    segment::{BARCODE_VALID_CHARS, CODE_39_BARCODE_SEGMENTS},
    svg::SvgLabel,
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
            svg.append_line(tab_index, format!("<rect id=\"{}\" width=\"{}\" height=\"{}\" x=\"1\" y=\"1\" rx=\"{}\" ry=\"{}\" />\n",
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

    let text_box_middle_x = format!("{:.3}", options.text_box_width / 2_f64);
    let text_box_middle_y = format!("{:.3}", (options.text_box_height / 2_f64) + 2_f64);

    // Add barcode text box, skip first and last '*'
    for (i, char) in barcode.chars().skip(1).take(barcode.len() - 4).enumerate() {
        barcode_text(
            &mut svg,
            translate_x,
            &barcode[(i + 1)..(i + 2)],
            "5",
            options.get_character_colour(char),
            text_box_middle_x.as_str(),
            text_box_middle_y.as_str(),
        );
        translate_x += options.text_box_width;
    }
    barcode_text(
        &mut svg,
        translate_x,
        &barcode[7..9],
        "4",
        options.get_character_colour('*'),
        text_box_middle_x.as_str(),
        text_box_middle_y.as_str(),
    ); // Last block as 2 characters and smaller

    svg.append_line(
        1, // Background outline
        format!(
            "<use href=\"#{}\" fill=\"none\" stroke=\"#000\" stroke-width=\"{}\"/>",
            BACKGROUND_ID, options.stroke_outer
        )
        .as_str(),
    );

    let result = svg.result();
    let _ = std::fs::write("test.svg", &result);

    Ok(result)
}

fn barcode_text(
    svg: &mut SvgLabel,
    translate_x: f64,
    text: &str,
    font_size: &str,
    colour: &str,
    tx: &str,
    ty: &str,
) {
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
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"{}\">{}</text>",
            tx, ty, font_size, text
        )
        .as_str(),
    );
    svg.append_line(1, "</g>");
}
