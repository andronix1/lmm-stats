use serde::{Deserialize, Serialize};

use crate::result::ApiResult;

use crate::tokens::TokensPair;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum RefreshSessionApiError {
    InvalidTokens
}

pub type RefreshSessionApiRequest = TokensPair;

pub type RefreshSessionApiResponse = TokensPair;

pub type RefreshSessionApiResult = ApiResult<RefreshSessionApiResponse, RefreshSessionApiError>;