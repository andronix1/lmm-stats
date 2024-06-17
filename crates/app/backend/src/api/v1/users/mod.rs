use axum::{routing::put, Router};

use crate::session_geq;

mod create;

pub fn router() -> Router {
    Router::new()
        .route("/create", put(create::put).layer(session_geq!(UserRole::Superuser)))
}