//! ## CAN Message Group: Board Status & Heartbeats
//!
//! - **ID Range:** `0x300–0x3FF`
//! - **Count:** 256 messages
//! - **Message Class:** Board Status & Heartbeats
//! - **Priority:** Medium-High
//! - **Type:** Periodic
//!
//! Health monitoring and presence confirmation for each board.

use crate::messages::event_messages::DprState;
use bincode::{Decode, Encode};
use hermes_can_derive::can_message;
///////////////////////////////////////////////////////////////////////////////
// Common definitions
///////////////////////////////////////////////////////////////////////////////

/// Shared runtime status structure for all boards (minimally required fields).
#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatusCommonMessage {
    /// Number of errors encountered since startup.
    pub errors: u32,
    /// Number of microseconds since the last board restart.
    pub micros_since_restart: u64,
}

/// Represents the online/offline status of a sensor, encoded as a u8.
#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SensorStatus {
    /// Sensor is not providing data or is not responding.
    #[default]
    Offline = 0,
    /// Sensor is generating data and responding.
    Online = 1,
}

/// Represents the current arming state of the subsystem
///
/// ## Notes
/// Arming can be implemented differently depending on what precise subsystem is used. In general, this enum
/// refers to hardware arming.
#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ArmingState {
    /// System is safe to approach, arming action is necessary to use the system.
    /// This can mean for example that actuators do not have power.
    #[default]
    Safe,
    /// System is not safe to approach anymore. System can be used to its full capacity
    /// This can mean for example that previously unpowered actuators now have power.
    Armed,
}

// We implement these traits manually, otherwise bincode uses a u32 for the enum ¯\_(ツ)_/¯
impl ::bincode::Encode for SensorStatus {
    fn encode<__E: ::bincode::enc::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> Result<(), ::bincode::error::EncodeError> {
        match self {
            Self::Offline => {
                <u8 as ::bincode::Encode>::encode(&(0u8), encoder)?;
                Ok(())
            }
            Self::Online => {
                <u8 as ::bincode::Encode>::encode(&(1u8), encoder)?;
                Ok(())
            }
        }
    }
}
impl<__Context> ::bincode::Decode<__Context> for SensorStatus {
    fn decode<__D: ::bincode::de::Decoder<Context = __Context>>(
        decoder: &mut __D,
    ) -> Result<Self, ::bincode::error::DecodeError> {
        let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
        match variant_index {
            0u8 => Ok(Self::Offline {}),
            1u8 => Ok(Self::Online {}),
            variant => Err(::bincode::error::DecodeError::UnexpectedVariant {
                found: u32::from(variant),
                type_name: "SensorStatus",
                allowed: &::bincode::error::AllowedEnumVariants::Range { min: 0, max: 1 },
            }),
        }
    }
}
impl<'__de, __Context> ::bincode::BorrowDecode<'__de, __Context> for SensorStatus {
    fn borrow_decode<__D: ::bincode::de::BorrowDecoder<'__de, Context = __Context>>(
        decoder: &mut __D,
    ) -> Result<Self, ::bincode::error::DecodeError> {
        let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
        match variant_index {
            0u8 => Ok(Self::Offline {}),
            1u8 => Ok(Self::Online {}),
            variant => Err(::bincode::error::DecodeError::UnexpectedVariant {
                found: u32::from(variant),
                type_name: "SensorStatus",
                allowed: &::bincode::error::AllowedEnumVariants::Range { min: 0, max: 1 },
            }),
        }
    }
}

/// Represents the online/offline status of all sensors on the Sensor Carrier board.
/// Each field corresponds to a specific sensor instance.
#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SensorsHealth {
    /// Status of barometer on I2C2 (bus 1: PA9 = SCL, PA8 = SDA)
    pub barometer_bus_1: SensorStatus,
    /// Status of barometer on I2C3 (bus 2: PC8 = SCL, PC9 = SDA)
    pub barometer_bus_2: SensorStatus,
    /// Status of `SHT4x` temperature/humidity sensor on I2C2 (bus 1)
    pub dht_bus_1: SensorStatus,
    /// Status of `SHT4x` temperature/humidity sensor on I2C3 (bus 2)
    pub dht_bus_2: SensorStatus,
    /// Status of magnetometer on I2C2 (bus 1), INT on PA0
    pub magnetometer_bus_1: SensorStatus,
    /// Status of magnetometer on I2C3 (bus 2), INT on PA1
    pub magnetometer_bus_2: SensorStatus,
    /// Status of IMU (LSM6DSO32) on SPI3 (PB3–PB6, INT on PC13)
    pub imu_1: SensorStatus,
    /// Status of IMU (LSM6DSO32) on SPI1 (PA4–PA7, INT on PC14)
    pub imu_2: SensorStatus,
    /// Status of GPS module on USART1 (PC5 = TX, PC4 = RX)
    pub gps_1: SensorStatus,
    /// Status of GPS module on USART3 (PC11 = TX, PC10 = RX)
    pub gps_2: SensorStatus,
}

///////////////////////////////////////////////////////////////////////////////
// Board Status Messages (ordered by BoardId)
///////////////////////////////////////////////////////////////////////////////

/// # `FlightComputerStatus`
/// Periodic status message emitted by the Flight Computer.
///
/// ## Type
/// - `Periodic @ 1 Hz`
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Communicates the runtime health and status of the Flight Computer.
///
/// ## Reaction
/// Receivers may log the data or use it for diagnostics and system health checks.
/// It is also used to only arm the RFS if the FC is armed and ensure exclusive PSS control.
#[derive(Default)]
#[can_message(0x310)]
pub struct FlightComputerStatus {
    pub common: StatusCommonMessage,
    /// `Armed` if the flight computer is in the `Armed` state for launch,
    /// which enables REC actuator power and allows an ignition command.
    pub is_software_armed: ArmingState,
}

/// # `RfsComputerStatus`
/// Periodic status message emitted by the RFS Computer.
///
/// ## Type
/// - `Periodic @ 1 Hz`
///
/// ## Direction
/// - **Emitted by:** `RfsComputer`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Communicates the runtime health and status of the RFS Computer.
///
/// ## Reaction
/// Receivers may log the data or use it for diagnostics and system health checks.
/// It is also used to only ingite if the RFS is armed and ensure exclusive PSS control.
#[derive(Default)]
#[can_message(0x311)]
pub struct RfsComputerStatus {
    pub common: StatusCommonMessage,
    /// `Armed` if the remote filling station is in a `Armed` state for ignition
    /// This is a software arming state of the RFS state machine that is part of the RFS hardware arming
    pub is_ignition_armed: ArmingState,
}

/// # `SensorCarrierStatus`
/// Periodic status message emitted by the Sensor Carrier.
///
/// ## Type
/// - `Periodic @ 1 Hz`
///
/// ## Direction
/// - **Emitted by:** `SensorCarrier`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Communicates the runtime health of the Sensor Carrier.
///
/// ## Reaction
/// Receivers may log the data or use it for diagnostics and system health checks.
#[derive(Default)]
#[can_message(0x312)]
pub struct SensorCarrierStatus {
    pub common: StatusCommonMessage,
    pub sensor_health: SensorsHealth,
}

/// # `CommunicationBoardStatus`
/// Periodic status message emitted by the Communication Board.
///
/// ## Type
/// - `Periodic @ 1 Hz`
///
/// ## Direction
/// - **Emitted by:** `CommunicationBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Communicates the runtime health of the Communication Board.
///
/// ## Reaction
/// Receivers may log the data or use it for diagnostics and system health checks.
#[derive(Default)]
#[can_message(0x313)]
pub struct CommunicationBoardStatus {
    pub common: StatusCommonMessage,
}

/// # `EngineControlBoardStatus`
/// Periodic status message emitted by the Engine Control Board.
///
/// ## Type
/// - `Periodic @ 1 Hz`
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Communicates the runtime health of the Engine Control Board.
///
/// ## Reaction
/// Receivers may log the data or use it for diagnostics and system health checks.
#[derive(Default)]
#[can_message(0x314)]
pub struct EngineControlBoardStatus {
    pub common: StatusCommonMessage,
    /// State of the ADS1120 device used to read thermocouples
    pub thermocouple_status: SensorStatus,
    /// Hash of the Thrust Curve used for this flight. Defined by
    /// [`event_messages::ThrustPoint:: calculate_unique_hash`].
    pub thrust_curve_hash: u64,
    /// `Armed` if the engine is armed and the main valves receive power, ie engine arming Pin is pulled
    pub armed: ArmingState,
}

/// # `PowerBoardStatus`
/// Periodic status message emitted by the Power Board.
///
/// ## Type
/// - `Periodic @ 1 Hz`
///
/// ## Direction
/// - **Emitted by:** `PowerBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Communicates the runtime health of the Power Board.
///
/// ## Reaction
/// Receivers may log the data or use it for diagnostics and system health checks.
#[derive(Default)]
#[can_message(0x315)]
pub struct PowerBoardStatus {
    pub common: StatusCommonMessage,
}

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

/// # `RecoveryBoardStatus`
/// Periodic status message emitted by the Recovery Board.
///
/// ## Type
/// - `Periodic @ 1 Hz`
///
/// ## Direction
/// - **Emitted by:** `RecoveryBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Communicates the runtime health of the Recovery Board, including actuator and steering diagnostics.
///
/// ## Reaction
/// Receivers may log the data or use it for diagnostics and system health checks.
#[derive(Default)]
#[can_message(0x316)]
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

/// # `FuelControlBoardStatus`
/// Periodic status message emitted by the Fuel Control Board.
///
/// ## Type
/// - `Periodic @ 1 Hz`
///
/// ## Direction
/// - **Emitted by:** `FuelControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Communicates the runtime health of the Fuel Control Board, including sensor.
///
/// ## Reaction
/// Receivers may log the data or use it for diagnostics and system health checks.
#[derive(Default)]
#[can_message(0x320)]
pub struct FuelControlBoardStatus {
    pub common: StatusCommonMessage,
    /// State of the ADS1120 device used to read thermocouples
    pub thermocouple_status: SensorStatus,
    /// State of the Keller BUS used to read pressure, indicates whether a single sensor is online or not.
    pub pressure_bus: SensorStatus,
}

/// # `OxidizerControlBoardStatus`
/// Periodic status message emitted by the Oxidizer Control Board.
///
/// ## Type
/// - `Periodic @ 1 Hz`
///
/// ## Direction
/// - **Emitted by:** `OxidizerControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Communicates the runtime health of the Oxidizer Control Board, including sensor.
///
/// ## Reaction
/// Receivers may log the data or use it for diagnostics and system health checks.
#[derive(Default)]
#[can_message(0x321)]
pub struct OxidizerControlBoardStatus {
    pub common: StatusCommonMessage,
    /// State of the ADS1120 device used to read thermocouples
    pub thermocouple_status: SensorStatus,
    /// State of the Keller BUS used to read pressure, indicates whether a single sensor is online or not.
    pub pressure_bus: SensorStatus,
}

////////////////// Valve States //////////////////

/// # `ValveState`
/// Enum representing the state of a valve.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValveState {
    /// Power should be enabled for the valve
    Active,
    /// Power should not be disabled for the valve
    #[default]
    Inactive,
}

impl ::bincode::Encode for ValveState {
    fn encode<__E: ::bincode::enc::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), ::bincode::error::EncodeError> {
        match self {
            Self::Active => {
                <u8 as ::bincode::Encode>::encode(&(0u8), encoder)?;
                core::result::Result::Ok(())
            }
            Self::Inactive => {
                <u8 as ::bincode::Encode>::encode(&(1u8), encoder)?;
                core::result::Result::Ok(())
            }
        }
    }
}

impl<__Context> ::bincode::Decode<__Context> for ValveState {
    fn decode<__D: ::bincode::de::Decoder<Context = __Context>>(
        decoder: &mut __D,
    ) -> core::result::Result<Self, ::bincode::error::DecodeError> {
        let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
        match variant_index {
            0u8 => core::result::Result::Ok(Self::Active {}),
            1u8 => core::result::Result::Ok(Self::Inactive {}),
            variant => {
                core::result::Result::Err(::bincode::error::DecodeError::UnexpectedVariant {
                    found: u32::from(variant),
                    type_name: "ValveState",
                    allowed: &::bincode::error::AllowedEnumVariants::Range { min: 0, max: 1 },
                })
            }
        }
    }
}

impl<'__de, __Context> ::bincode::BorrowDecode<'__de, __Context> for ValveState {
    fn borrow_decode<__D: ::bincode::de::BorrowDecoder<'__de, Context = __Context>>(
        decoder: &mut __D,
    ) -> core::result::Result<Self, ::bincode::error::DecodeError> {
        let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
        match variant_index {
            0u8 => core::result::Result::Ok(Self::Active {}),
            1u8 => core::result::Result::Ok(Self::Inactive {}),
            variant => {
                core::result::Result::Err(::bincode::error::DecodeError::UnexpectedVariant {
                    found: u32::from(variant),
                    type_name: "ValveState",
                    allowed: &::bincode::error::AllowedEnumVariants::Range { min: 0, max: 1 },
                })
            }
        }
    }
}

/// # `FuelControlBoardValveStates`
/// Periodic status message emitted by the Fuel Control Board.
///
/// ## Type
/// - `Periodic @ 5 Hz`
///
/// ## Direction
/// - **Emitted by:** `FuelControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Communicates the Valve State of all Valves controlled by the Fuel Control Board, including the DPR control loop state and setpoint.
///
/// ## Reaction
/// Receivers may log the data or use it for diagnostics and system health checks.
#[derive(Default)]
#[can_message(0x322)]
pub struct FuelControlBoardValveStates {
    /// Fuel DPR Control loop state
    pub fss_dpr: DprState,
    /// Pressurization Valve state
    pub prz_vnt_vlv: ValveState,
    /// Fuel Vent Valve state
    pub fss_vnt_vlv: ValveState,
}

/// # `OxidizerControlBoardValveStates`
/// Periodic status message emitted by the Oxidizer Control Board.
///
/// ## Type
/// - `Periodic @ 5 Hz`
///
/// ## Direction
/// - **Emitted by:** `OxidizerControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Communicates the Valve State of all Valves controlled by the Oxidizer Control Board, including the DPR control loop state and setpoint.
///
/// ## Reaction
/// Receivers may log the data or use it for diagnostics and system health checks.
#[derive(Default)]
#[can_message(0x323)]
pub struct OxidizerControlBoardValveStates {
    /// Oxidizer DPR Control loop state
    pub oss_dpr: DprState,
    /// Oxidizer Vent Valve state
    pub oss_vnt_vlv: ValveState,
}

/// # `EngineControlBoardValveStates`
/// Periodic status message emitted by the Engine Control Board.
///
/// ## Type
/// - `Periodic @ 5 Hz`
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Communicates the Valve State of all Valves controlled by the Engine Control Board.
/// This includes the Oxidizer Main and Fuel Main Valve.
/// ## Reaction
/// Receivers may log the data or use it for diagnostics and system health checks.
#[derive(Default)]
#[can_message(0x324)]
pub struct EngineControlBoardValveStates {
    /// Fuel Main Valve state
    pub fss_mnl_vlv: ValveState,
    /// Oxidizer Main Valve state
    pub oss_mnl_vlv: ValveState,
}

/// # `RfsComputerValveStates`
/// Periodic status message emitted by the Remote Filling Station Computer.
///
/// ## Type
/// - `Periodic @ 5 Hz`
///
/// ## Direction
/// - **Emitted by:** `RfsComputer`
/// - **Received by:** `EngineControlBoard` (mainly)
///
/// ## Purpose
/// Communicates the actuator state of the actuators in the Remote Filling Station controllable via CAN.
/// This includes the ingiter valves and the spark plug.
/// ## Reaction
/// Receivers may log the data or use it for diagnostics and system health checks.
#[derive(Default)]
#[can_message(0x325)]
pub struct RfsComputerValveStates {
    /// The current active state of the normally closed GOX igniter valve.
    pub gox_ign_vlv: ValveState,
    /// The current active state of the normally closed H2 igniter valve.
    pub h2_ign_vlv: ValveState,
    /// The current active state of the normally closed GOX purge valve.
    pub gox_prg_vlv: ValveState,
    /// The current active state of the normally closed H2 purge valve.
    pub h2_prg_vlv: ValveState,
    /// The current active state of the normally off spark plug.
    pub ign_spk_plg: ValveState,
}

/// # `CameraPowerStatus`
/// Periodic camera power status message emitted by the Recovery Board.
///
/// ## Type
/// - `Periodic @ 5 Hz`, and instantly when the status changes.
///
/// ## Direction
/// - **Emitted by:** `RecoveryBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Communicates what the power state of the cameras should be as set by the recovery board.
///
/// ## Reaction
/// No action required.
#[derive(Default)]
#[can_message(0x330)]
pub struct CameraPowerStatus {
    /// True if power to the recording camera(s) is enabled
    pub recording_pwr_enabled: bool,
    /// True if power to the live feed camera(s) is enabled
    pub live_pwr_enabled: bool,
}
