use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub email: String,
    pub password: String,
}
