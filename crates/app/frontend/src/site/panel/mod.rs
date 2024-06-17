use axum::Router;

pub mod systems;
// pub mod stats;
pub mod clients;
// pub mod users;

pub fn router() -> Router {
    Router::new()
        .nest("/systems", systems::router())
        .nest("/clients", clients::router())
        // .route("/stats", get(stats::get))
        // .route("/users", get(users::get))
}