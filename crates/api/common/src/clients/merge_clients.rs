use serde::{Deserialize, Serialize};

use crate::result::ApiResult;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MergeClientsInGroupApiError {
    GroupNotExists
}

pub type MergeClientsInGroupApiRequest = Vec<String>;

pub type MergeClientsInGroupApiResponse = ();

pub type MergeClientsInGroupApiResult = ApiResult<MergeClientsInGroupApiResponse, MergeClientsInGroupApiError>;