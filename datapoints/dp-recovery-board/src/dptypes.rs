#[cfg(feature = "mavlink")]
use data_core::mavlink;
use datatypes::status::{ArmingState, StatusCommonMessage};
use gated_derive::gated_derive;

/// Represents the current state of an actuator as detected by a board.
///
/// ## Notes
/// Detection of actuator presence is only possible before power is applied to the actuator.
/// Once powered, actuator detection is no longer possible.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum ActuatorStatus {
    /// Actuator is not connected (e.g. unplugged or circuit open).
    #[default]
    NotConnected = 0,
    /// Actuator is connected but not powered.
    Connected = 1,
    /// Actuator is powered; detection circuit is no longer active.
    PowerOn = 2,
}

/// Represents the state of one steering motor.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum SteeringStatus {
    /// Steering power is off, so the motor cannot be reached.
    #[default]
    Unpowered = 0,
    /// Steering is powered, but the motor did not answer its last status request.
    Unresponsive = 1,
    /// The motor answered its last status request.
    Responsive = 2,
}

/// Represents the current state of a watchdog timer.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum WatchdogState {
    /// Watchdog is not active. No steering signal has been received since startup.
    #[default]
    Disabled = 0,
    /// Watchdog is actively monitoring and receiving valid steering updates.
    Active = 1,
    /// Watchdog has expired due to lack of steering updates
    Expired = 2,
}

/// The state of a parachute deployment (main or drogue).
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum DeploymentState {
    #[default]
    Unarmed = 0,
    NotYetDeployed = 1,
    Deployed = 2,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SteeringPositions {
    /// Current position of the left steering motor in steps (4096 steps = 1 revolution) from the
    /// startup position (0). Positive values mean pulling.
    pub left_pos: i32,
    /// Current position of the right steering motor in steps (4096 steps = 1 revolution) from the
    /// startup position (0). Positive values mean pulling.
    pub right_pos: i32,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryPowerConfig {
    /// Whether power to the separation actuators should be enabled. Physically,
    /// actuator power is only provided after hardware electrical arming of the system.
    pub separation_enabled: bool,
    /// Whether power to the deployment actuators should be enabled. Physically,
    /// actuator power is only provided after hardware electrical arming of the system.
    pub deployment_enabled: bool,
    /// Whether power to the steering actuators should be enabled.
    /// Steering power is not affected by arming.
    pub steering_enabled: bool,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryBoardStatus {
    #[mavlink(flatten)]
    pub common: StatusCommonMessage,
    /// Detected state of separation actuator 1.
    #[mavlink(refine)]
    pub sep1_status: ActuatorStatus,
    /// Detected state of separation actuator 2.
    #[mavlink(refine)]
    pub sep2_status: ActuatorStatus,
    /// Detected state of deployment actuator 1.
    #[mavlink(refine)]
    pub depl1_status: ActuatorStatus,
    /// Detected state of deployment actuator 2.
    #[mavlink(refine)]
    pub depl2_status: ActuatorStatus,
    /// State of the left steering motor.
    #[mavlink(refine)]
    pub steering_left_status: SteeringStatus,
    /// State of the right steering motor.
    #[mavlink(refine)]
    pub steering_right_status: SteeringStatus,
    /// Current state of the steering watchdog.
    #[mavlink(refine)]
    pub steering_watchdog_status: WatchdogState,
    /// `Armed` if the Recovery board is armed and the recovery actuators receive power, ie recovery arming Pin is pulled
    #[mavlink(refine)]
    pub arming_state: ArmingState,
}
