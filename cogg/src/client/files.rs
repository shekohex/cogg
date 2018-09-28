use crate::state::ClientState;
use crate::util::Result;
use failure::err_msg;
use futures::future::Future;
use futures::stream::Stream;
use log::{error, info};
use protobuf::RepeatedField;
use protos::files::{File, FileCollection, FileStatus, Void};
use protos::files_grpc::FilesGuardClient;
pub struct Files<'a> {
    client: &'a FilesGuardClient,
}
impl<'a> Files<'a> {
    pub fn new(client: &'a FilesGuardClient) -> Self {
        Files { client }
    }

    pub fn get_files_paths(&self) -> Result<Vec<String>> {
        let req = Void::new();
        let mut reply = self.client.get_files_paths(&req)?;
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

    pub fn make_verify_files(&self, paths: &[String]) -> Result<bool> {
        let mut req = FileCollection::new();
        let state = ClientState::get_state().unwrap();
        let current_user = state.current_user.clone().unwrap();
        let username = current_user.get_username();
        let mut builder = grpcio::MetadataBuilder::with_capacity(1);
        builder.add_str("username", username)?;
        let metadata = builder.build();
        let call_opts = grpcio::CallOption::default().headers(metadata);
        let files: Vec<File> = paths
            .iter()
            .map(move |path| {
                let mut file = File::new();
                file.set_path(path.to_owned());
                let hash = fshash::get_hash_from(&path).unwrap_or_else(|_| String::new());
                file.set_hash(hash);
                file
            }).collect();

        let files = RepeatedField::from_vec(files);
        req.set_files(files);
        let mut reply = self.client.verify_files_opt(&req, call_opts)?;

        let result = loop {
            let f = reply.into_future();
            match f.wait() {
                Ok((Some(files_status), s)) => {
                    reply = s;
                    if !self.verify_status(files_status.get_status()) {
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

    fn verify_status(&self, status: FileStatus) -> bool {
        match status {
            FileStatus::OK => true,
            _ => false,
        }
    }
}
