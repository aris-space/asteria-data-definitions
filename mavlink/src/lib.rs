//! # Hermes MAVLink Crate
//!
//! This crate provides mavlink-message implementations for the Hermes project, generated from
//! `hermes.xml` using `mavlink-bindgen`.
//!
//! `hermes.xml` defines a custom MAVLink dialect for the Hermes project, which is used to communicate
//! between the Ground Station(s), RFS and the Flight Computer.
//!
//! The messages may be encoded into both MAVLink v1 and v2 packets, though we only use v2
//! in the Hermes project. A mavlink connection may be established using the `mavlink` crate.
//!
//! ## Features
//!
//! - `hermes`: Enables the custom Hermes MAVLink dialect.
//! - `serde`: Enables serialization/deserialization of messages using `serde`.
//! - `examples`: Internal. Enables a custom dialect used in examples.
//!
//! ## Examples
//!
//! Refer to the following examples for usage:
//! - **Encoding/Decoding**: [`examples/encode_decode.rs`](../examples/encode_decode.rs)
//! - **Sending/Receiving**: [`examples/send_receive.rs`](../examples/send_receive.rs)
//!
//! ## Build Process
//!
//! The custom MAVLink bindings are generated from `hermes.xml` at build time using `mavlink-bindgen`.

include!(concat!(env!("OUT_DIR"), "/mod.rs"));

#[cfg(feature = "hermes")]
pub use hermes::*;

#[test]
#[allow(clippy::assertions_on_constants)]
fn assert_sizes() {
    // This is here to catch further additions such that they can be added to the assert list below.
    #[allow(dead_code)]
    fn is_complete(msg: &hermes::MavMessage) {
        match msg {
            MavMessage::COMMAND_MANUAL_TRANSITION(_) => {}
            MavMessage::COMMAND_MANUAL_ACTUATOR(_) => {}

            MavMessage::CURRENT_STATE(_) => {}
            MavMessage::BUILD_INFORMATION(_) => {}
            MavMessage::CONFIG_INFORMATION(_) => {}
            MavMessage::FC_STATUS(_) => {}
            MavMessage::RFS_STATUS(_) => {}

            MavMessage::VELOCITY(_) => {}
            MavMessage::ORIENTATION(_) => {}
            MavMessage::ACCELERATION(_) => {}
            MavMessage::LOCATION(_) => {}

            MavMessage::RAIL_STATUS_24V(_) => {}
            MavMessage::RAIL_STATUS_5V(_) => {}
            MavMessage::ENVIRONMENTAL_DATA(_) => {}

            MavMessage::RFS_PRESSURE(_) => {}
            MavMessage::RFS_TEMPERATURE(_) => {}

            MavMessage::PSS_PRESSURE(_) => {}
            MavMessage::PSS_TEMPERATURE(_) => {}
            MavMessage::PSS_LEVEL(_) => {}

            MavMessage::STEERING_ACTUAL_POSITIONS(_) => {}
            MavMessage::PSS_ACTUATOR_STATES(_) => {}
            MavMessage::RFS_ACTUATORS_STATES(_) => {}
            MavMessage::RFS_OUTPUTS_STATES(_) => {}
        }
    }

    const MAX_SIZE: usize = 60;

    assert!(hermes::COMMAND_MANUAL_TRANSITION_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::COMMAND_MANUAL_ACTUATOR_DATA::ENCODED_LEN <= MAX_SIZE);

    assert!(hermes::CURRENT_STATE_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::BUILD_INFORMATION_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::CONFIG_INFORMATION_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::FC_STATUS_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::RFS_STATUS_DATA::ENCODED_LEN <= MAX_SIZE);

    assert!(hermes::LOCATION_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::VELOCITY_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::ACCELERATION_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::ORIENTATION_DATA::ENCODED_LEN <= MAX_SIZE);

    assert!(hermes::RAIL_STATUS_24V_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::RAIL_STATUS_5V_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::ENVIRONMENTAL_DATA_DATA::ENCODED_LEN <= MAX_SIZE);

    assert!(hermes::RFS_PRESSURE_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::RFS_TEMPERATURE_DATA::ENCODED_LEN <= MAX_SIZE);

    assert!(hermes::PSS_PRESSURE_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::PSS_TEMPERATURE_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::PSS_LEVEL_DATA::ENCODED_LEN <= MAX_SIZE);

    assert!(hermes::STEERING_ACTUAL_POSITIONS_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::PSS_ACTUATOR_STATES_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::RFS_ACTUATORS_STATES_DATA::ENCODED_LEN <= MAX_SIZE);
    assert!(hermes::RFS_OUTPUTS_STATES_DATA::ENCODED_LEN <= MAX_SIZE);
}
