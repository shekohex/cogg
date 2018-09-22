use chrono::prelude::*;
use crate::state::ClientState;
use crate::util::Result;
use log::debug;
use protobuf::RepeatedField;
use protos::processes;
use protos::processes_grpc::WinProcessGuardClient;

pub struct ProcService<'a> {
    client: &'a WinProcessGuardClient,
}

impl<'a> ProcService<'a> {
    pub fn new(client: &'a WinProcessGuardClient) -> Self {
        ProcService { client }
    }

    pub fn send_snapshot(&self) -> Result<bool> {
        let snapshot = self.get_current_running_processes();
        let last_snapshot = RepeatedField::from_slice(&snapshot);
        let state = ClientState::get_state().unwrap();
        let current_user = state.current_user.clone().unwrap();
        let mut req = processes::WinProcessSnapShot::new();
        req.set_username(current_user.get_username().to_string());
        let now = Utc::now();
        let ts = now.timestamp_millis();
        req.set_snapshot_time(ts);
        req.set_last_snapshot(last_snapshot);
        //TODO: add pc owner name
        let res = self.client.process_snapshot(&req)?;
        debug!("Snapshot Response {:?}", res);
        Ok(true)
    }

    fn get_current_running_processes(&self) -> Vec<processes::WinProcess> {
        //TODO: Get actual running processes
        let (mut proc1, mut proc2) = (processes::WinProcess::new(), processes::WinProcess::new());
        proc1.set_module_name("chrome".to_string());
        proc2.set_module_name("notepad".to_string());
        vec![proc1, proc2]
    }
}
