use chrono::prelude::*;
use crate::util::Result;
use failure::err_msg;
use lazy_static::lazy_static;
use log::{debug, error};
use protos::users::User;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::MutexGuard;

lazy_static! {
    static ref SERVER_STATE: Mutex<ServerState> = Mutex::new(ServerState::new());
}

#[derive(Debug)]
pub(crate) struct ServerState {
    /// Holds Online Users
    pub(crate) users: HashMap<String, i64>,
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

    pub fn get_state<'a>() -> Result<MutexGuard<'a, Self>> {
        let lock = SERVER_STATE.try_lock();
        if let Ok(state) = lock {
            Ok(state)
        } else {
            let err = "Error While trying to get server state, it may block!";
            error!("{}", err);
            Err(err_msg(err))
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

    pub fn kick_user(&mut self, username: &str) -> Result<bool> {
        if let Some(ts) = self.users.remove(username) {
            debug!(
                "Removed User {} from database at {} as found some running cheats",
                username, ts
            );
            Ok(true)
        } else {
            error!("Error While Trying to remove that user {}, as it is not found", username);
            Ok(false)
        }
    }

    pub fn ping_user(&mut self, username: &str) -> i64 {
        let now = Utc::now();
        let ts = now.timestamp_millis();
        if self.users.contains_key(username) {
            self.users.insert(username.to_string(), ts);
            ts
        } else {
            0
        }
    }
}
