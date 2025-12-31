use data_core::{CanMessage, Message};
use datatypes::{BuildInformationCommon, StatusCommonMessage};

#[derive(Message, CanMessage)]
pub enum Message {
    // ======================== TX ========================
    /// # `CommunicationBoardBuildInfo`
    /// Firmware build metadata emitted by the Communication Board.
    ///
    /// ## Type
    /// - `Periodic @ 0.2 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `CommunicationBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Allows version tracking and verification of the Communication Board firmware.
    ///
    /// ## Reaction
    /// Receivers may log or display the firmware version and build metadata.
    #[can(id = 0x613)]
    CommunicationBoardBuildInfo(BuildInformationCommon),

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
    #[can(id = 0x313)]
    CommunicationBoardStatus(StatusCommonMessage),
    // ======================== RX ========================
}
