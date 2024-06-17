use axum::{routing::get, Router};

mod panel;
mod login;

pub fn router() -> Router {
    Router::new()
        .route("/login", get(login::get).post(login::post))
        .nest("/panel", panel::router())
}