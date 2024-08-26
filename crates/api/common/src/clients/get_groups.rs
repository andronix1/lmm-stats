use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::result::ApiResult;

#[derive(Deserialize, Serialize)]
pub struct GetGroupsGroupInfo {
    pub id: Uuid,
    pub name: String
}

pub type GetGroupsApiResponse = Vec<GetGroupsGroupInfo>;

pub type GetGroupsApiResult = ApiResult<GetGroupsApiResponse, ()>;