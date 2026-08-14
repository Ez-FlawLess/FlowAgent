pub mod created;
pub mod initialized;
pub mod session;

#[allow(private_bounds)]
pub trait State: sealed::Sealed {}

mod sealed {
    pub(super) trait Sealed {}
}
