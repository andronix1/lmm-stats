use async_trait::async_trait;
use abstract_sqlx_bindings::SqlxTransaction;
use uuid::Uuid;

use crate::domain::{Credentials, UserAccessInfo, UserAuthInfo, UserCreateInfo, UserRole, UsersRepo};

#[async_trait]
impl UsersRepo for SqlxTransaction<'_> {
    async fn login_exists(&mut self, login: String) -> anyhow::Result<bool> {
        Ok(sqlx::query!("select count(*) from users where login = $1 limit 1", login)
            .fetch_one(self.as_conn())
            .await?
            .count == Some(1))
    }
    
    async fn create(&mut self, info: UserCreateInfo) -> anyhow::Result<()> {
        sqlx::query!("insert into users (login, encrypted_password, role) values ($1, crypt($2, gen_salt('md5')), $3)", info.login, info.password, info.role as UserRole)
            .execute(self.as_conn())
            .await?;
        Ok(())
    }
    
    async fn try_get_auth_info(&mut self, credentials: Credentials) -> anyhow::Result<Option<UserAuthInfo>> {
        Ok(sqlx::query_as!(UserAuthInfo, "select id, revision from users where encrypted_password = crypt($2, encrypted_password) and login = $1", credentials.login, credentials.password)
            .fetch_optional(self.as_conn())
            .await?)
    }

    async fn try_get_access_info(&mut self, id: Uuid) -> anyhow::Result<Option<UserAccessInfo>> {
        Ok(sqlx::query_as!(UserAccessInfo, "select role as \"role: UserRole\", revision from users where id = $1", id)
            .fetch_optional(self.as_conn())
            .await?)
    }

    async fn try_get_revision(&mut self, id: Uuid) -> anyhow::Result<Option<i32>> {
        Ok(sqlx::query!("select revision from users where id = $1", id)
            .fetch_optional(self.as_conn())
            .await?.map(|dto| dto.revision))
    }
}