use std::future::Future;

use api_common::result::ApiResult;
use axum::response::{IntoResponse, Response};
use serde::{de::DeserializeOwned, Serialize};

use crate::core::errors::{forbidden::ForbiddenError, internal::InternalError, unauthorized::UnauthorizedError};

pub struct ApiResultWrapper<T: Serialize + DeserializeOwned, E: Serialize + DeserializeOwned>(pub ApiResult<T, E>);

impl<T: Serialize + DeserializeOwned, E: Serialize + DeserializeOwned> ApiResultWrapper<T, E> {
    pub fn respond<FOk: FnOnce(T) -> Response, FErr: FnOnce(E) -> Response>(self, f_ok: FOk, f_err: FErr) -> Response {
        match self.0 {
            ApiResult::Success(ok) => f_ok(ok),
            ApiResult::Error(err) => f_err(err),
            ApiResult::Unauthorized => UnauthorizedError.into_response(),
            ApiResult::InternalError => InternalError.into_response(),
            ApiResult::Forbidden => ForbiddenError.into_response(),
        }
    }

    pub async fn respond_async<FOk: FnOnce(T) -> FOkRes, FOkRes: Future<Output = Response>, FErr: FnOnce(E) -> FErrRes, FErrRes: Future<Output = Response>>(self, f_ok: FOk, f_err: FErr) -> Response {
        match self.0 {
            ApiResult::Success(ok) => f_ok(ok).await,
            ApiResult::Error(err) => f_err(err).await,
            ApiResult::Unauthorized => UnauthorizedError.into_response(),
            ApiResult::InternalError => InternalError.into_response(),
            ApiResult::Forbidden => ForbiddenError.into_response(),
        }
    }
}