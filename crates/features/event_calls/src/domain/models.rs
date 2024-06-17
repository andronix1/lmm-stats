use uuid::Uuid;

pub struct CreateEventCallInfo {
    pub system: String,
    pub event: String,
    pub from_client: Uuid
}