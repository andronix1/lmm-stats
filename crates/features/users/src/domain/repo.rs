use async_trait::async_trait;
use uuid::Uuid;

use super::{Credentials, UserAccessInfo, UserAuthInfo, UserCreateInfo};

#[async_trait]
pub trait UsersRepo: Send + Sync {
    async fn login_exists(&mut self, login: String) -> anyhow::Result<bool>;
    async fn create(&mut self, info: UserCreateInfo) -> anyhow::Result<()>;
    
    async fn try_get_auth_info(&mut self, credentials: Credentials) -> anyhow::Result<Option<UserAuthInfo>>;

    async fn try_get_access_info(&mut self, id: Uuid) -> anyhow::Result<Option<UserAccessInfo>>;
    async fn try_get_revision(&mut self, id: Uuid) -> anyhow::Result<Option<i32>>;
}