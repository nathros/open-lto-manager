use dioxus::prelude::*;

use crate::shared::models::database::label_preset::model_label_preset::LabelOptions;

#[post("/api/generate/label/lto/svg")]
pub async fn generate_svg_label(mut barcode: String, designation: String) -> Result<String> {
    use base64::prelude::*;

    use crate::{
        backend::generate::lto_label::svg::generate::generate_lto_label_svg,
        shared::models::database::label_preset::model_label_preset::LabelOptions,
    };

    barcode = LabelOptions::combine_label(&barcode, &designation);
    let options = LabelOptions::default_preview();

    // TODO avoid base64 encode
    match generate_lto_label_svg(barcode, options) {
        Ok(o) => Ok(format!(
            "data:image/svg+xml;base64,{}",
            BASE64_STANDARD.encode(o)
        )),
        Err(e) => Ok(e),
    }
}

#[post("/api/generate/label/lto/pdf")]
pub async fn generate_pdf_label(options: LabelOptions) -> Result<String> {
    use base64::prelude::*;

    use crate::backend::generate::lto_label::{
        pdf::{generate::generate_lto_label_pdf, page::PDFPageType},
        svg::generate::generate_lto_label_svg,
    };

    let barcodes: Vec<String> = options.generate_barcodes();

    let mut svg: Vec<String> = vec![];
    for barcode in barcodes {
        match generate_lto_label_svg(barcode, options.clone()) {
            Ok(s) => {
                svg.push(s);
            }
            Err(e) => return Ok(e),
        }
    }
    let pdf = generate_lto_label_pdf(svg, PDFPageType::A4);
    Ok(format!(
        "data:application/pdf;base64,{}",
        BASE64_STANDARD.encode(pdf)
    ))
}
