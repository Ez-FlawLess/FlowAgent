pub mod connected;
pub mod initialized;

#[allow(private_bounds)]
pub trait State: sealed::Sealed {}

mod sealed {
    pub(super) trait Sealed {}
}
