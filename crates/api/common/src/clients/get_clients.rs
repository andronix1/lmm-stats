use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::result::ApiResult;

pub type GetClientsApiError = ();

pub type GetClientsApiRequest = ();

#[derive(Serialize, Deserialize)]
pub struct GetClientsApiGroup {
    pub name: String,
    pub clients: LinkedHashMap<Uuid, String>
}

pub type GetClientsApiResponse = LinkedHashMap<Uuid, GetClientsApiGroup>;

pub type GetClientsApiResult = ApiResult<GetClientsApiResponse, GetClientsApiError>;