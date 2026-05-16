//! This crate provides the traits and helpers to derive [`mavio`] compatible dialects based on the datapoint enums.
//!
//! The macros are in the `macros` crate, see their docs there on how to use this.
//! Additionally provides a UDP transport helper.

pub mod codec;
pub mod description;
pub mod macro_util;
pub mod transport;
pub mod typing;
