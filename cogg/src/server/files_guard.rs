use crate::state::ServerState;
use crate::util::ArcHashMap;
use futures::{sink::Sink, stream, Future};
use grpcio::{RpcContext, RpcStatus, RpcStatusCode, ServerStreamingSink, WriteFlags};
use log::{debug, error, warn};
use protos::files::{File, FileCollection, FileStatus, FilesPaths, FilesStatus, Void};
use protos::files_grpc::FilesGuard;

#[derive(Clone)]
pub struct FilesGuardService {
    hashes: ArcHashMap<String, String>,
}

impl FilesGuardService {
    pub fn new(hashes: ArcHashMap<String, String>) -> Self {
        FilesGuardService { hashes }
    }
}

impl FilesGuard for FilesGuardService {
    fn get_files_paths(
        &self,
        ctx: RpcContext<'_>,
        _req: Void,
        sink: ServerStreamingSink<FilesPaths>,
    ) {
        let file_paths = self.hashes.clone();
        let futures: Vec<_> = file_paths
            .keys()
            .map(move |path| {
                let mut file_paths = FilesPaths::new();
                file_paths.set_path(path.to_owned());
                (file_paths, WriteFlags::default())
            }).collect();
        // let peer: String = ctx.peer().split(':').skip(1).collect();
        debug!("Peer: {}", ctx.peer());
        let f = sink
            .send_all(stream::iter_ok::<_, grpcio::Error>(futures))
            .map(|_| ())
            .map_err(|e| error!("failed to handle get_files_paths request: {:?}", e));
        ctx.spawn(f)
    }

    fn verify_files(
        &self,
        ctx: RpcContext<'_>,
        req: FileCollection,
        sink: ServerStreamingSink<FilesStatus>,
    ) {
        let hashes = self.hashes.clone();
        let state = ServerState::get_state().unwrap();
        let files = req.get_files();
        let mut username = "".to_string();
        for (key, value) in ctx.request_headers() {
            if key == "username" {
                username = String::from_utf8(value.to_vec()).unwrap_or_default();
                debug!("We got the username: {} for checking files", username);
            }
        }
        if !state.users.contains_key(&username) {
            warn!(
                "I think we have a bug: username {} should be here, but it isn't !",
                username
            );
            let f = sink
                .fail(RpcStatus::new(
                    RpcStatusCode::Unauthenticated,
                    Some("Username not found".to_string()),
                )).map_err(move |e| error!("failed to reply verify_files: {:?}", e));
            ctx.spawn(f)
        } else {
            let futures: Vec<_> = files
                .iter()
                .map(move |file| {
                    let mut res = FilesStatus::new();

                    let (path, hash) = (file.get_path(), file.get_hash());
                    let mut file = File::new();
                    file.set_path(path.to_owned());
                    file.set_hash(hash.to_owned());
                    res.set_file(file);
                    if let Some(server_hash) = hashes.get(path) {
                        if hash == server_hash {
                            res.set_status(FileStatus::OK);
                        } else {
                            res.set_status(FileStatus::HASH_MISMATCH);
                        }
                    } else {
                        res.set_status(FileStatus::NOT_FOUND);
                    }

                    (res, WriteFlags::default())
                }).collect();
            let f = sink
                .send_all(stream::iter_ok::<_, grpcio::Error>(futures))
                .map(|_| ())
                .map_err(|e| error!("failed to handle verify_files request: {:?}", e));
            ctx.spawn(f)
        }
    }
}
