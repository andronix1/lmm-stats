use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::systems::get_owned::GetOwnedSystemsService;
use axum::{response::IntoResponse, Extension};

use crate::api::v1::auth::session::session::Session;

pub async fn get(
    session: Session,
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<GetOwnedSystemsService>>,
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.get_owned(tx, session.user_id), |_| unreachable!())
}