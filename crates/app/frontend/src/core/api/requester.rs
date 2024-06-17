use api_common::result::ApiResult;
use derive_new::new;
use reqwest::{Client, Method, RequestBuilder, Url};
use serde::{de::DeserializeOwned, Serialize};

use super::session::ApiSession;

#[derive(new)]
pub struct ApiRequester {
    base_url: Url
}

impl ApiRequester {
    pub async fn request<
        T: Serialize + DeserializeOwned, 
        E: Serialize + DeserializeOwned, 
        F: Fn(RequestBuilder) -> RequestBuilder,
    >(&self, method: Method, path: &str, mut session: Option<&mut dyn ApiSession>, prebuild: F) -> ApiResult<T, E> { 
        let client = Client::builder().build()
            .map_err(From::from)?;
        let mut url = self.base_url.clone();
        url.set_path(&path);
        let mut builder = prebuild(client.request(method.clone(), url));
        if let Some(session) = &session {
            builder = session.apply_to_builder(builder);
        }
        let response = builder.send()
            .await.map_err(From::from)?;
        log::debug!("request {path} finished with status {}", response.status());
        let result = response
            .json()
            .await.map_err(From::from)?;
        if let Some(my_session) = &mut session {
            if let ApiResult::Unauthorized = &result {
                log::info!("refreshing session on {path}!");
                if my_session.try_refresh().await {
                    return Box::pin(self.request(method, path, session, prebuild)).await;
                }
            }
        }
        result
    }
}