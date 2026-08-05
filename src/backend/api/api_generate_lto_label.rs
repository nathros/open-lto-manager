#[cfg(feature = "server")]
use axum::{
    body::Body,
    response::{IntoResponse, Response},
};
use dioxus::{
    fullstack::{ClientResponse, FromResponse},
    prelude::*,
};

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

    // TODO avoid base64 encode, look into blob
    match generate_lto_label_svg(barcode, options) {
        Ok(o) => Ok(format!(
            "data:image/svg+xml;base64,{}",
            BASE64_STANDARD.encode(o)
        )),
        Err(e) => Ok(e),
    }
}

#[post("/api/generate/label/lto/pdf/b64")]
pub async fn generate_pdf_label(options: LabelOptions) -> Result<String> {
    use crate::backend::generate::lto_label::pdf::generate::generate_lto_label_pdf_options;
    use base64::prelude::*;

    // TODO avoid base64 encode, look into blob
    Ok(format!(
        "data:application/pdf;base64,{}",
        BASE64_STANDARD.encode(generate_lto_label_pdf_options(options))
    ))
}

#[post("/api/generate/label/lto/pdf/blob")]
pub async fn generate_pdf_label_download(options: LabelOptions) -> Result<PDFResponse> {
    use crate::backend::generate::lto_label::pdf::generate::generate_lto_label_pdf_options;
    Ok(PDFResponse {
        data: generate_lto_label_pdf_options(options),
    })
}

pub struct PDFResponse {
    pub data: Vec<u8>,
}

impl FromResponse for PDFResponse {
    async fn from_response(res: ClientResponse) -> std::result::Result<Self, ServerFnError> {
        Ok(PDFResponse {
            data: res.bytes().await?.to_vec(),
        })
    }
}

#[cfg(feature = "server")]
impl IntoResponse for PDFResponse {
    fn into_response(self) -> Response {
        match Response::builder()
            .status(StatusCode::OK)
            .header(axum::http::header::CONTENT_TYPE, "application/pdf")
            .header(
                axum::http::header::CONTENT_DISPOSITION,
                "attachment; filename=\"labels.pdf\"",
            )
            .body(Body::from(self.data))
        {
            Ok(final_response) => final_response,
            Err(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("Failed to generate PDF: {}", e)))
                .unwrap_or_default(),
        }
    }
}
