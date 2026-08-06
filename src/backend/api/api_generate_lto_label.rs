#[cfg(feature = "server")]
use axum::response::{IntoResponse, Response};
use dioxus::{
    fullstack::{ClientResponse, FromResponse},
    prelude::*,
};

#[cfg(feature = "server")]
use crate::backend::api::response::ResponseBuilder;
use crate::shared::models::database::label_preset::model_label_preset::LabelOptions;

#[post("/api/generate/label/lto/svg/b64")]
pub async fn generate_single_svg_label(mut barcode: String, designation: String) -> Result<String> {
    use base64::prelude::*;

    use crate::{
        backend::generate::lto_label::svg::generate::generate_lto_label_svg_single,
        shared::models::database::label_preset::model_label_preset::LabelOptions,
    };

    barcode = LabelOptions::combine_label(&barcode, &designation);
    let options = LabelOptions::default_preview();

    // TODO avoid base64 encode, look into blob
    match generate_lto_label_svg_single(barcode, options) {
        Ok(o) => Ok(format!(
            "data:image/svg+xml;base64,{}",
            BASE64_STANDARD.encode(o)
        )),
        Err(e) => Ok(e),
    }
}

#[post("/api/generate/label/lto/pdf/preview")]
pub async fn generate_label_preview(options: LabelOptions) -> Result<Vec<String>> {
    use crate::backend::generate::lto_label::svg::generate::generate_lto_label_svg_pages;
    Ok(generate_lto_label_svg_pages(&options)) // List of SVG pages
}

pub const GENERATE_PDF_LABEL_DOWNLOAD: &str = "/api/generate/label/lto/pdf";
#[post("/api/generate/label/lto/pdf")]
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
        match Response::content_disposition_pdf("labels", self.data) {
            Ok(final_response) => final_response,
            Err(error) => Response::internal_error(error),
        }
    }
}
