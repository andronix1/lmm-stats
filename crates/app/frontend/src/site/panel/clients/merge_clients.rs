use std::sync::Arc;

use api_common::clients::merge_clients::{MergeClientsInGroupApiError, MergeClientsInGroupApiRequest, MergeClientsInGroupApiResult};
use axum::{extract::Path, response::{IntoResponse, Redirect, Response}, Extension, Json};
use reqwest::Method;
use serde::Deserialize;
use tera::{Context, Tera};
use uuid::Uuid;

use crate::core::{api::{requester::ApiRequester, result::ApiResultWrapper}, session::Session, tera::TeraTemplate};

const TEMPLATE: &'static str = "panel/clients/merge_clients.tera";
const REDIRECT_URL: &'static str = "/panel/clients";

#[derive(Deserialize)]
pub struct MergeClientsQuery {
    group_id: Uuid
}

pub async fn get(
    Extension(tera): Extension<Arc<Tera>>,
    Path(query): Path<MergeClientsQuery>
) -> Response {
    TeraTemplate::new(tera, TEMPLATE, {
        let mut ctx = Context::new();
        ctx.insert("group_id", &query.group_id);
        ctx
    }).into_response()
}

pub async fn post(
    mut session: Session,
    Extension(requester): Extension<Arc<ApiRequester>>,
    Path(query): Path<MergeClientsQuery>,
    Json(request): Json<MergeClientsInGroupApiRequest>
) -> Response {
    let result: MergeClientsInGroupApiResult = requester.request(Method::PATCH, &format!("/api/v1/clients/groups/{}", query.group_id), Some(&mut session), |builder| builder.json(&request)).await;
    ApiResultWrapper(result).respond(|_ok| {
        Redirect::to(REDIRECT_URL).into_response()
    }, |err| match err {
        MergeClientsInGroupApiError::GroupNotExists => Redirect::to(REDIRECT_URL).into_response(),
    })
}