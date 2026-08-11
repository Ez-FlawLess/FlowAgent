use std::process::Stdio;

use tokio::process::{Child, ChildStdin, ChildStdout, Command};

use crate::{
    json_rpc::JsonRpc,
    schemes::init::{ClientInfo, InitReq, InitRes},
    shared::acp_protocl_version::AcpProtocolVersion,
};

mod json_rpc;
mod schemes;
mod shared;
mod utils;

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
compile_error!("this code only runs on Windows, Linux, and macOS");

pub struct Core {
    acp_process: Child,
    rpc: JsonRpc<ChildStdin, ChildStdout>,
}

impl Core {
    pub async fn run() {
        println!("spawning");
        let mut acp_process = Command::new("opencode")
            .arg("acp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        println!("spawened");

        let stdin = acp_process.stdin.take().unwrap();
        let stdout = acp_process.stdout.take().unwrap();

        let mut json_rpc = JsonRpc::new(stdin, stdout);

        let response: InitRes = json_rpc
            .send(InitReq {
                acp_protocol_version: AcpProtocolVersion::V1,
                client_info: ClientInfo {
                    name: "flowagent".to_string(),
                    title: Some("Flow Agent".to_string()),
                    version: "1.0.0".to_string(),
                },
            })
            .await
            .unwrap();

        println!("name: {}", response.agent_info.name);

        acp_process.kill().await.unwrap();
    }
}
