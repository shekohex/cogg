#![warn(rust_2018_idioms)]

pub(crate) mod guard_client;
#[path = "../util.rs"]
pub(crate) mod util;
use colored::*;
use crate::util::Result;
use failure::err_msg;
use grpcio::{ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};
use log::{error, info, log};
use protos::main_grpc::FilesGuardClient;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

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
    // Bind the server's socket
    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env).secure_connect(&addr, credentials);
    let files_guard_client = FilesGuardClient::new(channel);

    let files = guard_client::get_files_paths(&files_guard_client)?;
    let verify_files_result = guard_client::make_verify_files(&files_guard_client, files)?;
    if verify_files_result {
        // Fire MsgBox Here
        info!("{}", "All is well".green());
        Ok(())
    } else {
        error!("{}", "Ok, cheater".red());
        Err(err_msg("Error While verifying files"))
    }
}
