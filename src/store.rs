use crate::models::{Tweet, User};
use std::sync::{Arc, Mutex};

pub type Db = Arc<Mutex<AppState>>;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<User>>>,
    pub tweets: Arc<Mutex<Vec<Tweet>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Vec::new())),
            tweets: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
