use std::collections::HashSet;

use crate::shared::error::ErrorStr;

use super::{
    segment::{BARCODE_VALID_CHARS, CODE_39_BARCODE_SEGMENTS},
    svg::Svg,
};

pub fn generate_lto_label_svg(mut barcode: String) -> Result<String, ErrorStr> {
    barcode.push_str(
        (0..(8 - barcode.len()))
            .map(|_| " ")
            .collect::<String>()
            .as_str(),
    );
    barcode.push('*');
    barcode.insert(0, '*');

    if barcode.len() > 10 {
        return Err("Barcode too long".to_string());
    }

    let mut svg = Svg::new();
    //svg.append_line(0, format!("<!--{}-->", barcode).as_str());

    let mut unique_characters: HashSet<i64> = HashSet::new();
    for char in barcode.chars() {
        if !BARCODE_VALID_CHARS.contains(char) {
            return Err(format!("Invalid character: {}", char));
        }
        let index = char as i64;
        unique_characters.insert(index);
    }

    svg.append_group(
        1,
        "defs",
        Box::new(move |tab_index: i32, svg: &mut Svg| {
            for index in unique_characters.iter() {
                if let Some(segment_gen) = CODE_39_BARCODE_SEGMENTS.get(index) {
                    svg.append_line(
                        tab_index,
                        format!("<svg id=\"{}\" width=\"6.588mm\" height=\"11.7mm\">", index)
                            .as_str(),
                    );
                    for segment in segment_gen.create_segment("11.7") {
                        svg.append_line(tab_index + 1, segment.as_str());
                    }
                    svg.append_line(tab_index, "</svg>");
                }
            }
            svg.append_line(tab_index, "<rect id=\"t\" width=\"10\" height=\"5.8\" x=\"0\" y=\"0\" rx=\"0\" ry=\"0\" stroke=\"#000\" stroke-width=\"0.035\" />");
        }),
    );

    let mut translate_x = 7.31;
    // Add barcode vertical lines
    for char in barcode.chars() {
        svg.append_line(
            1,
            format!(
                "<use href=\"#{}\" transform=\"translate({} 5.8)\"/>",
                char as i32, translate_x
            )
            .as_str(),
        );
        translate_x += 6.588;
    }

    translate_x = 5.25;
    // Add barcode text, skip first and last '*'
    for itr in barcode.chars().skip(2).take(barcode.len() - 4).enumerate() {
        barcode_text(
            &mut svg,
            translate_x,
            &barcode[(itr.0 + 1)..(itr.0 + 2)],
            "5",
        );
        translate_x += 10_f64;
    }
    barcode_text(&mut svg, translate_x, &barcode[7..9], "4"); // Last block as 2 characters and smaller

    let result = svg.result();
    //let _ = std::fs::write("test.svg", &result);

    Ok(result)
}

fn barcode_text(svg: &mut Svg, translate_x: f64, text: &str, font_size: &str) {
    // svg.append_line(0, format!("<!--{}-->", text).as_str());s
    svg.append_line(
        1,
        format!("<g transform=\"translate({} 1)\">", translate_x).as_str(),
    );
    svg.append_line(2, "<use href=\"#t\" fill=\"orange\" />");
    svg.append_line(
        2,
        format!("<text x=\"5\" y=\"5\" text-anchor=\"middle\" font-size=\"{}\" font-family=\"sans-serif\">{}</text>", font_size, text).as_str(),
    );
    svg.append_line(1, "</g>");
}
