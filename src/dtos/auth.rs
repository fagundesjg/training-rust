use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAuth {
    pub email: String,
    pub password: String,
}
