use std::ops::AsyncFnOnce;

use common::tokens::TokensPair;
use feature_tokens::TokenGenerator;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    #[serde(rename = "rid")]
    pub refresh_id: Uuid
}

pub trait RefreshTokenGenerator = TokenGenerator<RefreshTokenClaims> + Send + Sync;

#[derive(Serialize, Deserialize)]
pub struct RefreshableAccessTokenClaims<Info: Serialize + DeserializeOwned> {
    #[serde(flatten, bound="")]
    pub info: Info,
    #[serde(rename = "rid")]
    pub refresh_id: Uuid,
}

pub fn create_tokens_pair<ATI: Serialize + DeserializeOwned>(
    info: ATI, 
    atg: &(dyn TokenGenerator<RefreshableAccessTokenClaims<ATI>> + Send + Sync), 
    rtg: &dyn RefreshTokenGenerator
) -> anyhow::Result<TokensPair> {
    let refresh_id = Uuid::new_v4();
    Ok(TokensPair { 
        access_token: atg.generate(RefreshableAccessTokenClaims {
            info,
            refresh_id
        })?,
        refresh_token: rtg.generate(RefreshTokenClaims {
            refresh_id
        })?
    })
}

pub struct RefreshTokensError;

pub async fn refresh_tokens_pair<ATI: Serialize + DeserializeOwned, F: AsyncFnOnce(&mut ATI) -> anyhow::Result<bool>>(
    validate_and_update: F,
    pair: TokensPair, 
    atg: &(dyn TokenGenerator<RefreshableAccessTokenClaims<ATI>> + Send + Sync), 
    rtg: &dyn RefreshTokenGenerator
) -> anyhow::Result<Result<TokensPair, RefreshTokensError>> {
    let mut atc = atg.extract_payload(pair.access_token)?;
    let rtc: RefreshTokenClaims = match rtg.verify(pair.refresh_token)? {
        Ok(claims) => claims,
        Err(_) => return Ok(Err(RefreshTokensError))
    };
    if atc.refresh_id != rtc.refresh_id {
        return Ok(Err(RefreshTokensError));
    }
    if !validate_and_update(&mut atc.info).await? {
        return Ok(Err(RefreshTokensError));
    }
    Ok(Ok(create_tokens_pair(atc.info, atg, rtg)?))
}

pub struct UpdateAccessTokenError;

pub async fn update_access_token<ATI: Serialize + DeserializeOwned, F: AsyncFnOnce(&mut ATI) -> anyhow::Result<bool>>(
    validate_and_update: F,
    access_token: String, 
    atg: &(dyn TokenGenerator<RefreshableAccessTokenClaims<ATI>> + Send + Sync), 
) -> anyhow::Result<Result<String, UpdateAccessTokenError>> {
    let mut atc = atg.extract_payload(access_token)?;
    if !validate_and_update(&mut atc.info).await? {
        return Ok(Err(UpdateAccessTokenError));
    }
    Ok(Ok(atg.generate(atc)?))
}