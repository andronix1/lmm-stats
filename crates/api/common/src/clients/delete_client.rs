
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::result::ApiResult;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeleteClientApiError {
    NotExists
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteClientApiRequest {
    pub id: Uuid
}

pub type DeleteClientApiResponse = ();

pub type DeleteClientApiResult = ApiResult<DeleteClientApiResponse, DeleteClientApiError>;