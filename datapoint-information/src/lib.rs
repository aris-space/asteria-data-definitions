//! This crate exposes traits to gain additional metadata about messages,
//! mostly for sensors and valves to support dynamic UI generation.

pub mod datapoint;
pub mod macro_util;

#[cfg(feature = "csv")]
pub mod influx;
