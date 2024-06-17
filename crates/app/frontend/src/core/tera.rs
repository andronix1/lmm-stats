use std::sync::Arc;

use axum::response::{Html, IntoResponse, Response};
use derive_new::new;
use tera::{Context, Tera};

use crate::core::errors::internal::InternalError;

#[derive(new)]
pub struct TeraTemplate {
    tera: Arc<Tera>, 
    template: &'static str, 
    context: Context
}

impl IntoResponse for TeraTemplate {
    fn into_response(self) -> Response {
        let tera = if cfg!(debug_assertions) { // dynamic reloading
            let mut tera = self.tera.as_ref().clone();
            if let Err(err) = tera.full_reload() {
                log::error!("failed to reload tera engine: {err:?}");
                return InternalError.into_response();
            }
            Arc::new(tera)
        } else { 
            self.tera
        };
        Html(match tera.render(&self.template, &self.context) {
            Ok(html) => html,
            Err(err) => {
                log::error!("failed to render tera template: {err:?}");
                return InternalError.into_response();
            },
        }).into_response()
    }
}