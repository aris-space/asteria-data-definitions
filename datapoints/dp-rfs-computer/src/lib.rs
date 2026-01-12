use data_core::Message;
use datatypes::status::{BuildInformationCommon, ValveState};
use gated_derive::gated_derive;

mod dptypes;
pub use dptypes::*;

#[derive(Message)]
#[gated_derive(data_core::CanMessage(feature = "can"))]
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
    RfsComputerBuildInfo(BuildInformationCommon),

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
    RfsComputerStatusMsg(RfsComputerStatus),

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
    RfsComputerValveStatesMsg(RfsComputerValveStates),

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
    IgniterPurgeControl(ValveState),
}
