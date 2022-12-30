use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateMessage {
    pub content: String,
    pub group: i32,
    pub user: i32,
}
