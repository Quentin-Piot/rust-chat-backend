use serde::Serialize;

#[derive(Serialize)]
pub struct SerializableUser {
    pub id: i32,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
}
