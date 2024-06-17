use std::fmt::Debug;

use api_server::common::result::ApiResult;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::{de::DeserializeOwned, Serialize};

#[macro_export] macro_rules! api_result_transactional {
    ($($db:ident).*, |$tx:ident| $scope:expr) => {
        {
            use abstract_db::transactional;
            use api_server::common::result::ApiResult;
            use anyhow::anyhow;

            match transactional!($($db).*, |$tx| async {
                match $scope.await {
                    ApiResult::InternalError => Err(anyhow!("internal error while transaction in progress")),
                    val => Ok(val)
                }
            }).await {
                Ok(result) => result,
                Err(err) => {
                    log::error!("api error: {err:?}");
                    ApiResult::InternalError
                }
            }
        }
    };
}

#[macro_export] macro_rules! api_transactional {
    ($($db:ident).*, |$tx:ident| $scope:expr, $err_block:expr) => {
        {
            use $crate::api::v1::response::finish_api_result;
            finish_api_result(api_result_transactional!($($db).*, |$tx| $scope), $err_block)
        }
    };
}

pub fn finish_api_result<T: Serialize + DeserializeOwned, E: Serialize + DeserializeOwned, F: FnOnce(&E) -> StatusCode>(result: ApiResult<T, E>, map_err: F) -> Response {
    match result {
        ApiResult::Success(res) => (StatusCode::OK, Json(ApiResult::<T, E>::Success(res))).into_response(),
        ApiResult::Error(err) => (map_err(&err), Json(ApiResult::<T, E>::Error(err))).into_response(),
        ApiResult::Unauthorized => unauthorized_error(),
        ApiResult::InternalError => internal_error(),
        ApiResult::Forbidden => forbidden_error(),
    }
}

pub fn internal_error() -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResult::<(), ()>::InternalError)).into_response()
}

pub fn internal_error_arg<T: Debug>(err: T) -> Response {
    log::error!("internal error: {err:?}");
    (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResult::<(), ()>::InternalError)).into_response()
}

pub fn unauthorized_error() -> Response {
    (StatusCode::UNAUTHORIZED, Json(ApiResult::<(), ()>::Unauthorized)).into_response()
}

pub fn forbidden_error() -> Response {
    (StatusCode::FORBIDDEN, Json(ApiResult::<(), ()>::Forbidden)).into_response()
}