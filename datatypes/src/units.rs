//! Physical measurement units and related types.

use gated_derive::gated_derive;

/// A temperature in degrees Celsius.
///
/// Is `NAN` by default.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Celsius(pub f32);

/// A pressure in bar relative to atmospheric pressure (`barg`).
///
/// Is `NAN` by default.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BarG(pub f32);

/// Atmospheric pressure in hPa.
///
/// Is `NAN` by default.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HPa(pub f32);

/// Shared rail status structure for both 5V and 24V power rails.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct RailStatus {
    /// Voltage in volts.
    pub voltage: f32,
    /// Current in amps.
    pub current: f32,
    /// Power in watts.
    pub power: f32,
}
