//! ## CAN Message Group: Event Messages
//!
//! - **ID Range:** `0x100–0x2FF`
//! - **Count:** 512 messages
//! - **Message Class:** Event Messages
//! - **Priority:** High
//! - **Type:** Event-based
//!
//! Inter-board coordination signals or other event messages.

use crate::messages::board_status::ValveState;
use bincode::{Decode, Encode};
use hermes_can_derive::can_message;

/// # `FiringAbortInitiation`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC to initialize a firing abort
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `EngineControlBoard`
///
/// ## Purpose
/// Initiate abortion of firing sequence
///
/// ## Reaction
/// The ECU should properly abort the firing
#[can_message(0x120)]
pub struct FiringAbortInitiation;

/// # `FiringAborted`
/// Event message sent by the ECU.
///
/// ## Type
/// - `Event`: sent by the ECU to signalize an abort has occurred during the firing sequence
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `FlightComputer`
///
/// ## Purpose
/// Signaling Firing aborts
///
/// ## Reaction
///
#[can_message(0x121)]
pub struct FiringAborted;

/// # `FiringInitiation`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC to initiate firing sequence
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `EngineControlBoard`
///
/// ## Purpose
/// Initiate firing sequence
///
/// ## Reaction
/// The Engine Control Board shall initiate the firing sequence upon receiving this command
#[can_message(0x122)]
pub struct FiringInitiation;

/// # `FiringCompleted`
/// Event message sent by the ECU.
///
/// ## Type
/// - `Event`: sent by the ECU to signalize the firing sequence has successfully been completed
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Signaling completion of firing sequence
///
/// ## Reaction
///
#[can_message(0x123)]
pub struct FiringCompleted;

/// # `OxidizerPressurization`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC to initiate pressurization on the OxidizerControlBoard
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `OxidizerControlBoard`
///
/// ## Purpose
/// Pressurize the Oxidizer Tank to a desired setpoint
///
/// ## Reaction
///
#[can_message(0x124)]
pub struct OxidizerPressurization {
    /// Target Tank pressure in barg, range: 0-60 barg
    pub target_pressure: f32,
}

/// # `FuelPressurization`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC to initiate pressurization on the FuelControlBoard
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `FuelControlBoard`
///
/// ## Purpose
/// Pressurize the Fuel Tank to a desired setpoint
///
/// ## Reaction
///
#[can_message(0x125)]
pub struct FuelPressurization {
    /// Target Tank pressure in barg, range: 0-60 barg
    pub target_pressure: f32,
}

/// # `OxidizerPressurizationGain`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC to configure the pressurization gain on the OxidizerControlBoard
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `OxidizerControlBoard`
///
/// ## Purpose
/// Configure the Oxidizer Pressurization Gain
///
/// ## Reaction
///
#[can_message(0x126)]
pub struct OxidizerPressurizationGain {
    /// Pressurization Proportional Gain in [barg/ms]
    /// opening_time = kp * (target_pressure - current_pressure), in milliseconds
    pub kp: f32,
}

#[can_message(0x127)]
/// # `FuelPressurizationGain`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC to configure the pressurization gain on the FuelControlBoard
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `FuelControlBoard`
///
/// ## Purpose
/// Configure the Fuel Pressurization Gain
///
/// ## Reaction
///
pub struct FuelPressurizationGain {
    /// Pressurization Proportional Gain in [barg/ms]
    /// opening_time = kp * (target_pressure - current_pressure), in milliseconds
    pub kp: f32,
}

/// # `OxidizerPressurizationAbort`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC to abort pressurization on the OxidizerControlBoard
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `OxidizerControlBoard`
///
/// ## Purpose
/// Abort Oxidizer Tank pressurization
///
/// ## Reaction
///
#[can_message(0x128)]
pub struct OxidizerPressurizationAbort;

/// # `FuelPressurizationAbort`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC to abort pressurization on the FuelControlBoard
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `FuelControlBoard`
///
/// ## Purpose
/// Abort Fuel Tank pressurization
///
/// ## Reaction
///
#[can_message(0x129)]
pub struct FuelPressurizationAbort;

/// # `OxidizerPressurizationCompleted`
/// Event message sent by the OxidizerControlBoard
///
/// ## Type
/// - `Event`: sent by the OCB to signal completed pressurization on the OxidizerControlBoard
///
/// ## Direction
/// - **Emitted by:** `OxidizerControlBoard`
/// - **Received by:** `FlightComputer`
///
/// ## Purpose
/// Signal completion of Oxidizer Tank pressurization
///
/// ## Reaction
///
#[can_message(0x12a)]
pub struct OxidizerPressurizationCompleted;

/// # `FuelPressurizationCompleted`
/// Event message sent by the FuelControlBoard.
///
/// ## Type
/// - `Event`: sent by the FCB to signal completed pressurization on the FuelControlBoard
///
/// ## Direction
/// - **Emitted by:** `FuelControlBoard`
/// - **Received by:** `FlightComputer`
///
/// ## Purpose
/// Signal completion of Fuel Tank pressurization
///
/// ## Reaction
///
#[can_message(0x12b)]
pub struct FuelPressurizationCompleted;

/// # `FiringInitiated`
/// Event message sent by the EngineControlBoard.
///
/// ## Type
/// - `Event`: sent by the ECU to signal firing initiation
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Signal initiation of firing sequence
///
/// ## Reaction
///
#[can_message(0x130)]
pub struct FiringInitiated;

/// # `IgnitionDetected`
/// Event message sent by the EngineControlBoard.
///
/// ## Type
/// - `Event`: sent by the ECU to signal successful igniter ignition
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Signal successful igniter ignition
///
/// ## Reaction
///
#[can_message(0x131)]
pub struct IgnitionDetected;

/// # `CombustionDetected`
/// Event message sent by the EngineControlBoard.
///
/// ## Type
/// - `Event`: sent by the ECU to signal successful engine combustion
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Signal successful engine combustion
///
/// ## Reaction
///
#[can_message(0x132)]
pub struct CombustionDetected;

/// # `SeparationTrigger`
/// Event message indicating that separation shall be initiated.
///
/// ## Type
/// - `Event`: sent when separation is triggered
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `RecoveryBoard`
///
/// ## Purpose
/// Informs the Recovery Board that separation has occurred and that
/// it should trigger the separation actuators.
///
/// ## Reaction
/// The Recovery Board shall immediately trigger the separation actuators.
#[can_message(0x1FA)]
pub struct SeparationTrigger;

/// # `DeploymentTrigger`
/// Event message indicating that deployment shall be initiated.
///
/// ## Type
/// - `Event`: sent when main chute deployment is triggered
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `RecoveryBoard`
///
/// ## Purpose
/// Informs the Recovery Board that main chute deployment has been triggered
/// and that it should activate the deployment actuators.
///
/// ## Reaction
/// The Recovery Board shall immediately trigger the deployment actuators.
#[can_message(0x1FB)]
pub struct DeploymentTrigger;

/// # `SeparationOccurred`
/// Event message indicating that separation has been triggered.
///
/// ## Type
/// - `Event`: sent after separation is triggered
///
/// ## Direction
/// - **Emitted by:** `RecoveryBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Confirms that the Recovery Board has triggered the separation actuators.
/// Intended for logging, telemetry and other troubleshooting.
///
/// ## Reaction
/// No action required.
#[can_message(0x1FC)]
pub struct SeparationOccurred;

/// # `DeploymentOccurred`
/// Event message indicating that deployment has been triggered.
///
/// ## Type
/// - `Event`: sent after triggering deployment
///
/// ## Direction
/// - **Emitted by:** `RecoveryBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Confirms that the Recovery Board has triggered the deployment actuators.
/// Intended for logging, telemetry and other troubleshooting.
///
/// ## Reaction
/// No action required.
#[can_message(0x1FD)]
pub struct DeploymentOccurred;

/// # `SteeringTargetPositions`
/// Periodic event message emitted by the Flight Computer.
///
/// ## Type
/// - `Periodic @ 10 Hz`, but only during active steering phase
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `RecoveryBoard`
///
/// ## Purpose
/// Sets motor target positions for the steering servos.
///
/// ## Reaction
/// The first received message starts the `RecoveryBoard` watchdog, which expects frequent
/// steering target position messages until deactivation.
///
/// When this message is received, the `RecoveryBoard` shall set the target positions for the left and
/// right steering motors to the values indicated by `left_pos` and `right_pos` respectively.
#[can_message(0x1FE)]
pub struct SteeringTargetPositions {
    /// Absolute position from startup (0) for the left steering motor in steps (4096 steps = 1 revolution).
    /// Larger means pulling line in (shortening it), smaller means giving line out (lengthening it).
    pub left_pos: i32,
    /// Absolute position from startup (0) for the right steering motor in steps (4096 steps = 1 revolution).
    /// Larger means pulling line in (shortening it), smaller means giving line out (lengthening it).
    pub right_pos: i32,
}

/// # `SteeringActualPositions`
/// Periodic event message emitted by the Recovery Board.
///
/// ## Type
/// - `Periodic @ 10 Hz`, but only during active steering phase
///
/// ## Direction
/// - **Emitted by:** `RecoveryBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Returns the actual motor positions of the left and right steering actuators for monitoring or logging.
///
/// ## Reaction
/// No action required.
#[can_message(0x1FF)]
pub struct SteeringActualPositions {
    /// Current position of the left steering motor in steps (4096 steps = 1 revolution) from the
    /// startup position (0).
    pub left_pos: i32,
    /// Current position of the right steering motor in steps (4096 steps = 1 revolution) from the
    /// startup position (0).
    pub right_pos: i32,
}

/// # `RecoveryPowerConfig`
/// Event message sent by the Flight Computer.
///
/// ## Type
/// - `Event`: sent by the Flight Computer to change the actuator power state
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `RecoveryBoard`
///
/// ## Purpose
/// Configures the power state for all Recovery Board actuators.
///
/// ## Reaction
/// The Recovery Board shall switch power to the actuators as specified in the message.
#[can_message(0x200)]
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

/// # `CameraPowerControl`
/// Controls the camera power lines.
///
/// ## Type
/// - `Event`: triggered to toggle camera power outputs
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `PowerBoard`
///
/// ## Purpose
/// Allows remote enabling/disabling of camera recording and live feed.
///
/// ## Reaction
/// The Power Board sets the respective output pins according to the fields.
#[can_message(0x201)]
pub struct CameraPowerControl {
    /// Whether the recording camera(s) should be powered on (true=on, false=off).
    pub recording_pwr_enabled: bool,
    /// Whether the livestream camera(s) should be powered on (true=on, false=off).
    pub live_pwr_enabled: bool,
}

/// # `GoxIgniterValveControl`
/// Controls the GOX igniter valve on the RFS from the ECU.
///
/// ## Type
/// - `Event`: sent by the ECU
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `RemoteFillingStation`
///
/// ## Purpose
/// This is needed during the ignition sequence where the engine control board controls the igniter
/// and thus needs to open and close valves.
///
/// ## Reaction
/// The remote filling station shall adjust the GOX igniter valve state accordingly.
#[can_message(0x20a)]
pub struct GoxIgniterValveControl {
    /// The target active state of the normally closed GOX igniter valve.
    pub gox_ign_vlv: ValveState,
}

/// # `H2IgniterValveControl`
/// Controls the H2 igniter valve on the RFS from the ECU.
///
/// ## Type
/// - `Event`: sent by the ECU
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `RemoteFillingStation`
///
/// ## Purpose
/// This is needed during the ignition sequence where the engine control board controls the igniter
/// and thus needs to open and close valves.
///
/// ## Reaction
/// The remote filling station shall adjust the H2 igniter valve state accordingly.
#[can_message(0x20b)]
pub struct H2IgniterValveControl {
    /// The target active state of the normally closed H2 igniter valve.
    pub h2_ign_vlv: ValveState,
}

/// # `IgniterSparkPlugControl`
/// Controls the igniter spark plug on the RFS from the ECU.
///
/// ## Type
/// - `Event`: sent by the ECU
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `RemoteFillingStation`
///
/// ## Purpose
/// This is needed during the ignition sequence where the engine control board controls the igniter
/// and thus needs to open and close valves and activate the spark plug.
///
/// ## Reaction
/// The remote filling station shall adjust spark plug state accordingly.
#[can_message(0x20c)]
pub struct IgniterSparkPlugControl {
    /// The target active state of the normally off spark plug.
    pub ign_spk_plg: ValveState,
}

/// # `IgniterPurgeControl`
/// Controls the GOX and H2 purge valves on the RFS from the ECU.
///
/// ## Type
/// - `Event`: sent by the ECU
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `RemoteFillingStation`
///
/// ## Purpose
/// This is needed during the ignition sequence where the engine control board controls the ignition
/// and thus needs to open and close valves.
///
/// ## Reaction
/// The remote filling station shall adjust the GOX and H2 purge valve state accordingly.
#[can_message(0x20d)]
pub struct IgniterPurgeControl {
    /// The target active state of the normally closed GOX and H2 purge valves.
    pub gox_prg_vlv: ValveState,
}

///////////////////// Pressure Control Messages /////////////////////////
// The following 3 sets of messages have the same meaning, but are separate messages
// sent from different systems such they can never collide.

///////////////////// Pressure Control Messages emitted by FC /////////////////////////
#[derive(Copy, Clone, Debug, PartialEq, Encode, Decode, Default)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DprState {
    /// Enabled with pressure setpoint in Barg. 0-60.
    Enabled(f32),
    /// Disabled
    #[default]
    Disabled,
}

/// # `OxidizerDprConfigFC`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `OxidizerControlBoard`
///
/// ## Purpose
/// Set OXD Pressure Regulator configuration
/// Configures whether the control loop should be active, and what reference point (setpoint) it should assume.
///
/// ## Reaction
/// The Oxidizer Control Board shall adjust the OXD DPR setpoint and enable/disable the controller accordingly
#[can_message(0x210)]
pub struct OxidizerDprConfigFC {
    /// State of the OSS DPR Control Loop
    pub oss_dpr: DprState,
}

/// # `FuelDprConfigFC`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `FuelControlBoard`
///
/// ## Purpose
/// Set ETH Pressure Regulator configuration
/// Configures whether the control loop should be active, and what reference point (setpoint) it should assume.
///
/// ## Reaction
/// The Fuel Control Board shall adjust the FUE DPR setpoint and enable/disable the controller accordingly
#[can_message(0x211)]
pub struct FuelDprConfigFC {
    /// State of the FSS DPR Control Loop
    pub fss_dpr: DprState,
}

/// # `PressurizationVentControlFC`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `FuelControlBoard`
///
/// ## Purpose
/// Sets the N2 Vent Valve to a certain state.
///
/// ## Reaction
/// The Fuel Control Board shall adjust N2 Vent Valve state accordingly
#[can_message(0x212)]
pub struct PressurizationVentControlFC {
    pub prz_vnt_vlv: ValveState,
}

/// # `FuelVentControlFC`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `FuelControlBoard`
///
/// ## Purpose
/// Sets the Fuel Vent Valve to a certain state.
///
/// ## Reaction
/// The Fuel Control Board shall adjust the Fuel Vent Valve state accordingly
#[can_message(0x213)]
pub struct FuelVentControlFC {
    pub fss_vnt_vlv: ValveState,
}

/// # `OxidizerVentControlFC`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `OxidizerControlBoard`
///
/// ## Purpose
/// Sets the Oxidizer Vent Valve to a certain state.
///
/// ## Reaction
/// The Oxidizer Control Board shall adjust the Oxidizer Vent Valve state accordingly
#[can_message(0x214)]
pub struct OxidizerVentControlFC {
    pub oss_vnt_vlv: ValveState,
}

///////////////////// Pressure Control Messages emitted by ECU /////////////////////////

/// # `OxidizerDprConfigECU`
/// Event message sent by the ECU.
///
/// ## Type
/// - `Event`: sent by the ECU
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `OxidizerControlBoard`
///
/// ## Purpose
/// Set OXD Pressure Regulator configuration
/// Configures whether the control loop should be active, and what reference point (setpoint) it should assume.
///
/// ## Reaction
/// The Oxidizer Control Board shall adjust the OXD DPR setpoint and enable/disable the controller accordingly
#[can_message(0x215)]
pub struct OxidizerDprConfigECU {
    /// State of the OSS DPR Control Loop
    pub oss_dpr: DprState,
}

/// # `FuelDprConfigECU`
/// Event message sent by the ECU.
///
/// ## Type
/// - `Event`: sent by the ECU
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `FuelControlBoard`
///
/// ## Purpose
/// Set ETH Pressure Regulator configuration
/// Configures whether the control loop should be active, and what reference point (setpoint) it should assume.
///
/// ## Reaction
/// The Fuel Control Board shall adjust the FUE DPR setpoint and enable/disable the controller accordingly
#[can_message(0x216)]
pub struct FuelDprConfigECU {
    /// State of the FSS DPR Control Loop
    pub fss_dpr: DprState,
}

/// # `PressurizationVentControlECU`
/// Event message sent by the ECU.
///
/// ## Type
/// - `Event`: sent by the ECU
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `FuelControlBoard`
///
/// ## Purpose
/// Sets the N2 Vent Valve to a certain state.
///
/// ## Reaction
/// The Fuel Control Board shall adjust N2 Vent Valve state accordingly
#[can_message(0x217)]
pub struct PressurizationVentControlECU {
    pub prz_vnt_vlv: ValveState,
}

/// # `FuelVentControlECU`
/// Event message sent by the ECU.
///
/// ## Type
/// - `Event`: sent by the ECU
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `FuelControlBoard`
///
/// ## Purpose
/// Sets the Fuel Vent Valve to a certain state.
///
/// ## Reaction
/// The Fuel Control Board shall adjust the Fuel Vent Valve state accordingly
#[can_message(0x218)]
pub struct FuelVentControlECU {
    pub fss_vnt_vlv: ValveState,
}

/// # `OxidizerVentControlECU`
/// Event message sent by the ECU.
///
/// ## Type
/// - `Event`: sent by the ECU
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `OxidizerControlBoard`
///
/// ## Purpose
/// Sets the Oxidizer Vent Valve to a certain state.
///
/// ## Reaction
/// The Oxidizer Control Board shall adjust the Oxidizer Vent Valve state accordingly
#[can_message(0x219)]
pub struct OxidizerVentControlECU {
    pub oss_vnt_vlv: ValveState,
}

///////////////// Pressure Control Messages emitted by Remote filling station ///////////////////////

/// # `OxidizerDprConfigRFS`
/// Event message sent by the RFS.
///
/// ## Type
/// - `Event`: sent by the RFS
///
/// ## Direction
/// - **Emitted by:** `RemoteFillingStation`
/// - **Received by:** `OxidizerControlBoard`
///
/// ## Purpose
/// Set OXD Pressure Regulator configuration
///
/// ## Reaction
/// The Oxidizer Control Board shall adjust the OXD DPR setpoint and enable/disable the controller accordingly
#[can_message(0x21a)]
pub struct OxidizerDprConfigRFS {
    /// State of the OSS DPR Control Loop
    pub oss_dpr: DprState,
}

/// # `FuelDprConfigRFS`
/// Event message sent by the RFS.
///
/// ## Type
/// - `Event`: sent by the RFS
///
/// ## Direction
/// - **Emitted by:** `RemoteFillingStation`
/// - **Received by:** `FuelControlBoard`
///
/// ## Purpose
/// Set ETH Pressure Regulator configuration
///
/// ## Reaction
/// The Fuel Control Board shall adjust the FUE DPR setpoint and enable/disable the controller accordingly
#[can_message(0x21b)]
pub struct FuelDprConfigRFS {
    /// State of the FSS DPR Control Loop
    pub fss_dpr: DprState,
}

/// # `PressurizationVentControlRFS`
/// Event message sent by the RFS.
///
/// ## Type
/// - `Event`: sent by the RFS
///
/// ## Direction
/// - **Emitted by:** `RemoteFillingStation`
/// - **Received by:** `FuelControlBoard`
///
/// ## Purpose
/// Sets the N2 Vent Valve to a certain state.
///
/// ## Reaction
/// The Fuel Control Board shall adjust N2 Vent Valve state accordingly
#[can_message(0x21c)]
pub struct PressurizationVentControlRFS {
    pub prz_vnt_vlv: ValveState,
}

/// # `FuelVentControlRFS`
/// Event message sent by the RFS.
///
/// ## Type
/// - `Event`: sent by the RFS
///
/// ## Direction
/// - **Emitted by:** `RemoteFillingStation`
/// - **Received by:** `FuelControlBoard`
///
/// ## Purpose
/// Sets the Fuel Vent Valve to a certain state.
///
/// ## Reaction
/// The Fuel Control Board shall adjust the Fuel Vent Valve state accordingly
#[can_message(0x21d)]
pub struct FuelVentControlRFS {
    pub fss_vnt_vlv: ValveState,
}

/// # `OxidizerVentControlRFS`
/// Event message sent by the RFS.
///
/// ## Type
/// - `Event`: sent by the RFS
///
/// ## Direction
/// - **Emitted by:** `RemoteFillingStation`
/// - **Received by:** `OxidizerControlBoard`
///
/// ## Purpose
/// Sets the Oxidizer Vent Valve to a certain state.
///
/// ## Reaction
/// The Oxidizer Control Board shall adjust the Oxidizer Vent Valve state accordingly
#[can_message(0x21e)]
pub struct OxidizerVentControlRFS {
    pub oss_vnt_vlv: ValveState,
}

/// # `OxidizerMainControlFC`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC to control the Oxidizer Main Valve
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `EngineControlBoard`
///
/// ## Purpose
/// Sets the Oxidizer Main valve to be active or inactive.
///
/// ## Reaction
/// The Engine Control Board shall change the Oxidizer Main Valve position accordingly
/// ## Note
/// The Engine Control Board might also change the valve position without input from this message, during the firing sequence
/// The Valve is Normally Closed
#[can_message(0x225)]
pub struct OxidizerMainControlFC {
    pub oss_mnl_vlv: ValveState,
}

/// # `FuelMainControlFC`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC to control the Fuel Main Valve
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `EngineControlBoard`
///
/// ## Purpose
/// Sets the Fuel Main valve to be active or inactive.
///
/// ## Reaction
/// The Engine Control Board shall change the Fuel Main Valve position accordingly.
/// ## Note
/// The Engine Control Board might also change the valve position without input from this message, during the firing sequence
/// /// The Valve is Normally Closed
#[can_message(0x226)]
pub struct FuelMainControlFC {
    pub fss_mnl_vlv: ValveState,
}

/// # `OxidizerMainControlRFS`
/// Event message sent by the RFS.
///
/// ## Type
/// - `Event`: sent by the RFS to control the Oxidizer Main Valve
///
/// ## Direction
/// - **Emitted by:** `RemoteFillingStation`
/// - **Received by:** `EngineControlBoard`
///
/// ## Purpose
/// Sets the Oxidizer Main valve to be active or inactive.
///
/// ## Reaction
/// The Engine Control Board shall change the Oxidizer Main Valve position accordingly
/// ## Note
/// The Engine Control Board might also change the valve position without input from this message, during the firing sequence
/// /// The Valve is Normally Closed
#[can_message(0x227)]
pub struct OxidizerMainControlRFS {
    pub oss_mnl_vlv: ValveState,
}

/// # `FuelMainControlRFS`
/// Event message sent by the RFS.
///
/// ## Type
/// - `Event`: sent by the RFS to control the Fuel Main Valve
///
/// ## Direction
/// - **Emitted by:** `RemoteFillingStation`
/// - **Received by:** `EngineControlBoard`
///
/// ## Purpose
/// Sets the Fuel Main valve to be active or inactive.
///
/// ## Reaction
/// The Engine Control Board shall change the Fuel Main Valve position accordingly.
/// ## Note
/// The Engine Control Board might also change the valve position without input from this message, during the firing sequence
/// /// The Valve is Normally Closed
#[can_message(0x228)]
pub struct FuelMainControlRFS {
    pub fss_mnl_vlv: ValveState,
}

/////////////////////////// End Pressure Control Messages //////////////////////////////

/// # `ThrustPoint`
/// Abstracted thrust point for the thrust curve.
/// Contains time after ignition and desired thrust level at that time.
#[derive(Copy, Clone, Debug, PartialEq, Default, Encode, Decode, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThrustPoint {
    /// Thrust level in percentage (0-100).
    pub thrust: u8,
    /// Time in 100ms steps after ignition when this thrust level should be reached.
    pub time: u8,
}

/// # `ThrustCurveConfig`
/// Event message sent by the FC.
///
/// ## Type
/// - `Event`: sent by the FC configure the Launch Thrust Curve
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `EngineControlBoard`
///
/// ## Purpose
/// Transmit the flight thrust curve to the Engine Control Board.
///
/// ## Reaction
///
#[can_message(0x240)]
pub struct ThrustCurveConfig {
    /// The thrust curve, consisting of 25 points.
    pub thrust_curve: [ThrustPoint; 25],
    /// The length of the thrust curve in points (0-25).
    pub thrust_curve_length: u8,
}

impl ThrustCurveConfig {
    /// Calculates a unique hash for the entire thrust curve array.
    /// This uses `SipHasher` which is deprecated, but still used because we don't care about
    /// cryptographic security here.
    #[must_use]
    pub fn calculate_unique_hash(&self) -> u64 {
        #![allow(deprecated)]
        use core::hash::{Hash, Hasher, SipHasher};
        let mut hasher = SipHasher::new();
        self.thrust_curve[..self.thrust_curve_length as usize].hash(&mut hasher);
        hasher.finish()
    }
}
