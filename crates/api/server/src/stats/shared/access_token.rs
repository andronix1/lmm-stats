use feature_tokens::TokenGenerator;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tokens_pair::RefreshableAccessTokenClaims;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsAccessTokenInfo {
    #[serde(default, rename = "cid")]
    pub client_id: Option<Uuid>,
    #[serde(rename = "sys")]
    pub system: String,
}

pub type StatsAccessTokenClaims = RefreshableAccessTokenClaims<StatsAccessTokenInfo>;

pub trait StatsAccessTokenGenerator = TokenGenerator<StatsAccessTokenClaims> + Send + Sync;
