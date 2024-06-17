use feature_tokens::TokenGenerator;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tokens_pair::RefreshableAccessTokenClaims;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenInfo {
    pub id: Uuid,
    #[serde(rename = "rev")]
    pub user_revision: i32,
}

pub type AccessTokenClaims = RefreshableAccessTokenClaims<AccessTokenInfo>;

pub trait AccessTokenGenerator = TokenGenerator<AccessTokenClaims> + Send + Sync;
