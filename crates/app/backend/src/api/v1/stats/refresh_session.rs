use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{common::stats::refresh_session::StatsRefreshSessionApiRequest, stats::refresh_session::StatsRefreshSessionService};
use axum::{response::IntoResponse, Extension, Json};

pub async fn post(
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<StatsRefreshSessionService>>,
    Json(request): Json<StatsRefreshSessionApiRequest>
) -> impl IntoResponse {
    api_transactional!(db, |tx| service.refresh_session(tx, request), |_err| unreachable!())
}