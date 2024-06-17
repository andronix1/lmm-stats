use axum::response::{Redirect, IntoResponse, Response};

pub struct UnauthorizedError;

impl IntoResponse for UnauthorizedError {
    fn into_response(self) -> Response {
        Redirect::to("/login").into_response()
    }
}