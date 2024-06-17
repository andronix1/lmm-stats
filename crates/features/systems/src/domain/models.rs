use uuid::Uuid;

pub struct CreateSystemInfo {
    pub name: String,
    pub human_name: String,
    pub owner: Uuid,
    pub secret: Option<String>
}

pub struct ShortSystemInfo {
    pub name: String,
    pub human_name: String,
}

pub struct FullSystemInfo {
    pub human_name: String,
    pub active: bool,
    pub owner_login: String,
    pub secret: Option<String>,
}

pub struct SystemPatch {
    pub human_name: Option<String>,
    pub active: Option<bool>,
    pub secret: Option<Option<String>>,
}