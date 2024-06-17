use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::result::ApiResult;

use super::shared::systems::SystemInfoApiResponse;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GetOwnedSystemInfoApiError {
    NotExists
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOwnedSystemInfoApiRequest {
    pub name: String,
    pub owner: Uuid,
}

pub type GetOwnedSystemInfoApiResult = ApiResult<SystemInfoApiResponse, GetOwnedSystemInfoApiError>;