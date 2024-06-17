use axum::{routing::{get, post, put}, Router};
use serde::Deserialize;

pub mod refresh_session;
pub mod get_auth_info;
pub mod start_auth;
pub mod finish_auth;
pub mod get_clients;
pub mod send_event;
pub mod session;

#[derive(Deserialize)]
pub struct SystemScope {
    system: String
}

pub fn router() -> Router {
    Router::new()
        .route("/auth/info/:system", get(get_auth_info::get))
        .route("/auth/start/:system", post(start_auth::post))
        .route("/auth/finish", post(finish_auth::post))
        .route("/auth/refresh", post(refresh_session::post))
        .route("/clients", get(get_clients::get))
        .route("/events", put(send_event::put))
}