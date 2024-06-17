use serde::{Deserialize, Serialize};

use crate::result::ApiResult;

use crate::tokens::TokensPair;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum LoginApiError {
    InvalidCredentials
}

#[derive(Serialize, Deserialize)]
pub struct LoginApiRequest {
    pub login: String,
    pub password: String,
}

pub type LoginApiResponse = TokensPair;

pub type LoginApiResult = ApiResult<LoginApiResponse, LoginApiError>;