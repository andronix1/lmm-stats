use std::sync::Arc;

use api_common::systems::create::{CreateSystemApiRequest, CreateSystemApiResult};
use axum::{response::{IntoResponse, Redirect, Response}, Extension, Form};
use reqwest::Method;
use serde::Deserialize;
use tera::{Context, Tera};

use crate::core::{api::{requester::ApiRequester, result::ApiResultWrapper}, session::Session, tera::TeraTemplate};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSystemRequest {
    pub name: String,
    pub human_name: String,
    #[serde(default)]
    pub secret: Option<String>
}

const PANEL_CREATE_SYSTEM_TEMPLATE: &'static str = "panel/systems/create.tera";

pub async fn get(
    Extension(tera): Extension<Arc<Tera>>,
) -> Response { TeraTemplate::new(tera, PANEL_CREATE_SYSTEM_TEMPLATE, Context::new()).into_response() }

pub async fn post(
    Extension(tera): Extension<Arc<Tera>>,
    Extension(requester): Extension<Arc<ApiRequester>>,
    mut session: Session,
    Form(request): Form<CreateSystemRequest>
) -> Response {
    let result: CreateSystemApiResult = requester.request(Method::PUT, "/api/v1/systems/create", Some(&mut session), |builder| builder.json(&CreateSystemApiRequest { 
        name: request.name.clone(),
        human_name: request.human_name.clone(),
        secret: request.secret.clone()
    })).await;
    ApiResultWrapper(result).respond(
        |_ok| {
            Redirect::to("/panel/systems").into_response()
        }, |err| {
            TeraTemplate::new(tera, PANEL_CREATE_SYSTEM_TEMPLATE, {
                let mut ctx = Context::new();
                ctx.insert("error", &err);
                ctx
            }).into_response()
        }
    )
}