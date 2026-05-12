#![cfg_attr(not(feature = "std"), no_std)]

use data_core::Message;
#[cfg(feature = "can")]
use data_core::can;
#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "pss")]
use data_core::pss;
use datatypes::status::{
    BuildInformationCommon, ManualActuatorChange, ManualStateTransition, ValveState,
};
use gated_derive::gated_derive;

mod dptypes;
pub use dptypes::*;

mod version;
pub use version::{VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH};

#[derive(Message, Clone)]
#[gated_derive(can::Message, Debug, defmt::Format)]
#[gated_derive(mavlink::Message)]
#[gated_derive(utoipa::ToSchema, serde::Deserialize, serde::Serialize)]
#[gated_derive(pss::Data)]
#[serde(tag = "type", content = "content")]
#[schema(as = RfsComputerMessage)]
pub enum Message {
    // ======================== TX ========================
    /// # `RfsComputerBuildInfo`
    /// Firmware build metadata emitted by the RFS Computer.
    ///
    /// ## Type
    /// - `Periodic @ 0.2 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `RfsComputer`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Allows version tracking and verification of the RFS Computer firmware.
    ///
    /// ## Reaction
    /// Receivers may log or display the firmware version and build metadata.
    #[can(id = 0x611)]
    #[mavlink(id = 0x10_06_00)]
    BuildInfo(BuildInformationCommon),

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
    #[can(id = 0x311)]
    #[mavlink(id = 0x10_06_01)]
    Status(RfsComputerStatus),

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
    #[can(id = 0x325)]
    #[mavlink(id = 0x10_06_02)]
    ValveStates(RfsComputerValveStates),

    // ======================== RX ========================
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
    #[can(id = 0x20a)]
    GoxIgniterValveControl(ValveState),

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
    #[can(id = 0x20b)]
    #[mavlink(id = 0x10_06_03)]
    H2IgniterValveControl(ValveState),

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
    #[can(id = 0x20c)]
    #[mavlink(id = 0x10_06_04)]
    IgniterSparkPlugControl(ValveState),

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
    #[can(id = 0x20d)]
    #[mavlink(id = 0x10_06_05)]
    IgniterPurgeControl(ValveState),

    /// Command a manual state transition on the RFSC.
    /// The state machine will perform the state transition and emit a state change event.
    #[mavlink(id = 0x20_06_00)]
    ManualStateTransition(ManualStateTransition),

    /// Command a specific actuator to move to a specific target position.
    /// The actuator name and the semantics of the value are defined using
    /// an `#[pss(name = "...", kind  = "...")]` attribute from the [`data_core::pss::Data`]
    /// macro.
    ///
    /// Upon change, an "actuator states" message will be emitted.
    #[mavlink(id = 0x20_06_01)]
    ManualActuatorChange(ManualActuatorChange),
}
