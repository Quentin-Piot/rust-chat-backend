use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateGroup {
    pub is_duo: bool,
    pub created_by: i32,
}
