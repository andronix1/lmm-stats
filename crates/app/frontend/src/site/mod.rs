use axum::{routing::get, Router};
use tower_http::services::ServeDir;

mod panel;
mod login;

pub fn router() -> Router {
    Router::new()
        .route("/login", get(login::get).post(login::post))
        .nest("/panel", panel::router())
        .nest_service("/", ServeDir::new("static"))
}