#![warn(rust_2018_idioms)]

pub(crate) mod files;
pub(crate) mod proc_service;
pub(crate) mod state;
pub(crate) mod users;
#[path = "../util.rs"]
pub(crate) mod util;
use colored::*;
use crate::files::Files;
use crate::proc_service::ProcService;
use crate::users::Users;
use crate::util::Result;
use failure::err_msg;
use grpcio::{ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};
use log::{error, info};
use protos::files_grpc::FilesGuardClient;
use protos::processes_grpc::WinProcessGuardClient;
use protos::users::User;
use protos::users_grpc::UsersClient;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    std::env::set_var("GG_LOGS", "ggclient,fshash");
    let mut builder = env_logger::Builder::from_env("GG_LOGS");
    builder.format(|buf, record| writeln!(buf, " {} -- {}", record.level(), record.args()));
    builder.init();
    info!("{}", "Starting Client..".green());
    let config = util::setup_config(Path::new("./config/config.toml"))?;
    let addr = format!("{}", config.server);
    let cert = include_str!("../../private/localhost.crt");
    let credentials = ChannelCredentialsBuilder::new()
        .root_cert(cert.into())
        .build();

    let mut state = state::ClientState::get_state().unwrap();
    let mut current_user = User::new();
    let username = std::env::args().nth(1).unwrap_or_default();
    current_user.set_username(username);
    state.add_current_user(current_user);

    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env).secure_connect(&addr, credentials);

    let files_guard_client = FilesGuardClient::new(channel.clone());
    let users_client = UsersClient::new(channel.clone());
    let proc_client = WinProcessGuardClient::new(channel.clone());

    let mut me = Users::new(&users_client);
    let files = Files::new(&files_guard_client);
    let proc_watcher = ProcService::new(&proc_client);

    let paths = files.get_files_paths()?;
    let verify_files_result = files.make_verify_files(&paths)?;

    if verify_files_result {
        // Fire MsgBox Here
        me.add_user()?;
        proc_watcher.send_snapshot()?;
        info!("{}", "All is well".green());
        let mut count = 0u8;
        let sleep_ms = Duration::from_millis(4000);
        loop {
            if count > 10 {
                break;
            }
            me.ping_server()?;
            count += 1;
            thread::sleep(sleep_ms);
        }
        Ok(())
    } else {
        error!("{}", "Ok, cheater".red());
        Err(err_msg("Error While verifying files"))
    }
}
