use crate::state::ClientState;
use crate::util::Result;
use failure::err_msg;
use log::{error, info};
use protos::users::{User, UserPing};
use protos::users_grpc::UsersClient;

pub struct Users<'a> {
    client: &'a UsersClient,
    current_user: User,
    user_added: bool,
}

impl<'a> Users<'a> {
    pub fn new(client: &'a UsersClient) -> Self {
        let state = ClientState::get_state().unwrap();
        // we have to clone the current user data, so it is not that big tho.
        let current_user = state.current_user.clone();
        Users {
            client,
            current_user: current_user.unwrap(),
            user_added: false,
        }
    }
    pub fn add_user(&mut self) -> Result<()> {
        if self.user_added {
            Ok(())
        } else {
            let res = self.client.add_user(&self.current_user)?;
            if res.get_added() {
                info!("Got Res: {:?}", res);
                self.user_added = true;
                Ok(())
            } else {
                Err(err_msg("Error While Adding user to the server."))
            }
        }
    }

    pub fn ping_server(&self) -> Result<()> {
        if !self.user_added {
            Err(err_msg("You need to set the user first"))
        } else {
            let mut req = UserPing::new();
            req.set_username(self.current_user.get_username().to_string());
            let res = self.client.ping_user(&req)?;
            let last_ping = res.get_last_ping();
            if last_ping == 0 {
                let error_msg =
                    "Error While Ping User, User dose not exist in the server, maybe kicked out?";
                error!("{}", error_msg);
                Err(err_msg(error_msg))
            } else {
                info!("Got Res: {}", last_ping);
                Ok(())
            }
        }
    }
}
