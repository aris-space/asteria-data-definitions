//! Physical measurement units and related types.

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// A temperature in degrees Celsius.
///
/// Is `NAN` by default.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Encode, Decode)]
pub struct Celsius(pub f32);

/// A pressure in bar relative to atmospheric pressure (`barg`).
///
/// Is `NAN` by default.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Encode, Decode)]
pub struct BarG(pub f32);

/// Atmospheric pressure in hPa.
///
/// Is `NAN` by default.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Encode, Decode)]
pub struct HPa(pub f32);

/// Shared rail status structure for both 5V and 24V power rails.
#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub struct RailStatus {
    /// Voltage in volts.
    pub voltage: f32,
    /// Current in amps.
    pub current: f32,
    /// Power in watts.
    pub power: f32,
}
