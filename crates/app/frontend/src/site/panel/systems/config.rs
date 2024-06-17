use std::sync::Arc;

use api_common::systems::{get_owned_info::GetOwnedSystemInfoApiResult, patch_owned::{PatchOwnedSystemApiRequest, PatchOwnedSystemApiResult}};
use axum::{extract::Path, response::{IntoResponse, Redirect, Response}, Extension, Form};
use reqwest::Method;
use serde::Deserialize;
use tera::{Context, Tera};

use crate::core::{api::{requester::ApiRequester, result::ApiResultWrapper}, session::Session, tera::TeraTemplate};

const PANEL_SYSTEM_TEMPLATE: &'static str = "panel/systems/config.tera";

#[derive(Deserialize)]
pub struct GetRequestQuery {
    name: String
}

async fn render_page(
    tera: Arc<Tera>,
    requester: Arc<ApiRequester>,
    mut session: Session,
    mut ctx: Context,
    name: String
) -> Response { 
    let result: GetOwnedSystemInfoApiResult = requester.request(Method::GET, &format!("/api/v1/systems/my/{}", name), Some(&mut session), |builder| builder).await;
    ApiResultWrapper(result).respond(
        |ok| {
            TeraTemplate::new(tera, PANEL_SYSTEM_TEMPLATE, {
                ctx.insert("systemName", &name);
                ctx.insert("system", &ok);
                ctx
            }).into_response()
        }, 
        |_| unreachable!()
    )
}

pub async fn get(
    Extension(tera): Extension<Arc<Tera>>,
    Extension(requester): Extension<Arc<ApiRequester>>,
    session: Session,
    Path(query): Path<GetRequestQuery>
) -> Response { render_page(tera, requester, session, Context::new(), query.name).await }

pub async fn post(
    Extension(tera): Extension<Arc<Tera>>,
    Extension(requester): Extension<Arc<ApiRequester>>,
    mut session: Session,
    Path(query): Path<GetRequestQuery>,
    Form(request): Form<PatchOwnedSystemApiRequest>
) -> Response { 
    let result: PatchOwnedSystemApiResult = requester.request(Method::PATCH, &format!("/api/v1/systems/my/{}", query.name), Some(&mut session), |builder| builder.json(&request)).await;
    ApiResultWrapper(result).respond_async(
        |_ok| async { Redirect::to("/panel/systems").into_response() },
        |err| async move {
            let mut ctx = Context::new();
            ctx.insert("error", &err);
            render_page(tera, requester, session, ctx, query.name).await
        }
    ).await
}