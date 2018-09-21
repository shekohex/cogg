use crate::util::Result;
use failure::err_msg;
use lazy_static::lazy_static;
use log::error;
use protos::users::User;
use std::sync::Mutex;
use std::sync::MutexGuard;

lazy_static! {
    static ref CLIENT_STATE: Mutex<ClientState> = Mutex::new(ClientState::new());
}

#[derive(Debug)]
pub(crate) struct ClientState {
    pub(crate) current_user: Option<User>,
    last_ping: i64,
}

impl ClientState {
    pub fn new() -> Self {
        ClientState {
            current_user: None,
            last_ping: 0,
        }
    }

    pub fn add_current_user(current_user: User) {
        Self::get_state().unwrap().current_user = Some(current_user);
    }

    pub fn get_state<'a>() -> Result<MutexGuard<'a, Self>> {
        let lock = CLIENT_STATE.try_lock();
        if let Ok(state) = lock {
            Ok(state)
        } else {
            let err = "Error While trying to get client state, it may block!";
            error!("{}", err);
            Err(err_msg(err))
        }
    }
}
