#![feature(rust_2018_preview, use_extern_macros)]
#![warn(rust_2018_idioms)]

#[path = "../util.rs"]
pub(crate) mod util;
use colored::*;
use crate::util::Result;
use failure::err_msg;
use futures::future::Future;
use futures::stream::Stream;
use grpcio::{ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};
use log::{error, info, log};
use protobuf::RepeatedField;
use protos::main::{File, FileCollection, FileStatus, Void};
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
    let cert = include_str!("../../private/server.crt");
    let credentials = ChannelCredentialsBuilder::new()
        .root_cert(cert.into())
        .build();

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).secure_connect(&addr, credentials);
    let client = FilesGuardClient::new(ch);
    let files = get_files_paths(&client)?;
    let verify_files_result = make_verify_files(&client, files)?;
    if verify_files_result {
        // Fire MsgBox Here
        info!("{}", "All is well".green());
        Ok(())
    } else {
        error!("{}", "Ok, cheater".red());
        Err(err_msg("Error While verifying files"))
    }
}

fn get_files_paths(client: &FilesGuardClient) -> Result<Vec<String>> {
    let req = Void::new();
    let mut reply = client.get_files_paths(&req)?;
    let mut paths = Vec::new();
    let ok = loop {
        let f = reply.into_future();
        match f.wait() {
            Ok((Some(file), s)) => {
                reply = s;
                let path = file.get_path();
                info!("Found Path at {}", path);
                paths.push(path.to_owned());
            }
            Ok((None, _)) => {
                info!("List Paths rpc succeeded.");
                break true;
            }
            Err((e, _)) => {
                error!("List Paths failed: {:?}", e);
                break false;
            }
        }
    };
    if ok {
        Ok(paths)
    } else {
        Err(err_msg("Error While Listing Paths"))
    }
}

fn make_verify_files(client: &FilesGuardClient, paths: Vec<String>) -> Result<bool> {
    let mut req = FileCollection::new();
    let files: Vec<File> = paths
        .iter()
        .map(move |path| {
            let mut file = File::new();
            file.set_path(path.to_owned());
            let hash = fshash::get_hash_from(path.to_owned()).unwrap_or_else(|_| String::new());
            file.set_hash(hash);
            file
        }).collect();

    let files = RepeatedField::from_vec(files);
    req.set_files(files);
    let mut reply = client.verify_files(&req)?;
    let result = loop {
        let f = reply.into_future();
        match f.wait() {
            Ok((Some(files_status), s)) => {
                reply = s;
                if !verify_status(files_status.get_status()) {
                    error!("Hash MisMatch or maybe file not found");
                    break false;
                }
            }
            Ok((None, _)) => {
                info!("Verifying Files rpc succeeded.");
                break true;
            }
            Err((e, _)) => {
                error!("Verifying Files failed: {:?}", e);
                break false;
            }
        }
    };
    Ok(result)
}

fn verify_status(status: FileStatus) -> bool {
    match status {
        FileStatus::OK => true,
        _ => false,
    }
}
