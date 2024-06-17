use axum::Router;

#[macro_use] pub mod response;

mod auth;
mod users;
mod systems;
mod clients;
mod stats;

pub fn router() -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/systems", systems::router())
        .nest("/stats", stats::router())
        .nest("/clients", clients::router())
}