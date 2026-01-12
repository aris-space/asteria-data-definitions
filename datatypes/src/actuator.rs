//! Actuator control types and configurations.

use gated_derive::gated_derive;

/// The state of a valve that is normally open.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NormallyOpenValve {
    #[default]
    Open,
    Closed,
}

/// The state of a valve that is normally closed.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NormallyClosedValve {
    Open,
    #[default]
    Closed,
}

/// Pressurization Proportional Gain in [barg/ms]
/// opening_time = kp * (target_pressure - current_pressure), in milliseconds
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PressurizationGain(pub f32);

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Clone, Debug, PartialEq, Default)]
pub enum DprState {
    /// Enabled with pressure setpoint in Barg. 0-60.
    Enabled(f32),
    /// Disabled
    #[default]
    Disabled,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Clone, Debug, PartialEq)]
pub struct CameraPowerStatus {
    /// Whether the recording camera(s) should be powered on (true=on, false=off).
    pub recording_pwr_enabled: bool,
    /// Whether the livestream camera(s) should be powered on (true=on, false=off).
    pub live_pwr_enabled: bool,
}
