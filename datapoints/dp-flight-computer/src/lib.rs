use data_core::Message;
use datatypes::status::BuildInformationCommon;
use gated_derive::gated_derive;

mod dptypes;
pub use dptypes::*;

#[derive(Message)]
#[gated_derive(data_core::CanMessage(feature = "can"))]
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
    Status(FlightComputerStatus),
    // ======================== RX ========================
}
