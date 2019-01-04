#![cfg(feature = "nightly")]

use std::fmt::Debug;

pub trait MaybeDebug {
    fn maybe_debug(&self) -> Option<&Debug>;
}

impl<T> MaybeDebug for T where T: ?Sized {
    default fn maybe_debug(&self) -> Option<&Debug> {
        None
    }
}

impl<T> MaybeDebug for T
where
    T: Debug,
{
    fn maybe_debug(&self) -> Option<&Debug> {
        Some(self as &Debug)
    }
}
