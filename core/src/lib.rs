#![cfg_attr(not(feature = "std"), no_std)]
//! This crate should be used for using the macros from the `data_macros` crate.
//! The reason for this is, that this crate reexports all crates needed by the macros
//! (unless specified otherwhise by the macro itself)

pub use data_macros::*;

#[cfg(feature = "can")]
pub use can_hal;

#[cfg(feature = "mavlink")]
pub use mavlink_hal;
#[cfg(feature = "mavlink")]
pub use mavio;

#[cfg(feature = "bincode")]
pub use bincode;
