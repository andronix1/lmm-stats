use std::sync::Arc;

use api_common::{auth::refresh_session::RefreshSessionApiResult, tokens::TokensPair};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, response::{IntoResponse, Response}, Extension, RequestPartsExt};
use reqwest::{Method, RequestBuilder};
use tower_cookies::{Cookie, Cookies};

use super::{api::{requester::ApiRequester, session::ApiSession}, errors::{internal::InternalError, unauthorized::UnauthorizedError}};

#[derive(Clone)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    pub cookies: Cookies,
    pub requester: Arc<ApiRequester>,
}

#[async_trait]
impl<S> FromRequestParts<S> for Session {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookies = parts.extract::<Cookies>().await.map_err(|_| InternalError.into_response())?;
        let Extension(requester) = parts.extract::<Extension<Arc<ApiRequester>>>().await.map_err(|_| InternalError.into_response())?;
        let access_token = cookies.get("access").ok_or_else(|| UnauthorizedError.into_response())?.value().to_owned();
        let refresh_token = cookies.get("refresh").ok_or_else(|| UnauthorizedError.into_response())?.value().to_owned();
        Ok(Session { access_token, refresh_token, cookies, requester })
    }
}

pub fn set_tokens(cookies: &mut Cookies, pair: TokensPair) {
    let mut access_token = Cookie::new("access", pair.access_token);
    access_token.set_http_only(true);
    cookies.add(access_token);
    let mut refresh_token = Cookie::new("refresh", pair.refresh_token);
    refresh_token.set_http_only(true);
    cookies.add(refresh_token);
}


#[async_trait]
impl ApiSession for Session {
    async fn try_refresh(&mut self) -> bool {
        let pair = TokensPair {
            access_token: self.access_token.clone(),
            refresh_token: self.refresh_token.clone(),
        };
        if let RefreshSessionApiResult::Success(new_tokens) = self.requester.clone().request(Method::POST, "/api/v1/auth/refreshSession", None, |builder| builder.json(&pair)).await {
            self.access_token = new_tokens.access_token.clone();
            self.refresh_token = new_tokens.refresh_token.clone();
            set_tokens(&mut self.cookies, new_tokens);
            true
        } else {
            false
        }
    }

    fn apply_to_builder(&self, builder: RequestBuilder) -> RequestBuilder {
        builder.header("Authorization", format!("Bearer {}", self.access_token))
    }
}