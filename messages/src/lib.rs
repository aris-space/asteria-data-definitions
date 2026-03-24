//! This crate provides [`Message`], an enum with all the messages in the system.
//!
//! It is also used to check IDs are unique and to generate the full mavlink dialect.
//!
//! Like other crates, various cargo features can enable certain components,
//! currently `can` and `mavlink`.
//!
//! ## MavLink Messages
//! The generated MavLink dialect is a MavLink V2 dialect. This means that the message id's
//! are 24 bit (or 6 hex digits).
//! The id's follow the following schema (from MSB to LSB):
//! - Hex digits 0,1: `01` for downlink messages and `02` for uplink messages.
//! - Hex digits 2,3: `xx` where `xx` is the ascending id for a specific board/message enum.
//! - Hex digits 4,5: `xx` where `xx` is the ascending message number for the given board. Starts at `00` for every board/message enum.

use gated_derive::gated_derive;

#[cfg(feature = "can")]
use data_core::can;
#[cfg(feature = "mavlink")]
use data_core::mavlink;

/// Reexport all datapoints to simplify imports
pub mod datapoints {
    pub use dp_engine_control_board;
    pub use dp_flight_computer;
    pub use dp_fuel_control_board;
    pub use dp_oxidizer_control_board;
    pub use dp_power_board;
    pub use dp_recovery_board;
    pub use dp_rfs_computer;
    pub use dp_sensor_carrier;
    pub use dp_system_management;
}

/// All messages in the full system
#[gated_derive(can::Decode, can::Encode, mavlink::Dialect)]
#[gated_derive(utoipa::ToSchema, serde::Deserialize, serde::Serialize, Debug)]
#[dialect(name = "ASTERIA", id = 900, version = 1)]
#[can(dispatch)]
#[serde(tag = "board", content = "message")]
pub enum Message {
    FlightComputer(dp_flight_computer::Message),
    RfsComputer(dp_rfs_computer::Message),
    SystemManagement(dp_system_management::Message),
    RecoveryBoard(dp_recovery_board::Message),
    PowerBoard(dp_power_board::Message),
    SensorCarrier(dp_sensor_carrier::Message),
    FuelControlBoard(dp_fuel_control_board::Message),
    OxidizerControlBoard(dp_oxidizer_control_board::Message),
    EngineControlBoard(dp_engine_control_board::Message),
}

/// Test to assert that all CAN IDs are unique
#[cfg(feature = "can")]
#[test]
fn unique_can_ids() {
    let mut all_ids = [
        &dp_flight_computer::Message::ALL_CAN_IDS[..],
        &dp_rfs_computer::Message::ALL_CAN_IDS[..],
        &dp_system_management::Message::ALL_CAN_IDS[..],
        &dp_recovery_board::Message::ALL_CAN_IDS[..],
        &dp_power_board::Message::ALL_CAN_IDS[..],
        &dp_sensor_carrier::Message::ALL_CAN_IDS[..],
        &dp_fuel_control_board::Message::ALL_CAN_IDS[..],
        &dp_oxidizer_control_board::Message::ALL_CAN_IDS[..],
        &dp_engine_control_board::Message::ALL_CAN_IDS[..],
    ]
    .concat();

    let nr_msg = all_ids.len();
    all_ids.sort();
    all_ids.dedup();
    assert_eq!(nr_msg, all_ids.len(), "contains duplicate IDs!");
}
