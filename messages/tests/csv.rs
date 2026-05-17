//! This test file checks proper CSV/Influx serialization of messages

use datapoint_information::influx;
use datatypes::{actuator::NormallyOpenValve, status::StatusCommonMessage};
use dp_flight_computer::FlightComputerStatus;
use messages::Message;

#[test]
fn log_fuel_vent_valve() {
    let msg = Message::FuelControlBoard(dp_fuel_control_board::Message::FuelVentValveState(
        NormallyOpenValve::Open,
    ));
    let open = influx::to_datapoint(&msg, 42).expect("must serialize");
    assert_eq!(open.measurement, "valve_state");
    assert_eq!(
        open.tags.get("sensor_id").expect("must have sensor id"),
        "fss_tnk_vnt"
    );
    assert!(matches!(
        open.fields.get("open").expect("must have open field"),
        influx::FieldValue::Bool(true)
    ));
    let msg = Message::FuelControlBoard(dp_fuel_control_board::Message::FuelVentValveState(
        NormallyOpenValve::Closed,
    ));

    let closed = influx::to_datapoint(&msg, 42).expect("must serialize");
    assert_eq!(closed.measurement, "valve_state");
    assert_eq!(
        closed.tags.get("sensor_id").expect("must have sensor id"),
        "fss_tnk_vnt"
    );
    assert!(matches!(
        closed.fields.get("open").expect("must have open field"),
        influx::FieldValue::Bool(false)
    ));
}

#[test]
fn log_fc_status() {
    let msg = Message::FlightComputer(dp_flight_computer::Message::Status(FlightComputerStatus {
        common: StatusCommonMessage {
            errors: 42,
            micros_since_restart: 99999,
        },
        has_pss_control: false,
        is_software_armed: datatypes::status::ArmingState::Safe,
    }));
    let status = influx::to_datapoint(&msg, 42).expect("must serialize");
    assert_eq!(status.measurement, "FlightComputerStatus");
    assert!(matches!(
        status
            .fields
            .get("common.errors")
            .expect("must have error field"),
        influx::FieldValue::I64(42)
    ));
    let influx::FieldValue::String(val) = status
        .fields
        .get("is_software_armed")
        .expect("must have armed field")
    else {
        panic!("must be a string");
    };
    assert_eq!(val.as_ref(), "Safe");
}

#[test]
fn log_unit_message() {
    let msg = Message::RecoveryBoard(dp_recovery_board::Message::SeparationOccurred);
    let sep = influx::to_datapoint(&msg, 42).expect("must serialize");
    dbg!(sep); // TODO: fails
}

#[test]
fn log_enum_message() {
    let msg =
        Message::EngineControlBoard(dp_engine_control_board::Message::FuelMainValveControlFC(
            datatypes::actuator::NormallyClosedValve::Open,
        ));
    let cmd = influx::to_datapoint(&msg, 42).expect("must serialize");

    assert_eq!(cmd.measurement, "NormallyClosedValve");
    let influx::FieldValue::String(val) = cmd.fields.get("value").expect("must have value field")
    else {
        panic!("must be a string");
    };
    assert_eq!(val.as_ref(), "Open");
}
