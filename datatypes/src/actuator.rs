//! Actuator control types and configurations.

#[cfg(feature = "derive_mavlink")]
use data_core::mavlink;
use gated_derive::gated_derive;

/// The state of a valve that is normally open.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema)]
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum NormallyOpenValve {
    #[default]
    Open = 0,
    Closed = 1,
}

/// The state of a valve that is normally closed.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema)]
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum NormallyClosedValve {
    Open = 0,
    #[default]
    Closed = 1,
}

/// Pressurization Proportional Gain in [barg/ms]
/// opening_time = kp * (target_pressure - current_pressure), in milliseconds
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct PressurizationGain(pub f32);

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
// #[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum DprState {
    /// Enabled with pressure setpoint in Barg. 0-60.
    Enabled(f32),
    /// Disabled
    #[default]
    Disabled,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CameraPowerStatus {
    /// Whether the recording camera(s) should be powered on (true=on, false=off).
    pub recording_pwr_enabled: bool,
    /// Whether the livestream camera(s) should be powered on (true=on, false=off).
    pub live_pwr_enabled: bool,
}
