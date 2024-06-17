
use serde::{Deserialize, Serialize};

use crate::result::ApiResult;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CreateGroupApiError {
    NameNotUnique
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupApiRequest {
    pub name: String
}

pub type CreateGroupApiResponse = ();

pub type CreateGroupApiResult = ApiResult<CreateGroupApiResponse, CreateGroupApiError>;