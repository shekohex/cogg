use chrono::prelude::*;
use log::debug;
use protos::users::User;
use std::collections::HashMap;
#[derive(Debug)]
pub(crate) struct ServerState {
    /// Holds Online Users
    users: HashMap<String, i64>,
    last_mutate: i64,
}

impl ServerState {
    pub fn new() -> Self {
        let now = Utc::now();
        ServerState {
            users: HashMap::new(),
            last_mutate: now.timestamp_millis(),
        }
    }

    pub fn add_user(&mut self, user: &User) -> i64 {
        let now = Utc::now();
        let ts = now.timestamp_millis();
        let username = user.get_username().to_owned();
        self.users.insert(username, ts);
        debug!("Added User {} to database, at {}", user.get_username(), ts);
        self.last_mutate = ts;
        ts
    }

    pub fn ping_user(&mut self, username: &str) -> i64 {
        let now = Utc::now();
        let ts = now.timestamp_millis();
        if self.users.get(username).is_some() {
            self.users.insert(username.to_string(), ts);
            return ts;
        } else {
            return 0;
        }
    }
}
