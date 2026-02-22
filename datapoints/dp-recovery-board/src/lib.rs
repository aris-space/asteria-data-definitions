#![cfg_attr(not(feature = "std"), no_std)]

use data_core::Message;
#[cfg(feature = "can")]
use data_core::can;
#[cfg(feature = "mavlink")]
use data_core::mavlink;
use datatypes::status::BuildInformationCommon;
use gated_derive::gated_derive;

mod dptypes;
pub use dptypes::*;

#[derive(Message)]
#[gated_derive(can::Message, Debug)]
#[gated_derive(mavlink::Message)]
#[gated_derive(utoipa::ToSchema, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", content = "content")]
#[schema(as = RecoveryBoardMessage)]
pub enum Message {
    // ======================== TX ========================
    /// # `RecoveryBoardBuildInfo`
    /// Firmware build metadata emitted by the Recovery Board.
    ///
    /// ## Type
    /// - `Periodic @ 0.2 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `RecoveryBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Allows version tracking and verification of the Recovery Board firmware.
    ///
    /// ## Reaction
    /// Receivers may log or display the firmware version and build metadata.
    #[can(id = 0x616)]
    BuildInfo(BuildInformationCommon),

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
    #[can(id = 0x316)]
    BoardStatus(RecoveryBoardStatus),

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
    #[can(id = 0x1FF)]
    SteeringActualPositions(SteeringPositions),

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
    #[can(id = 0x1FC)]
    SeparationOccurred,

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
    #[can(id = 0x1FD)]
    DeploymentOccurred,

    // ======================== RX ========================
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
    #[can(id = 0x1FE)]
    SteeringTargetPositions(SteeringPositions),

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
    #[can(id = 0x200)]
    RecoveryPowerConfig(RecoveryPowerConfig),

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
    #[can(id = 0x1FA)]
    SeparationTrigger,

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
    #[can(id = 0x1FB)]
    DeploymentTrigger,
}
