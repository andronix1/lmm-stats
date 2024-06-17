use core::api::requester::ApiRequester;
use std::{env, sync::Arc};

use axum::Extension;
use dotenv::dotenv;
use reqwest::Url;
use tera::Tera;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

pub mod core;
pub mod site;

pub fn get_env(name: &str) -> String {
    env::var(name).expect(&format!("failed to get {name} env var"))
}

pub async fn run() -> anyhow::Result<()> {
    dotenv()?;
    env_logger::init();

    let tera = Arc::new(Tera::new("templates/**/*.tera")?);

    log::info!("starting server at address http://localhost:8080/");
    axum::serve(
        TcpListener::bind(get_env("FRONTEND_ADDR")).await?,
        axum::Router::new()
            .nest("/", site::router())
            .layer(Extension(tera))
            .layer(Extension(Arc::new(ApiRequester::new(Url::parse(&format!("http://{}", get_env("BACKEND_ADDR")))?))))
            .layer(CookieManagerLayer::new())
    ).await?;
    Ok(())
}