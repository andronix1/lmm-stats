use std::{convert::Infallible, ops::FromResidual};

use serde::{de::DeserializeOwned, Deserialize};
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "kind", content = "result")]
pub enum ApiResult<T: Serialize + DeserializeOwned, E: Serialize + DeserializeOwned> {
    Success(#[serde(bound = "")] T),
    Error(#[serde(bound = "")] E),
    Unauthorized,
    Forbidden,
    InternalError
}

impl<T: Serialize + DeserializeOwned, E: Serialize + DeserializeOwned> FromResidual<Result<Infallible, anyhow::Error>> for ApiResult<T, E> {
    fn from_residual(residual: Result<Infallible, anyhow::Error>) -> Self {
        log::error!("api error: {}", residual.unwrap_err());
        Self::InternalError
    }
}