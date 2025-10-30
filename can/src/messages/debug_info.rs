//! ## CAN Message Group: Build Info / Debug Metadata
//!
//! - **ID Range:** `0x600–0x6FF`
//! - **Count:** 256 messages
//! - **Message Class:** Build Info / Debug Metadata
//! - **Priority:** Low
//! - **Type:** Periodic
//!
//! Firmware version, debug flags, and other build-time metadata used for diagnostics and traceability.

use bincode::{Decode, Encode};
use hermes_can_derive::can_message;

///////////////////////////////////////////////////////////////////////////////
// Common definitions
///////////////////////////////////////////////////////////////////////////////

/// Shared firmware build information used by all boards.
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuildInformationCommon {
    /// Unix timestamp when the build was compiled, provided in seconds since
    /// the Unix epoch `(1970-01-01 00:00:00 UTC)`. While this is a 32-bit integer,
    /// overflow is not a concern as we don't intend to use this code up to 2038.
    pub unix_timestamp: u32,
    /// Initials of the person who compiled the firmware, or `\0\0` if not set.
    pub author_initials: [u8; 2],
    /// Whether the build is a release build.
    pub is_release: bool,
    /// Whether the build includes debugging defmt over RTT.
    pub debug_defmt_rtt: bool,
    /// Firmware version information.
    pub commit_hash: [u8; 7],
    /// Indicates if the git repository was dirty at build time.
    pub is_git_dirty: bool,
    /// CAN library version information (major, minor, patch).
    pub can_semver: [u8; 3],
}

///////////////////////////////////////////////////////////////////////////////
// Build Information Messages (ordered by BoardId)
///////////////////////////////////////////////////////////////////////////////

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
#[can_message(0x610)]
pub struct FlightComputerBuildInfo {
    pub data: BuildInformationCommon,
}

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
#[can_message(0x611)]
pub struct RfsComputerBuildInfo {
    pub data: BuildInformationCommon,
}

/// # `SensorCarrierBuildInfo`
/// Firmware build metadata emitted by the Sensor Carrier.
///
/// ## Type
/// - `Periodic @ 0.2 Hz`
///
/// ## Direction
/// - **Emitted by:** `SensorCarrier`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Allows version tracking and verification of the Sensor Carrier firmware.
///
/// ## Reaction
/// Receivers may log or display the firmware version and build metadata.
#[can_message(0x612)]
pub struct SensorCarrierBuildInfo {
    pub data: BuildInformationCommon,
}

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
#[can_message(0x613)]
pub struct CommunicationBoardBuildInfo {
    pub data: BuildInformationCommon,
}

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
#[can_message(0x614)]
pub struct EngineControlBoardBuildInfo {
    pub data: BuildInformationCommon,
}

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
#[can_message(0x615)]
pub struct PowerBoardBuildInfo {
    pub data: BuildInformationCommon,
}

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
#[can_message(0x616)]
pub struct RecoveryBoardBuildInfo {
    pub data: BuildInformationCommon,
}

/// # `FuelControlBoardBuildInfo`
/// Firmware build metadata emitted by the Fuel Control Board.
///
/// ## Type
/// - `Periodic @ 0.2 Hz`
///
/// ## Direction
/// - **Emitted by:** `FuelControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Allows version tracking and verification of the Fuel Control Board firmware.
///
/// ## Reaction
/// Receivers may log or display the firmware version and build metadata.
#[can_message(0x617)]
pub struct FuelControlBoardBuildInfo {
    pub data: BuildInformationCommon,
}

/// # `OxidizerControlBoardBuildInfo`
/// Firmware build metadata emitted by the Oxidizer Control Board.
///
/// ## Type
/// - `Periodic @ 0.2 Hz`
///
/// ## Direction
/// - **Emitted by:** `OxidizerControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Allows version tracking and verification of the Oxidizer Control Board firmware.
///
/// ## Reaction
/// Receivers may log or display the firmware version and build metadata.
#[can_message(0x618)]
pub struct OxidizerControlBoardBuildInfo {
    pub data: BuildInformationCommon,
}
