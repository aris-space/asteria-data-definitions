//! This crate holds common datatypes to be used in datapoints.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod actuator;
#[cfg(feature = "derive_datapoint_information")]
pub mod datapoint_information;
pub mod status;
pub mod units;
