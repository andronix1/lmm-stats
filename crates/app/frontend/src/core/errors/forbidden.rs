use axum::response::{Html, IntoResponse, Response};
use reqwest::StatusCode;

pub struct ForbiddenError;

impl IntoResponse for ForbiddenError {
    fn into_response(self) -> Response {
        (StatusCode::FORBIDDEN, Html("forbidden")).into_response()
    }
}