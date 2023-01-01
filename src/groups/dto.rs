use serde::{Deserialize, Serialize};

use crate::messages::types::SerializableMessage;
use crate::users::types::SerializableUser;

#[derive(Deserialize)]
pub struct CreateGroup {
    pub is_duo: bool,
    pub name: String,
    pub created_by: i32,
}

#[derive(Deserialize)]
pub struct JoinGroup {
    pub user: i32,
}

#[derive(Serialize)]
pub struct PopulatedGroup {
    pub id: i32,
    pub name: String,
    pub is_duo: bool,
    pub created_by: SerializableUser,
    pub users: Vec<SerializableUser>,
    pub messages: Vec<SerializableMessage>,
}
