use axum::{routing::{get, post}, Router};

mod owned;
mod event_create;
mod event_delete;
mod config;
mod create;
mod events;

pub fn router() -> Router {
    Router::new()
        .route("/", get(owned::get))
        .route("/create", get(create::get).post(create::post))
        .route("/my/:name/config", get(config::get).post(config::post))
        .route("/my/:name/events", get(events::get))
        .route("/my/:system/events/create", get(event_create::get).post(event_create::post))
        .route("/my/:system/events/delete/:event", post(event_delete::post))
        .route("/my/:system/create", get(create::get).post(create::post))
}