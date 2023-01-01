use serde::Serialize;

#[derive(Serialize)]
pub struct SerializableGroup {
    pub id: i32,
    pub is_duo: bool,
    pub created_by: i32,
}
