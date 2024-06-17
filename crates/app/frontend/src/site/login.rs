use std::sync::Arc;

use api_common::auth::login::LoginApiResult;
use axum::{response::{IntoResponse, Redirect, Response}, Extension, Form};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tower_cookies::Cookies;

use crate::core::{api::{requester::ApiRequester, result::ApiResultWrapper}, session::set_tokens, tera::TeraTemplate};

const LOGIN_TEMPLATE: &'static str = "login.tera";

pub async fn get(
    Extension(tera): Extension<Arc<Tera>>
) -> Response { TeraTemplate::new(tera, LOGIN_TEMPLATE, Context::new()).into_response() }

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub login: String,
    pub password: String
}

pub async fn post(
    Extension(tera): Extension<Arc<Tera>>,
    Extension(requester): Extension<Arc<ApiRequester>>,
    mut cookies: Cookies,
    Form(credentials): Form<LoginRequest>
) -> Response {
    let result: LoginApiResult = requester.request(Method::POST, "/api/v1/auth/login", None, |builder| builder.json(&credentials)).await;
    ApiResultWrapper(result).respond(
        |ok| {
            set_tokens(&mut cookies, ok);
            Redirect::to("/panel/systems").into_response()
        }, 
        |err| TeraTemplate::new(tera.clone(), LOGIN_TEMPLATE, {
            let mut ctx = Context::new();
            ctx.insert("error", &err);
            ctx
        }).into_response()
    )
}