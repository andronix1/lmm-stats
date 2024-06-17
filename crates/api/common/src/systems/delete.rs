use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::result::ApiResult;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeleteSystemApiError {
    NotExists
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSystemApiRequest {
    pub owner: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteSystemApiResponse;

pub type DeleteSystemApiResult = ApiResult<DeleteSystemApiResponse, DeleteSystemApiError>;