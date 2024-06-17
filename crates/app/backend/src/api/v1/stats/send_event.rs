use std::sync::Arc;

use abstract_sqlx_bindings::SqlxDb;
use api_server::{common::stats::send_event::{StatsSendEventApiError, StatsSendEventApiRequest}, stats::send_event::StatsSendEventService};
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Extension};

use crate::api::v1::response::unauthorized_error;

use super::session::session::StatsSession;

pub async fn put(
    session: StatsSession,
    Extension(db): Extension<Arc<SqlxDb>>,
    Extension(service): Extension<Arc<StatsSendEventService>>,
    Query(request): Query<StatsSendEventApiRequest>
) -> impl IntoResponse {
    if let Some(client_id) = session.client_id {
        api_transactional!(db, |tx| service.send_event(tx, session.system, client_id, request), |err| match err {
            StatsSendEventApiError::InvalidEvent => StatusCode::NOT_FOUND,
        })
    } else {
        unauthorized_error()
    }
}