use std::marker::PhantomData;
use derive_new::new;
use jwt_simple::{algorithms::{HS256Key, MACLike}, claims::Claims, common::VerificationOptions, reexports::coarsetime::Duration, JWTError};
use crate::{TokenGenerator, TokenVerifyError};
use serde::{Deserialize, Serialize};

#[derive(new)]
pub struct JwtTokenGenerator<T: Serialize + for<'a> Deserialize<'a>, Key: MACLike> {
    key: Key,
    expire_time: Duration,
    #[new(default)]
    phantom_claims: PhantomData<T>,
}

impl<T: Serialize + for<'a> Deserialize<'a>, Key: MACLike> TokenGenerator<T> for JwtTokenGenerator<T, Key> {
    fn generate(&self, claims: T) -> anyhow::Result<String> {
        self.key.authenticate(Claims::with_custom_claims(claims, self.expire_time))
    }

    fn verify(&self, token: String) -> anyhow::Result<Result<T, TokenVerifyError>> {
        let mut opts = VerificationOptions::default();
        opts.time_tolerance = None;
        match self.key.verify_token(token.as_str(), Some(opts)) {
            Ok(claims) => Ok(Ok(claims.custom)),
            Err(err) => if let Some(JWTError::TokenHasExpired) = err.downcast_ref() {
                Ok(Err(TokenVerifyError::Expired))
            } else {
                Err(err)
            },
        }
    }
    
    fn extract_payload(&self, token: String) -> jwt_simple::reexports::anyhow::Result<T> {
        let mut options = VerificationOptions::default();
        options.time_tolerance = Some(Duration::from_days(365 * 10)); // jwt_timple library hasn't ignore expiration feature :( 
        self.key.verify_token(token.as_str(), Some(options)).map(|res| res.custom)
    }
}

pub fn create_hs256_key(bytes: &[u8]) -> impl MACLike {
    HS256Key::from_bytes(bytes)
}

pub fn duration_from_secs(secs: u64) -> Duration {
    Duration::from_secs(secs)
}