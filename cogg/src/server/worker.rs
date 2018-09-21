use chrono::prelude::*;
use colored::Colorize;
use crate::state::ServerState;
use log::{debug, error};
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Interval;

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
    /// Start the Service Worker
    pub fn work(&mut self) {
        if self.is_working || self.check_ervery == 0 {
            return;
        }
        let runner = |_instant| {
            let mut removed: Vec<String> = Vec::new();
            let mut state = ServerState::get_state().unwrap();
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
