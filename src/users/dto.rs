use serde::Deserialize;
use serde_with::skip_serializing_none;

#[derive(Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[skip_serializing_none]
#[derive(Deserialize)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub password: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}
