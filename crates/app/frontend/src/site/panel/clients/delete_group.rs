use std::sync::Arc;

use api_common::clients::delete_group::{DeleteGroupApiError, DeleteGroupApiResult};
use axum::{extract::Path, response::{IntoResponse, Redirect, Response}, Extension};
use reqwest::Method;
use serde::Deserialize;
use uuid::Uuid;

use crate::core::{api::{requester::ApiRequester, result::ApiResultWrapper}, session::Session};

const REDIRECT_URL: &'static str = "/panel/clients";

#[derive(Deserialize)]
pub struct DeleteGroupQuery {
    group_id: Uuid
}

pub async fn get(
    mut session: Session,
    Extension(requester): Extension<Arc<ApiRequester>>,
    Path(query): Path<DeleteGroupQuery>
) -> Response {
    let result: DeleteGroupApiResult = requester.request(Method::DELETE, &format!("/api/v1/clients/groups/{}", query.group_id), Some(&mut session), |builder| builder).await;
    ApiResultWrapper(result).respond(|_ok| Redirect::to(REDIRECT_URL).into_response(), |err| match err {
        DeleteGroupApiError::NotExists => Redirect::to(REDIRECT_URL).into_response(),
    })
}