use serde::Serialize;

pub mod init;

pub trait Request
where
    Self: Serialize,
{
    fn method() -> &'static str;
}
