use serde::{Deserialize, Serialize};

use crate::result::ApiResult;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PatchOwnedSystemApiError {
    NotExists,
    HumanNameNotUnique,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchOwnedSystemApiRequest {
    #[serde(default)]
    pub human_name: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub secret: Option<Option<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchOwnedSystemApiResponse;

pub type PatchOwnedSystemApiResult = ApiResult<PatchOwnedSystemApiResponse, PatchOwnedSystemApiError>;