use uuid::Uuid;

pub struct ClientInfo {
    pub id: Uuid,
    pub name: String
}

pub struct FullGroup {
    pub id: Uuid,
    pub name: String,
    pub clients: Vec<ClientInfo>
}

pub struct GroupInfo {
    pub id: Uuid,
    pub name: String,
}