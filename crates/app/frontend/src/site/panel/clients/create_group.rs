use std::sync::Arc;

use api_common::clients::create_group::{CreateGroupApiError, CreateGroupApiRequest, CreateGroupApiResult};
use axum::{response::{IntoResponse, Redirect, Response}, Extension, Form};
use reqwest::Method;
use tera::{Context, Tera};

use crate::core::{api::{requester::ApiRequester, result::ApiResultWrapper}, session::Session, tera::TeraTemplate};

const TEMPLATE: &'static str = "panel/clients/group_exists.tera";
const REDIRECT_URL: &'static str = "/panel/clients";

pub async fn post(
    mut session: Session,
    Extension(tera): Extension<Arc<Tera>>,
    Extension(requester): Extension<Arc<ApiRequester>>,
    Form(request): Form<CreateGroupApiRequest>
) -> Response {
    let result: CreateGroupApiResult = requester.request(Method::PUT, "/api/v1/clients/groups", Some(&mut session), |builder| builder.json(&request)).await;
    ApiResultWrapper(result).respond(|_ok| {
        Redirect::to(REDIRECT_URL).into_response()
    }, |err| match err {
        CreateGroupApiError::NameNotUnique => TeraTemplate::new(tera, TEMPLATE, {
            let mut ctx = Context::new();
            ctx.insert("redirectUrl", REDIRECT_URL);
            ctx
        }).into_response(),
    })
}