#![feature(try_trait_v2)]

use std::{env, sync::Arc};
use abstract_sqlx_bindings::SqlxDb;
use anyhow::anyhow;
use api_server::{auth::{get_session_info::GetSessionInfoService, login::LoginService, refresh_session::RefreshSessionService, shared::access_token::AccessTokenGenerator}, clients::{create_group::CreateGroupService, delete_client::DeleteClientService, delete_group::DeleteGroupService, get_clients::GetClientsService, merge_clients::MergeClientsInGroupService}, stats::{auth::StatsAuthService, finish_auth::StatsFinishAuthService, get_auth_info::StatsGetAuthInfoService, get_session_info::StatsGetSessionInfoService, refresh_session::StatsRefreshSessionService, send_event::StatsSendEventService, shared::access_token::StatsAccessTokenGenerator}, systems::{create::CreateSystemService, delete::DeleteSystemService, event_create::SystemEventCreateService, event_delete::SystemEventDeleteService, get_events_of_owned::GetEventsOfOwnedService, get_owned::GetOwnedSystemsService, get_owned_info::GetOwnedSystemInfoService, patch_owned::PatchOwnedSystemService}, tokens_pair::RefreshTokenGenerator, users::create_user::CreateUserService};
use axum::{Extension, Router};
use dotenv::dotenv;
use feature_tokens::{jwt::{self, JwtTokenGenerator}, TokenGenerator};
use serde::{de::DeserializeOwned, Serialize};
use tokio::net::TcpListener;

mod api;

fn get_env(name: &str) -> String {
    env::var(name).expect(&format!("failed to get {name} env var"))
}

fn load_jwt_from_env<Claims: Serialize + DeserializeOwned + Send + Sync + 'static>(key_env: &str, lifetime_env: &str) -> anyhow::Result<Arc<dyn TokenGenerator<Claims> + Send + Sync>> {
    Ok(Arc::new(JwtTokenGenerator::new(
        jwt::create_hs256_key(get_env(key_env).as_bytes()), 
        jwt::duration_from_secs(get_env(lifetime_env).parse()?)
    )))
}

pub async fn run() -> anyhow::Result<()> {
    dotenv()?;

    env_logger::init();

    let patg: Arc<dyn AccessTokenGenerator> = load_jwt_from_env("PANEL_ACCESS_TOKEN_KEY", "PANEL_ACCESS_TOKEN_LIFETIME_SECS")?;
    let prtg: Arc<dyn RefreshTokenGenerator> = load_jwt_from_env("PANEL_REFRESH_TOKEN_KEY", "PANEL_REFRESH_TOKEN_LIFETIME_SECS")?;

    let satg: Arc<dyn StatsAccessTokenGenerator> = load_jwt_from_env("STATS_ACCESS_TOKEN_KEY", "STATS_ACCESS_TOKEN_LIFETIME_SECS")?;
    let srtg: Arc<dyn RefreshTokenGenerator> = load_jwt_from_env("STATS_REFRESH_TOKEN_KEY", "STATS_REFRESH_TOKEN_LIFETIME_SECS")?;

    let address = get_env("BACKEND_ADDR");
    log::info!("starting server at address http://{address}");
    axum::serve(
        TcpListener::bind(address).await?, 
        Router::new()
            .nest("/api", api::router())
            .layer(Extension(Arc::new(SqlxDb::connect(&get_env("DATABASE_URL")).await?)))
            .layer(Extension(Arc::new(GetSessionInfoService {
                atg: patg.clone(),
            })))
            .layer(Extension(Arc::new(LoginService {
                atg: patg.clone(),
                rtg: prtg.clone(),
            })))
            .layer(Extension(Arc::new(RefreshSessionService {
                atg: patg.clone(),
                rtg: prtg.clone(),
            })))
            .layer(Extension(Arc::new(CreateUserService)))
            .layer(Extension(Arc::new(GetOwnedSystemsService)))
            .layer(Extension(Arc::new(CreateSystemService)))
            .layer(Extension(Arc::new(DeleteSystemService)))
            .layer(Extension(Arc::new(GetOwnedSystemInfoService)))
            .layer(Extension(Arc::new(PatchOwnedSystemService)))
            .layer(Extension(Arc::new(GetEventsOfOwnedService)))
            .layer(Extension(Arc::new(SystemEventCreateService)))
            .layer(Extension(Arc::new(SystemEventDeleteService)))
            .layer(Extension(Arc::new(GetClientsService)))
            .layer(Extension(Arc::new(CreateGroupService)))
            .layer(Extension(Arc::new(MergeClientsInGroupService)))
            .layer(Extension(Arc::new(DeleteClientService)))
            .layer(Extension(Arc::new(DeleteGroupService)))
            .layer(Extension(Arc::new(StatsAuthService {
                atg: satg.clone(),
                rtg: srtg.clone(),
            })))
            .layer(Extension(Arc::new(StatsFinishAuthService {
                atg: satg.clone(),
            })))
            .layer(Extension(Arc::new(StatsGetAuthInfoService)))
            .layer(Extension(Arc::new(StatsRefreshSessionService {
                atg: satg.clone(),
                rtg: srtg.clone(),
            })))
            .layer(Extension(Arc::new(StatsSendEventService)))
            .layer(Extension(Arc::new(StatsGetSessionInfoService {
                atg: satg.clone(),
            })))
    ).await?;
    Err(anyhow!("server suddenly stopped"))
}