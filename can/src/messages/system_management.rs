//! ## CAN Message Group: System Management
//!
//! - **ID Range:** `0x000–0x0FF`
//! - **Count:** 256 messages
//! - **Message Class:** System Management
//! - **Priority:** Very High
//! - **Type:** Event-based
//!
//! Global commands like resets, aborts, or watchdog triggers.

use crate::messages::BoardId;
use hermes_can_derive::can_message;

/// # `ResetAll`
/// Initiates a system-wide reset on all nodes.
///
/// ## Type
/// - `Event`: triggered by user command or fault condition
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`, `RfsComputer`
/// - **Received by:** All nodes except `FlightComputer` and `RfsComputer`
///
/// ## Purpose
/// Forces an immediate reset of all boards for system recovery or coordination.
///
/// ## Reaction
/// All boards perform a full software reset immediately on reception.
#[can_message(0x10)]
pub struct ResetAll;

/// # `ResetSpecific`
/// Instructs a specific board to reset.
///
/// ## Type
/// - `Event`: triggered by user command or fault condition
///
/// ## Direction
/// - **Emitted by:** `FlightComputer`
/// - **Received by:** All nodes except `FlightComputer` and `RfsComputer`
///
/// ## Purpose
/// Allows targeted recovery or restart of a malfunctioning board.
///
/// ## Reaction
/// The addressed board performs an immediate reset. Others shall ignore the message.
#[can_message(0x20)]
pub struct ResetSpecific {
    /// The id of the board to reset.
    pub board_id: BoardId,
}

/// # `UTCTimeUpdate`
/// Synchronizes system time using GPS-derived UTC.
///
/// ## Type
/// - `Periodic @ 10 Hz`
///
/// ## Direction
/// - **Emitted by:** `SensorCarrier`
/// - **Received by:** `All nodes`
///
/// ## Purpose
/// Ensures consistent timekeeping across all nodes using GPS as a source of truth.
///
/// ## Reaction
/// Boards shall update their internal time reference used for logging and
/// time-sensitive operations *once at startup*. While this value could in theory
/// be used to correct for drift, this may result in non-monotonic timekeeping
/// and is therefore discouraged.
#[can_message(0x30)]
pub struct UTCTimeUpdate {
    /// UTC timestamp given in microseconds since the Unix epoch (`1970-01-01T00:00:00Z`).
    ///
    /// This value is derived from the GPS time sensors and is therefore highly accurate.
    pub timestamp_micros: u64,
}
