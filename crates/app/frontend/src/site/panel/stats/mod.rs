use axum::{routing::get, Router};
use serde::Deserialize;

mod set_active;
mod list;
mod system;

#[derive(Deserialize)]
pub struct SystemPath {
    name: String
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(list::get))
        .route("/:name/setActive", get(set_active::get))
        .route("/:name", get(system::get))
}