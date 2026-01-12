//! Actuator control types and configurations.

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// The state of a valve that is normally open.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub enum NormallyOpenValve {
    #[default]
    Open,
    Closed,
}

/// The state of a valve that is normally closed.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub enum NormallyClosedValve {
    Open,
    #[default]
    Closed,
}

/// Pressurization Proportional Gain in [barg/ms]
/// opening_time = kp * (target_pressure - current_pressure), in milliseconds
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Encode, Decode)]
pub struct PressurizationGain(pub f32);

#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub enum DprState {
    /// Enabled with pressure setpoint in Barg. 0-60.
    Enabled(f32),
    /// Disabled
    #[default]
    Disabled,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct CameraPowerStatus {
    /// Whether the recording camera(s) should be powered on (true=on, false=off).
    pub recording_pwr_enabled: bool,
    /// Whether the livestream camera(s) should be powered on (true=on, false=off).
    pub live_pwr_enabled: bool,
}