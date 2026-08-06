use axum::{
    body::Body,
    http::{Error, StatusCode, header},
    response::Response,
};

pub trait ResponseBuilder {
    fn content_disposition_pdf(filename: &str, data: Vec<u8>) -> Result<Response, Error>;
    fn internal_error(error: Error) -> Response;
}

impl ResponseBuilder for Response {
    fn content_disposition_pdf(filename: &str, data: Vec<u8>) -> Result<Response, Error> {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/pdf")
            .header(
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}.pdf\"", filename),
            )
            .body(Body::from(data))
    }

    fn internal_error(error: Error) -> Response {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(format!("Request failed: {}", error)))
            .unwrap_or_default()
    }
}
