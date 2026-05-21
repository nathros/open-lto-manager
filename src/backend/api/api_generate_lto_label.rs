use dioxus::prelude::*;

#[post("/api/generate/label/lto/svg")]
pub async fn generate_svg_label(mut barcode: String, mut designation: String) -> Result<String> {
    use base64::prelude::*;

    use crate::{
        backend::generate::lto_label::svg::generate::generate_lto_label_svg,
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
        Ok(o) => {
            use crate::backend::generate::lto_label::pdf::{
                generate::generate_lto_label_pdf, page::PDFPageType,
            };

            // Test TODO remove
            let mut labels = vec![];
            for _i in 0..32 {
                labels.push(o.clone());
            }
            generate_lto_label_pdf(labels, PDFPageType::A4);
            // Test

            // TODO avoid base64 encode
            Ok(format!(
                "data:image/svg+xml;base64,{}",
                BASE64_STANDARD.encode(o)
            ))
        }
        Err(e) => Ok(e),
    }
}
