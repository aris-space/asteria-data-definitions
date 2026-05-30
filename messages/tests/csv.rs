//! This test file checks proper CSV/Influx serialization of messages

use datapoint_information::influx;
use datatypes::actuator::NormallyOpenValve;
use datatypes::status::{StatusCommonMessage, UplinkTelemetryHealth};
use datatypes::units::{Celsius, HPa, InstantUs};
use dp_flight_computer::FlightComputerStatus;
use messages::Message;

#[test]
fn log_fuel_vent_valve() {
    let msg = Message::FuelControlBoard(dp_fuel_control_board::Message::FuelVentValveState(
        NormallyOpenValve::Open,
    ));
    let open = influx::to_datapoint(&msg).expect("must serialize");
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

    let closed = influx::to_datapoint(&msg).expect("must serialize");
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
    let status = influx::to_datapoint(&msg).expect("must serialize");
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

/// It broke before because `InstantUs` was not repr transparent
#[test]
fn log_uplink_telemetry_health() {
    let msg = Message::FlightComputer(dp_flight_computer::Message::UplinkTelemetryHealth(
        UplinkTelemetryHealth {
            time_of_last_msg: InstantUs(23232),
            num_msg_received: 123,
            num_msg_lost: 12,
            time_of_last_processed_msg: InstantUs(10200),
            num_msg_processed: 8,
        },
    ));
    let status = influx::to_datapoint(&msg).expect("must serialize");
    eprintln!("{status:?}");
}

#[test]
fn log_unit_message() {
    let msg = Message::RecoveryBoard(dp_recovery_board::Message::SeparationOccurred);
    let sep = influx::to_datapoint(&msg).expect("must serialize");

    assert_eq!(sep.measurement, "event");
    assert_eq!(
        sep.tags.get("message").expect("must have message tag"),
        "SeparationOccurred"
    );
    assert!(matches!(
        sep.fields.get("happened").expect("must have dummy field"),
        influx::FieldValue::Bool(true)
    ));
}

#[test]
#[allow(clippy::float_cmp)]
fn log_newtype_message() {
    let msg = Message::SensorCarrier(dp_sensor_carrier::Message::PressureData(HPa(100.0)));
    let dp = influx::to_datapoint(&msg).expect("must serialize");

    assert_eq!(dp.measurement, "HPa");
    let influx::FieldValue::F64(val) = dp.fields.get("value").expect("must have value field")
    else {
        panic!("must be a float");
    };
    assert_eq!(
        dp.tags.get("message").expect("must have message tag"),
        "PressureData"
    );
    assert_eq!(*val, 100.);
}

#[test]
#[allow(clippy::float_cmp)]
fn log_newtype_in_struct() {
    let msg = Message::SensorCarrier(dp_sensor_carrier::Message::EnvironmentalData(
        dp_sensor_carrier::EnvironmentalData {
            temperature: Celsius(20.),
            humidity: 50.,
            pressure: HPa(100.0),
        },
    ));
    let dp = influx::to_datapoint(&msg).expect("must serialize");

    assert_eq!(dp.measurement, "EnvironmentalData");
    let influx::FieldValue::F64(pressure) =
        dp.fields.get("pressure").expect("must have pressure field")
    else {
        panic!("must be a float");
    };
    assert_eq!(
        dp.tags.get("board").expect("must have message tag"),
        "SensorCarrier"
    );
    assert_eq!(*pressure, 100.);
}

#[test]
fn log_enum_message() {
    let msg =
        Message::EngineControlBoard(dp_engine_control_board::Message::FuelMainValveControlFC(
            datatypes::actuator::NormallyClosedValve::Open,
        ));
    let cmd = influx::to_datapoint(&msg).expect("must serialize");

    assert_eq!(cmd.measurement, "NormallyClosedValve");
    let influx::FieldValue::String(val) = cmd.fields.get("value").expect("must have value field")
    else {
        panic!("must be a string");
    };
    assert_eq!(val.as_ref(), "Open");
}
