use axum::{routing::{get, post}, Router};

pub mod list;
pub mod create_group;
pub mod delete_group;
pub mod merge_clients;
pub mod delete_client;

pub fn router() -> Router {
    Router::new()
        .route("/", get(list::get))
        .route("/groups/create", post(create_group::post))
        .route("/groups/:group_id/delete", get(delete_group::get))
        .route("/groups/:group_id/merge-clients", get(merge_clients::get).post(merge_clients::post))
        .route("/:client_id/delete", get(delete_client::get))
}