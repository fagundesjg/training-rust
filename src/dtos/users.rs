use serde::Deserialize;

use crate::models::user::Gender;

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub birth_date: String,
    pub gender: Gender,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub birth_date: Option<String>,
    pub gender: Option<Gender>,
}
