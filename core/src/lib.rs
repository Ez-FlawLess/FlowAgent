use crate::{acp_agent::AcpAgent, json_rpc::JsonRpc, states::State};

pub mod acp_agent;
mod json_rpc;
mod schemes;
mod shared;
pub mod states;
mod utils;

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
compile_error!("this code only runs on Windows, Linux, and macOS");

pub struct Core<A: AcpAgent, S: State> {
    acp_process: Option<A::Process>,
    rpc: JsonRpc<A::Writer, A::Reader>,
    state: S,
}
