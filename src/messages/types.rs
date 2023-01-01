use serde::Serialize;

#[derive(Serialize)]
pub struct SerializableMessage {
    pub id: i32,
    pub content: String,
    pub group: i32,
    pub user: i32,
}
