use std::process::Stdio;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, ChildStdin, Command},
};

use crate::{
    json_rpc::JsonRpc,
    requests::init::{ClientInfo, InitRequest},
    shared::acp_protocl_version::AcpProtocolVersion,
};

mod json_rpc;
mod requests;
mod shared;

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
compile_error!("this code only runs on Windows, Linux, and macOS");

pub struct Core {
    acp_process: Child,
    rpc: JsonRpc<ChildStdin>,
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

        let mut json_rpc = JsonRpc::new(stdin);

        json_rpc
            .send_request(InitRequest {
                acp_protocol_version: AcpProtocolVersion::V1,
                client_info: ClientInfo {
                    name: "flowagent".to_string(),
                    title: "Flow Agent".to_string(),
                    version: "1.0.0".to_string(),
                },
            })
            .await;

        println!("waiting for response");

        let mut reader = BufReader::new(stdout);
        let mut response = String::new();
        reader.read_line(&mut response).await.unwrap();

        println!("{response}");

        acp_process.kill().await.unwrap();
    }
}
