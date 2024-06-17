use uuid::Uuid;

// isn't good idea but... COME ON
#[derive(Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")] 
pub enum UserRole {
    Superuser,
    Developer,
    Viewer
}

pub struct UserCreateInfo {
    pub login: String,
    pub password: String,
    pub role: UserRole,
}

pub struct Credentials {
    pub login: String,
    pub password: String,
}

pub struct UserAccessInfo {
    pub role: UserRole,
    pub revision: i32
}

pub struct UserAuthInfo {
    pub id: Uuid,
    pub revision: i32
}