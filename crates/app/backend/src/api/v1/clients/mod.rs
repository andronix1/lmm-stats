use axum::{routing::{get, patch, put}, Router};

mod create_group;
mod get_clients;
mod merge_clients;
mod delete_client;
mod delete_group;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_clients::get))
        .route("/groups", put(create_group::put))
        .route("/groups/:group_id", patch(merge_clients::patch).delete(delete_group::delete))
        .route("/:client_id", patch(merge_clients::patch).delete(delete_client::delete))
}