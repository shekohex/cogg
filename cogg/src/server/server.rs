#![warn(rust_2018_idioms)]

pub(crate) mod files_guard;
pub(crate) mod state;
pub(crate) mod users;
#[path = "../util.rs"]
pub(crate) mod util;
use colored::*;
use crate::files_guard::FilesGuardService;
use crate::util::Result;
use futures::{sync::oneshot, Future};
use grpcio::{Environment, ServerBuilder, ServerCredentialsBuilder};
use log::info;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use std::{io, thread};

fn main() -> Result<()> {
    std::env::set_var("GG_LOGS", "ggserver");
    let mut builder = env_logger::Builder::from_env("GG_LOGS");
    builder.format(|buf, record| writeln!(buf, " {} -- {}", record.level(), record.args()));
    builder.init();
    info!("{}", "Starting Server..".green());

    let config = util::setup_config(Path::new("./config/config.toml"))?;
    let hashes = util::calculate_hashes(config.files.paths)?;
    let env = Arc::new(Environment::new(2));

    let private_key = include_str!("../../private/localhost.key");
    let cert = include_str!("../../private/localhost.crt");

    let credentials = ServerCredentialsBuilder::new()
        .add_cert(cert.into(), private_key.into())
        .build();

    let files_guard = FilesGuardService::new(hashes);
    let files_guard_service = protos::files_grpc::create_files_guard(files_guard);
    let users_service = protos::users_grpc::create_users(users::UsersService);
    let mut server = ServerBuilder::new(env)
        .register_service(files_guard_service)
        .register_service(users_service)
        .bind_secure(config.server.ip, config.server.port.parse()?, credentials)
        .build()?;

    server.start();
    for &(ref host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("{}", "Press ENTER to exit...".blue());
        let _ = io::stdin().read(&mut [0u8]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
    Ok(())
}
