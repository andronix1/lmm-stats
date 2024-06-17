use axum::{extract::{Request, State}, middleware::Next, response::{IntoResponse, Response}, RequestExt};
use feature_users::domain::UserRole;

use crate::api::v1::response::forbidden_error;

use super::session::Session;

pub async fn session_geq_handler(
    State(role): State<UserRole>,
    mut request: Request,
    next: Next
) -> Response {
    match request.extract_parts::<Session>().await {
        Ok(session) => {
            fn role_order(role: UserRole) -> u32 {
                match role {
                    UserRole::Superuser => 2,
                    UserRole::Developer => 1,
                    UserRole::Viewer => 0,
                }
            }
            if role_order(session.role) < role_order(role) {
                forbidden_error()
            } else {
                next.run(request).await
            }
        },
        Err(err) => err.into_response(),
    }
}

#[macro_export] macro_rules! session_geq {
    ($role:expr) => {
        {
            use $crate::api::v1::auth::session::middlewares::session_geq_handler;
            use feature_users::domain::UserRole;
            use axum::middleware;
            middleware::from_fn_with_state::<_, UserRole, _>($role, session_geq_handler)
        }
    };
}