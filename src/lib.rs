#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    trivial_numeric_casts,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_results,
    trivial_casts,
    unsafe_op_in_unsafe_fn,
    unsafe_attr_outside_unsafe,
    clippy::must_use_candidate,
    clippy::doc_markdown,
    clippy::cast_possible_truncation,
    clippy::mut_mut,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::unwrap_used,
    clippy::unreadable_literal,
    clippy::ptr_as_ptr,
    clippy::cast_lossless,
    clippy::missing_panics_doc
)]

extern crate link_cplusplus;

#[macro_use]
mod ffi;

pub mod cpp;

pub mod graphics;
pub mod system;
pub mod window;
use std::{error::Error, fmt::Display};

/// An SFML operation has failed
#[derive(Clone, Copy, Debug)]
pub enum SfError {
    /// An string argument passed had interior nul bytes
    NulInStr,
    /// Call to SFML function returned an error
    CallFailed,
}

impl Display for SfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SfError::NulInStr => write!(f, "Interior nul byte in string passed to SFML"),
            SfError::CallFailed => write!(f, "Call to SFML function returned an error"),
        }
    }
}

impl From<std::ffi::NulError> for SfError {
    fn from(_value: std::ffi::NulError) -> Self {
        Self::NulInStr
    }
}

impl Error for SfError {}

/// Result of a fallible SFML operation
pub type SfResult<T> = Result<T, SfError>;

trait IntoSfResult<T> {
    fn into_sf_result(self) -> SfResult<T>;
}

impl IntoSfResult<()> for bool {
    fn into_sf_result(self) -> SfResult<()> {
        if self {
            Ok(())
        } else {
            Err(SfError::CallFailed)
        }
    }
}

impl<T: ?Sized> IntoSfResult<cpp::FBox<T>> for Option<cpp::FBox<T>> {
    fn into_sf_result(self) -> SfResult<cpp::FBox<T>> {
        self.ok_or(SfError::CallFailed)
    }
}
