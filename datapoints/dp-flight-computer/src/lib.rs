#![cfg_attr(not(feature = "std"), no_std)]

use data_core::Message;
#[cfg(feature = "can")]
use data_core::can;
#[cfg(feature = "mavlink")]
use data_core::mavlink;
use datatypes::status::{BuildInformationCommon, ManualActuatorChange, ManualStateTransition};
use gated_derive::gated_derive;

mod dptypes;
pub use dptypes::*;

mod version;
pub use version::{VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH};

#[derive(Message)]
#[gated_derive(can::Message, Debug)]
#[gated_derive(mavlink::Message)]
#[gated_derive(utoipa::ToSchema, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", content = "content")]
#[schema(as = FlightComputerMessage)]
pub enum Message {
    // ======================== TX ========================
    /// # `FlightComputerBuildInfo`
    /// Firmware build metadata emitted by the Flight Computer.
    ///
    /// ## Type
    /// - `Periodic @ 0.2 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Allows version tracking and verification of the Flight Computer firmware.
    ///
    /// ## Reaction
    /// Receivers may log or display the firmware version and build metadata.
    #[can(id = 0x610)]
    #[mavlink(id = 0x10_01_00)]
    BuildInfo(BuildInformationCommon),

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
    #[can(id = 0x310)]
    #[mavlink(id = 0x10_01_01)]
    Status(FlightComputerStatus),
    // ======================== RX ========================
    /// Command a manual state transition on the FC.
    /// The state machine will perform the state transition and emit a state change event.
    #[mavlink(id = 0x20_01_00)]
    ManualStateTransition(ManualStateTransition),

    /// Command a specific actuator to move to a specific target position.
    /// The actuator name and the semantics of the value are defined using
    /// an `#[pss(name = "...", kind  = "...")]` attribute from the [`data_core::pss::Data`]
    /// macro.
    ///
    /// Upon change, an "actuator states" message will be emitted.
    #[mavlink(id = 0x20_01_01)]
    ManualActuatorChange(ManualActuatorChange),
}
