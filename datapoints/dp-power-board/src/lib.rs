use data_core::Message;
use datatypes::actuator::CameraPowerStatus;
use datatypes::status::{BuildInformationCommon, StatusCommonMessage};
use datatypes::units::RailStatus;

use gated_derive::gated_derive;

#[derive(Message)]
#[gated_derive(data_core::CanMessage(feature = "can"))]
pub enum Message {
    // ======================== TX ========================
    /// # `PowerBoardBuildInfo`
    /// Firmware build metadata emitted by the Power Board.
    ///
    /// ## Type
    /// - `Periodic @ 0.2 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `PowerBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Allows version tracking and verification of the Power Board firmware.
    ///
    /// ## Reaction
    /// Receivers may log or display the firmware version and build metadata.
    #[can(id = 0x615)]
    BuildInfo(BuildInformationCommon),

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
    #[can(id = 0x315)]
    BoardStatus(StatusCommonMessage),

    /// # `RailStatus5V`
    /// Periodic telemetry message reporting the 5V rail parameters.
    ///
    /// ## Type
    /// - Periodic @ 10Hz
    ///
    /// ## Direction
    /// - **Emitted by:** `PowerBoard`
    /// - **Received by:** unspecified
    ///
    /// ## Purpose
    /// Reports the measured voltage, current, and power on the 5V power rail.
    ///
    /// ## Reaction
    /// Receivers may log, display, or track these values. They may trigger warnings if any measurement is out of nominal range.
    #[can(id = 0x500)]
    RailStatus5V(RailStatus),

    /// # `RailStatus24V`
    /// Periodic telemetry message reporting the 24V rail parameters.
    ///
    /// ## Type
    /// - Periodic @ 10Hz
    ///
    /// ## Direction
    /// - **Emitted by:** `PowerBoard`
    /// - **Received by:** unspecified
    ///
    /// ## Purpose
    /// Reports the measured voltage, current, and power on the 24V power rail.
    ///
    /// ## Reaction
    /// Receivers may log, display, or track these values. They may trigger warnings if any measurement is out of nominal range.
    #[can(id = 0x501)]
    RailStatus24V(RailStatus),

    // ======================== RX ========================
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
    #[can(id = 0x201)]
    CameraPowerControl(CameraPowerStatus),

    /// # `CameraPowerStatus`
    /// Periodic camera power status message emitted by the Power Board.
    ///
    /// ## Type
    /// - `Periodic @ 5 Hz`, and instantly when the status changes.
    ///
    /// ## Direction
    /// - **Emitted by:** `PowerBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates what the power state of the cameras should be as set by the power board.
    ///
    /// ## Reaction
    /// No action required.
    #[can(id = 0x330)]
    CameraPowerStatus(CameraPowerStatus),
}
