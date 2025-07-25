use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateAuth {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct CreatedAuth {
    pub token: String,
}
