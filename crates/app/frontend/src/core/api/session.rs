use axum::async_trait;
use reqwest::RequestBuilder;

#[async_trait]
pub trait ApiSession: Sync + Send {
    async fn try_refresh(&mut self) -> bool;
    fn apply_to_builder(&self, builder: RequestBuilder) -> RequestBuilder;
}