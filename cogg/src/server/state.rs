use chrono::prelude::*;
use colored::Colorize;
use lazy_static::lazy_static;
use log::{debug, error};
use protos::users::User;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Interval;
lazy_static! {
    pub(crate) static ref SERVER_STATE: Mutex<ServerState> = Mutex::new(ServerState::new());
}

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
        if self.users.contains_key(username) {
            self.users.insert(username.to_string(), ts);
            ts
        } else {
            0
        }
    }
}

pub(crate) struct ServiceWorker {
    check_ervery: u64,
    is_working: bool,
}

impl ServiceWorker {
    pub fn new(check_ervery: u64) -> Self {
        ServiceWorker {
            check_ervery,
            is_working: false,
        }
    }

    pub fn work(&mut self) {
        if self.is_working || self.check_ervery == 0 {
            return;
        }
        let runner = |_instant| {
            let mut removed: Vec<String> = Vec::new();
            let lock = SERVER_STATE.try_lock();
            if let Ok(mut state) = lock {
                let now = Utc::now();
                let now_ts = now.timestamp_millis();
                state.users.retain(|username, &mut ts| {
                    let last_ping = Duration::from_millis(ts as u64);
                    let now_ms = Duration::from_millis(now_ts as u64);
                    let diff = now_ms - last_ping;
                    if diff.as_secs() > 5 {
                        removed.push(username.to_string());
                        return false;
                    }
                    true
                });
                if !removed.is_empty() {
                    debug!("Removed {} Users: {:?}", removed.len(), removed);
                }
                debug!("We Have {} Users Connected", state.users.len());
            } else {
                error!("Error While trying to get server state, it may block!");
            }
            Ok(())
        };
        let task = Interval::new(Instant::now(), Duration::from_millis(self.check_ervery))
            .for_each(runner)
            .map_err(|e| error!("Failed While Running Worker; err={:?}", e));
        self.is_working = true;
        debug!("{}", "Worker Started OK!".green());
        tokio::run(task);
    }
}
