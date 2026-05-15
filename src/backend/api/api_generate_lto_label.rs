use dioxus::prelude::*;

#[post("/api/generate/label/lto/svg")]
pub async fn generate_svg_label(mut barcode: String, mut designation: String) -> Result<String> {
    use base64::prelude::*;

    use crate::{
        backend::generate::code39::generate::generate_lto_label_svg,
        shared::models::database::model_label_preset::LabelOptions,
    };

    if designation.len() != 2 {
        designation = "  ".to_string();
    }
    barcode.push_str(
        (0..(6 - barcode.len()))
            .map(|_| " ")
            .collect::<String>()
            .as_str(),
    );
    barcode.push_str(designation.as_str());

    let options = LabelOptions::default();

    match generate_lto_label_svg(barcode, options) {
        Ok(o) => Ok(format!(
            "data:image/svg+xml;base64,{}",
            BASE64_STANDARD.encode(o)
        )),
        Err(e) => Ok(e),
    }
}
