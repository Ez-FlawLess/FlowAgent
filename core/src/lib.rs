use tokio::process::{Child, ChildStdin, ChildStdout};

use crate::{json_rpc::JsonRpc, states::State};

mod json_rpc;
mod schemes;
mod shared;
pub mod states;
mod utils;

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
compile_error!("this code only runs on Windows, Linux, and macOS");

pub struct Core<S: State> {
    acp_process: Child,
    rpc: JsonRpc<ChildStdin, ChildStdout>,
    state: S,
}
