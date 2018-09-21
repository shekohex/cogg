use crate::state::ServerState;
use futures::future::Future;
use grpcio::{RpcContext, RpcStatus, RpcStatusCode, UnarySink};
use log::{debug, error, warn};
use protobuf::RepeatedField;
use protos::processes;
use protos::processes_grpc::WinProcessGuard;

#[derive(Clone)]
pub struct ProtectorService {
    cheats: Vec<String>,
}

impl ProtectorService {
    pub fn new(cheats: Vec<String>, _allow_cloud: bool) -> Self {
        ProtectorService { cheats }
    }
}

impl WinProcessGuard for ProtectorService {
    fn process_snapshot(
        &self,
        ctx: RpcContext<'_>,
        req: processes::WinProcessSnapShot,
        sink: UnarySink<processes::SnapshotResponse>,
    ) {
        let (username, last_snapshot) = (req.get_username(), req.get_last_snapshot());
        let state = ServerState::get_state().unwrap();
        let mut res = processes::SnapshotResponse::new();
        if !state.users.contains_key(username) {
            warn!(
                "I think we have a bug: username {} should be here, but it isn't !",
                username
            );
            let f = sink
                .fail(RpcStatus::new(
                    RpcStatusCode::Unauthenticated,
                    Some("Username not found".to_string()),
                )).map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
            ctx.spawn(f)
        } else {
            let cheats = RepeatedField::from_vec(check_for_cheats(last_snapshot, &self.cheats));
            res.set_cheats(cheats);
            let f = sink
                .success(res)
                .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
            ctx.spawn(f)
        }
    }
}

fn check_for_cheats(
    current_processes: &[processes::WinProcess],
    known_cheats: &[String],
) -> Vec<processes::WinProcess> {
    current_processes
        .iter()
        .map(|p| {
            debug!("Got Process {:?}", p);
            p
        }).filter(|p| known_cheats.contains(&p.get_module_name().to_string()))
        .cloned()
        .collect()
}
