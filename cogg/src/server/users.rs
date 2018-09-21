use crate::state::ServerState;
use futures::Future;
use log::{debug, error, info};
use protos::users::{User, UserPing, UserResponse};
use protos::users_grpc::Users;

#[derive(Clone)]
pub struct UsersService;

impl Users for UsersService {
    fn add_user(
        &self,
        ctx: grpcio::RpcContext<'_>,
        req: User,
        sink: grpcio::UnarySink<UserResponse>,
    ) {
        let username = req.get_username();
        debug!("Got Username {}", username);
        let mut state = ServerState::get_state().unwrap();
        let ts = state.add_user(&req);
        let mut res = UserResponse::new();
        res.set_last_ping(ts);
        res.set_added(true);
        let f = sink
            .success(res)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }

    fn ping_user(
        &self,
        ctx: grpcio::RpcContext<'_>,
        req: UserPing,
        sink: grpcio::UnarySink<UserResponse>,
    ) {
        let username = req.get_username();
        info!("Got Ping form {}", username);
        let mut state = ServerState::get_state().unwrap();
        let ts = state.ping_user(username);
        let mut res = UserResponse::new();
        res.set_last_ping(ts);
        res.set_added(false);
        let f = sink
            .success(res)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}
