use bincode::{Decode, Encode};
use datatypes::{ArmingState, StatusCommonMessage};

/// Represents the current state of an actuator as detected by a board.
///
/// ## Notes
/// Detection of actuator presence is only possible before power is applied to the actuator.
/// Once powered, actuator detection is no longer possible.
#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActuatorStatus {
    /// Actuator is not connected (e.g. unplugged or circuit open).
    #[default]
    NotConnected,
    /// Actuator is connected but not powered.
    Connected,
    /// Actuator is powered; detection circuit is no longer active.
    PowerOn,
}

/// Represents the current state of a watchdog timer.
#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WatchdogState {
    /// Watchdog is not active. No steering signal has been received since startup.
    #[default]
    Disabled,
    /// Watchdog is actively monitoring and receiving valid steering updates.
    Active,
    /// Watchdog has expired due to lack of steering updates
    Expired,
}

#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub struct SteeringPositions {
    /// Current position of the left steering motor in steps (4096 steps = 1 revolution) from the
    /// startup position (0).
    left_pos: i32,
    /// Current position of the right steering motor in steps (4096 steps = 1 revolution) from the
    /// startup position (0).
    right_pos: i32,
}

#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub struct RecoveryPowerConfig {
    /// Whether power to the separation actuators should be enabled. Physically,
    /// actuator power is only provided after hardware electrical arming of the system.
    separation_enabled: bool,
    /// Whether power to the deployment actuators should be enabled. Physically,
    /// actuator power is only provided after hardware electrical arming of the system.
    deployment_enabled: bool,
    /// Whether power to the steering actuators should be enabled.
    /// Steering power is not affected by arming.
    steering_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub struct RecoveryBoardStatus {
    pub common: StatusCommonMessage,
    /// Detected state of separation actuator 1.
    pub sep1_status: ActuatorStatus,
    /// Detected state of separation actuator 2.
    pub sep2_status: ActuatorStatus,
    /// Detected state of deployment actuator 1.
    pub depl1_status: ActuatorStatus,
    /// Detected state of deployment actuator 2.
    pub depl2_status: ActuatorStatus,
    /// General steering actuator state as observed from detection circuitry.
    pub steering_general: ActuatorStatus,
    /// True if left steering motor has correctly responded to ping.
    /// Will be false until the steering servos have power
    pub steering_left_connected: bool,
    /// True if right steering motor has correctly responded to ping.
    /// Will be false until the steering servos have power
    pub steering_right_connected: bool,
    /// Current state of the steering watchdog.
    pub steering_watchdog_status: WatchdogState,
    /// `Armed` if the Recovery board is armed and the recovery actuators receive power, ie recovery arming Pin is pulled
    pub arming_state: ArmingState,
}
