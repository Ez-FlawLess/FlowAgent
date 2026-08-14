use std::process::Stdio;

use thiserror::Error;
use tokio::{
    io::{self},
    process::{Child, ChildStdin, ChildStdout, Command},
};

use crate::acp_agent::AcpAgent;

pub struct Opencode {
    process: Child,
    stdin: ChildStdin,
    stdout: ChildStdout,
}

impl Opencode {
    pub fn new() -> Result<Self, NewOpencodeErr> {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let program = "opencode";

        #[cfg(target_os = "windows")]
        let program = "opencode.cmd";

        let mut process = Command::new(program)
            .arg("acp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(NewOpencodeErr::SpawnProc)?;

        let stdin = process.stdin.take().ok_or(NewOpencodeErr::Internal)?;
        let stdout = process.stdout.take().ok_or(NewOpencodeErr::Internal)?;

        Ok(Self {
            process,
            stdin,
            stdout,
        })
    }
}

#[derive(Debug, Error)]
pub enum NewOpencodeErr {
    #[error("error spawning opencode agent process: {0}")]
    SpawnProc(io::Error),
    #[error("there was an internal error")]
    Internal,
}

impl AcpAgent for Opencode {
    type Process = Child;
    type Reader = ChildStdout;
    type Writer = ChildStdin;

    fn name(&self) -> &str {
        "Opencode"
    }

    fn into_parts(self) -> (Self::Reader, Self::Writer, Option<Self::Process>) {
        (self.stdout, self.stdin, Some(self.process))
    }
}
