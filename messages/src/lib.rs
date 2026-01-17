/// All messages in the full system
#[gated_derive::gated_derive(
    data_core::CanDecode(feature = "can"),
    data_core::CanEncode(feature = "can")
)]
// #[derive(MavlinkDialect)] // TODO
// #[dialect(name = "ASTERIA", id = 900, version = 1)] // idk whether 900 is good
#[can(dispatch)]
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

// TODO
// const _: () = mavlink_hal::macro_util::check_unique_ids(
//     <AnyMessage as mavlink_hal::description::DescribeMessages>::MESSAGES,
// );
