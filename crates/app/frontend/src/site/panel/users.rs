use std::sync::Arc;

use axum::{response::{IntoResponse, Response}, Extension};
use tera::{Context, Tera};

use crate::core::tera::TeraTemplate;


const PANEL_USERS_TEMPLATE: &'static str = "panel/users.tera";

pub async fn get(
    Extension(tera): Extension<Arc<Tera>>
) -> Response { TeraTemplate::new(tera, PANEL_USERS_TEMPLATE, Context::new()).into_response() }