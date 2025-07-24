use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub birth_date: String,
    pub gender: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub birth_date: Option<String>,
    pub gender: Option<String>,
    pub password: Option<String>,
}
