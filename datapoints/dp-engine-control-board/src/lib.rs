#![cfg_attr(not(feature = "std"), no_std)]

use data_core::Message;
#[cfg(feature = "can")]
use data_core::can;
#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "pss")]
use data_core::pss;
use datatypes::actuator::NormallyClosedValve;
use datatypes::status::BuildInformationCommon;
use datatypes::units::BarG;
use gated_derive::gated_derive;

mod dptypes;
pub use dptypes::*;

mod version;
pub use version::{VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH};

#[derive(Message)]
#[gated_derive(can::Message, Debug, defmt::Format)]
#[gated_derive(mavlink::Message)]
#[gated_derive(utoipa::ToSchema, serde::Deserialize, serde::Serialize)]
#[gated_derive(pss::Data)]
#[serde(tag = "type", content = "content")]
#[schema(as = EngineControlBoardMessage)]
pub enum Message {
    // ======================== TX ========================
    /// # `EngineControlBoardBuildInfo`
    /// Firmware build metadata emitted by the Engine Control Board.
    ///
    /// ## Type
    /// - `Periodic @ 0.2 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Allows version tracking and verification of the Engine Control Board firmware.
    ///
    /// ## Reaction
    /// Receivers may log or display the firmware version and build metadata.
    #[can(id = 0x614)]
    #[mavlink(id = 0x10_08_00)]
    BuildInfo(BuildInformationCommon),

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
    #[can(id = 0x314)]
    #[mavlink(id = 0x10_08_01)]
    BoardStatus(EngineControlBoardStatus),

    /// ## Type
    /// - `Periodic @ 5 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the Fuel Main Valve State.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x324)]
    #[mavlink(id = 0x10_08_02)]
    #[pss(label = "Fuel Main Valve", name = "fss_ml_vlv")]
    FuelMainValveState(NormallyClosedValve),

    /// ## Type
    /// - `Periodic @ 5 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the Oxidizer Main Valve State.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x329)]
    #[mavlink(id = 0x10_08_05)]
    #[pss(label = "Oxidizer Main Valve", name = "oss_ml_vlv")]
    OxidizerMainValveState(NormallyClosedValve),

    /// ## Type
    /// - `Periodic @ 20 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Reports the Chamber Pressure Sensor reading.
    ///
    /// ## Reaction
    ///
    #[can(id = 0x460)]
    #[mavlink(id = 0x10_08_03)]
    #[pss(label = "Chamber Pressure Sensor", name = "eng_cc_p")]
    ChamberPressureSensor(BarG),

    /// ## Type
    /// - `Periodic @ 20 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Reports the Injector Fuel Pressure Sensor reading.
    ///
    /// ## Reaction
    ///
    #[can(id = 0x462)]
    #[mavlink(id = 0x10_08_06)]
    #[pss(label = "Injector Fuel Pressure Sensor", name = "eng_inj_p")]
    InjectorFuelPressureSensor(BarG),

    /// ## Type
    /// - `Periodic @ 20 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Reports the Injector Oxidizer Pressure Sensor reading.
    ///
    /// ## Reaction
    ///
    #[can(id = 0x463)]
    #[mavlink(id = 0x10_08_07)]
    #[pss(label = "Injector Oxidizer Pressure Sensor", name = "oss_inj_p")]
    InjectorOxidizerPressureSensor(BarG),

    /// # `EngineBayTemperature`
    /// Periodic telemetry message emitted by the Engine Control Board.
    ///
    /// ## Type
    /// - `Periodic @ 10 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Reports various Data from Thermocouples in the Engine Bay.
    /// This includes the OSS Runline Temperature and the OSS Tank Bottom Temperature.
    ///
    /// ## Reaction
    /// No action required.
    ///
    /// ## Data Origin
    /// Data originates from high precision ADCs which read the differential thermocouple voltage together with a thermistor from which the temperature is calculated.
    /// Thermocouples installed can be either Type K or Type T.
    /// If the value read in firmware is outside the specific temperature range of [-200.0, 200.0] °C,
    /// the value will be set to `NaN`.
    #[can(id = 0x461)]
    #[mavlink(id = 0x10_08_04)]
    #[pss(flatten)]
    EngineBayTemperature(EngineBayTemperature),

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
    #[can(id = 0x121)]
    FiringAborted,

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
    #[can(id = 0x123)]
    FiringCompleted,

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
    #[can(id = 0x130)]
    FiringInitiated,

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
    #[can(id = 0x131)]
    IgnitionDetected,

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
    #[can(id = 0x132)]
    CombustionDetected,

    // ======================== RX ========================
    /// # `FiringAbortInitiation`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC to initialize a firing abort
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `EngineControlBoard`
    ///datapoints/dp-flight-computer/src
    /// ## Purpose
    /// Initiate abortion of firing sequence
    ///
    /// ## Reaction
    /// The ECU should properly abort the firing
    #[can(id = 0x120)]
    FiringAbortInitiation,

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
    #[can(id = 0x122)]
    FiringInitiation,

    // ======================== RX (cont.) ========================
    // The following 2 sets of messages have the same meaning, but are separate messages
    // sent from different systems such they can never collide.
    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `EngineControlBoard`
    ///
    /// ## Purpose
    /// Sets the Oxidizer Main valve to a certain state.
    ///
    /// ## Reaction
    /// The Engine Control Board shall change the Oxidizer Main Valve position accordingly
    #[can(id = 0x225)]
    OxidizerMainValveControlFC(NormallyClosedValve),

    /// ## Type
    /// - `Event`: sent by the RFS
    ///
    /// ## Direction
    /// - **Emitted by:** `RemoteFillingStation`
    /// - **Received by:** `EngineControlBoard`
    ///
    /// ## Purpose
    /// Sets the Oxidizer Main valve to a certain state.
    ///
    /// ## Reaction
    /// The Engine Control Board shall change the Oxidizer Main Valve position accordingly
    #[can(id = 0x227)]
    OxidizerMainValveControlRFS(NormallyClosedValve),

    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `EngineControlBoard`
    ///
    /// ## Purpose
    /// Sets the Fuel Main valve to a certain state.
    ///
    /// ## Reaction
    /// The Engine Control Board shall change the Fuel Main Valve position accordingly.
    #[can(id = 0x226)]
    FuelMainValveControlFC(NormallyClosedValve),

    /// ## Type
    /// - `Event`: sent by the RFS
    ///
    /// ## Direction
    /// - **Emitted by:** `RemoteFillingStation`
    /// - **Received by:** `EngineControlBoard`
    ///
    /// ## Purpose
    /// Sets the Fuel Main valve to a certain state.
    ///
    /// ## Reaction
    /// The Engine Control Board shall change the Fuel Main Valve position accordingly.
    #[can(id = 0x228)]
    FuelMainValveControlRFS(NormallyClosedValve),
}
