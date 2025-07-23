use crate::models::user::User;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub user: UserState,
}

#[derive(Clone)]
pub struct UserState {
    pub users: Arc<Mutex<Vec<User>>>,
    pub counter: Arc<Mutex<u64>>,
}
