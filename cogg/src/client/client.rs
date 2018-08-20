#![feature(rust_2018_preview, use_extern_macros)]
#![warn(rust_2018_idioms)]

pub(crate) mod guard_client;
#[path = "../util.rs"]
pub(crate) mod util;
use colored::*;
use crate::util::Result;
use failure::err_msg;
use futures::stream::Stream;
use grpcio::{ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};
use log::{debug, error, info, log};
use protos::main_grpc::FilesGuardClient;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() -> Result<()> {
    std::env::set_var("GG_LOGS", "ggclient,fshash");
    let mut builder = env_logger::Builder::from_env("GG_LOGS");
    builder.format(|buf, record| writeln!(buf, " {} -- {}", record.level(), record.args()));
    builder.init();
    info!("{}", "Starting Client..".green());
    let config = util::setup_config(Path::new("./config/config.toml"))?;
    let addr = format!("{}", config.server);
    let cert = include_str!("../../private/server.crt");
    let credentials = ChannelCredentialsBuilder::new()
        .root_cert(cert.into())
        .build();
    // Bind the server's socket
    let tokio_addr = "127.0.0.1:12345".parse().unwrap();
    let tcp = TcpListener::bind(&tokio_addr).unwrap();

    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env).secure_connect(&addr, credentials);
    let files_guard_client = FilesGuardClient::new(channel);

    let files = guard_client::get_files_paths(&files_guard_client)?;
    let verify_files_result = guard_client::make_verify_files(&files_guard_client, files)?;
    if verify_files_result {
        // Fire MsgBox Here
        info!("{}", "All is well".green());
        // Iterate incoming connections
        let server = tcp
            .incoming()
            .for_each(|socket| {
                info!("accepted socket; addr={:?}", socket.peer_addr().unwrap());
                let buf = vec![0u8; 64];
                let reader = io::read_exact(socket, buf)
                    .map(|(_, vec)| {
                        let packet = std::str::from_utf8(&vec).unwrap_or_else(|_| ".");
                        let v: Vec<&str> = packet.split('.').collect();
                        let username = v.first().unwrap_or_else(|| &"");
                        debug!("Username: {:?}", username);
                    }).then(|_| Ok(()));
                tokio::spawn(reader);
                Ok(())
            }).map_err(|err| {
                error!("server error {:?}", err);
            });

        info!("Tokio is running on {}", tokio_addr);
        // Start the runtime and spin up the server
        tokio::run(server);
        Ok(())
    } else {
        error!("{}", "Ok, cheater".red());
        Err(err_msg("Error While verifying files"))
    }
}
