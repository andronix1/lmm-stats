use axum::{routing::post, Router};

mod login;
mod refresh_session;
#[macro_use] pub mod session;

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login::post))
        .route("/refreshSession", post(refresh_session::post))
}