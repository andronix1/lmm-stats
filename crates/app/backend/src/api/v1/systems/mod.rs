use axum::{routing::{delete, get, put}, Router};

use crate::session_geq;

mod create_event;
mod delete_event;
mod get_events_of_owned;
mod patch_owned;
mod get_my;
mod create;
mod get_owned_info;
mod delete_system;

pub fn router() -> Router {
    Router::new()
        .route("/my", get(get_my::get))
        .route("/my/:name", get(get_owned_info::get).patch(patch_owned::patch).delete(delete_system::delete))
        .route("/my/:name/events", get(get_events_of_owned::get).put(create_event::put))
        .route("/my/:system/events/:event", delete(delete_event::delete))
        .route("/create", put(create::put))
        .layer(session_geq!(UserRole::Developer))
}