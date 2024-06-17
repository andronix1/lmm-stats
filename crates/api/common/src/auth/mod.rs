use serde::{Deserialize, Serialize};

pub mod login;
pub mod refresh_session;
pub mod get_session_info;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub enum UserRole {
    Superuser,
    Developer,
    Viewer
}

#[cfg(feature = "user-role-map")]
impl Into<feature_users::domain::UserRole> for UserRole {
    fn into(self) -> feature_users::domain::UserRole {
        match self {
            UserRole::Superuser => feature_users::domain::UserRole::Superuser,
            UserRole::Developer => feature_users::domain::UserRole::Developer,
            UserRole::Viewer => feature_users::domain::UserRole::Viewer,
        }
    }
}

#[cfg(feature = "user-role-map")]
impl From<feature_users::domain::UserRole> for UserRole {
    fn from(value: feature_users::domain::UserRole) -> Self {
        match value {
            feature_users::domain::UserRole::Superuser => UserRole::Superuser,
            feature_users::domain::UserRole::Developer => UserRole::Developer,
            feature_users::domain::UserRole::Viewer => UserRole::Viewer,
        }
    }
}