use tokio::io::{AsyncRead, AsyncWrite};

pub mod opencode;

pub trait AcpAgent {
    type Reader: AsyncRead + Unpin + Send + 'static;
    type Writer: AsyncWrite + Unpin;
    type Process;

    fn name(&self) -> &str;

    /// Consumes the agent, returning its writer, reader, and optional process handle.
    fn into_parts(self) -> (Self::Reader, Self::Writer, Option<Self::Process>);
}
