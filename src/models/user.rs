use serde::{Deserialize, Serialize};

use crate::dtos::users::UpdateUser;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub birth_date: String,
    pub gender: Gender,
}

impl User {
    pub fn update_with(&mut self, updates: UpdateUser) {
        if let Some(name) = updates.name {
            self.name = name;
        }
        if let Some(birth_date) = updates.birth_date {
            self.birth_date = birth_date;
        }
        if let Some(gender) = updates.gender {
            self.gender = gender;
        }
    }
}
