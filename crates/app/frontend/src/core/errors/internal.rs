use axum::response::{Html, IntoResponse, Response};
use reqwest::StatusCode;

pub struct InternalError;

impl IntoResponse for InternalError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Html("internal error")).into_response()
    }
}