use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::result::ApiResult;

use super::UserRole;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct GetSessionInfoApiRequest {
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct GetSessionInfoApiResponse {
    pub id: Uuid,
    pub role: UserRole,
}

pub type GetSessionInfoApiResult = ApiResult<GetSessionInfoApiResponse, ()>;