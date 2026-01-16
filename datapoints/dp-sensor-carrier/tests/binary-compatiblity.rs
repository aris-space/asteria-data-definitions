#![cfg(feature = "can")]
#![allow(clippy::bool_assert_comparison)] // nicer to read here
#![allow(clippy::float_cmp)] // we want binary compatiblity here

use data_core::can_hal::CanEncode;
use dp_sensor_carrier::*;

#[test]
fn basic_test() {
    let x = dp_sensor_carrier::Message::PressureData(datatypes::units::HPa(1013.25));

    let mut buf = [0; 64];
    let (id, len) = x.encode_into(&mut buf).expect("must encode");

    let x = hermes_can::messages::Message::try_from_parts(id, &buf[..(len as usize)])
        .expect("must decode");
    let hermes_can::messages::Message::PressureData(x) = x else {
        panic!("must be right variant");
    };
    assert_eq!(x.pressure, 1013.25);
}

// Helper macro to reduce boilerplate
macro_rules! test_binary_compat {
    ($test_name:ident, $new_msg:expr, $old_variant:pat => $assertions:expr) => {
        #[test]
        fn $test_name() {
            let new_msg = $new_msg;
            let mut buf = [0u8; 64];

            // Encode with new system
            let (id, len) = new_msg
                .encode_into(&mut buf)
                .expect("new system encoding must succeed");

            // Decode with old system
            let old_msg = hermes_can::messages::Message::try_from_parts(id, &buf[..(len as usize)])
                .expect("old system decoding must succeed");

            // Verify variant match and field values
            let $old_variant = old_msg else {
                panic!(
                    "Expected variant {:?}, got {:?}",
                    stringify!($old_variant),
                    old_msg
                );
            };
            $assertions
        }
    };
}

// Test compatibility of BuildInfo
test_binary_compat!(
    test_build_info,
    Message::BuildInfo(datatypes::status::BuildInformationCommon {
        unix_timestamp: 1_704_067_200, // 2024-01-01 00:00:00 UTC
        author_initials: [b'A', b'B'],
        is_release: true,
        debug_defmt_rtt: false,
        commit_hash: [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x11],
        is_git_dirty: false,
        can_semver: [2, 1, 0],
    }),
    hermes_can::messages::Message::SensorCarrierBuildInfo(decoded) => {
        assert_eq!(decoded.data.unix_timestamp, 1_704_067_200);
        assert_eq!(decoded.data.author_initials, [b'A', b'B']);
        assert_eq!(decoded.data.is_release, true);
        assert_eq!(decoded.data.debug_defmt_rtt, false);
        assert_eq!(decoded.data.commit_hash, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x11]);
        assert_eq!(decoded.data.is_git_dirty, false);
        assert_eq!(decoded.data.can_semver, [2, 1, 0]);
    }
);

// Test compatibility of BoardStatus - Pattern 1: Binary pattern 0b0101010101
// Each sensor alternates Online/Offline to detect adjacent field swaps
test_binary_compat!(
    test_board_status_pattern1,
    Message::BoardStatus(SensorCarrierStatus {
        common: datatypes::status::StatusCommonMessage {
            errors: 123,
            micros_since_restart: 987_654_321,
        },
        sensor_health: SensorsHealth {
            barometer_bus_1: datatypes::status::SensorStatus::Online,
            barometer_bus_2: datatypes::status::SensorStatus::Offline,
            dht_bus_1: datatypes::status::SensorStatus::Online,
            dht_bus_2: datatypes::status::SensorStatus::Offline,
            magnetometer_bus_1: datatypes::status::SensorStatus::Online,
            magnetometer_bus_2: datatypes::status::SensorStatus::Offline,
            imu_1: datatypes::status::SensorStatus::Online,
            imu_2: datatypes::status::SensorStatus::Offline,
            gps_1: datatypes::status::SensorStatus::Online,
            gps_2: datatypes::status::SensorStatus::Offline,
        },
    }),
    hermes_can::messages::Message::SensorCarrierStatus(decoded) => {
        assert_eq!(decoded.common.errors, 123);
        assert_eq!(decoded.common.micros_since_restart, 987_654_321);
        assert_eq!(decoded.sensor_health.barometer_bus_1, hermes_can::messages::board_status::SensorStatus::Online);
        assert_eq!(decoded.sensor_health.barometer_bus_2, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.dht_bus_1, hermes_can::messages::board_status::SensorStatus::Online);
        assert_eq!(decoded.sensor_health.dht_bus_2, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.magnetometer_bus_1, hermes_can::messages::board_status::SensorStatus::Online);
        assert_eq!(decoded.sensor_health.magnetometer_bus_2, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.imu_1, hermes_can::messages::board_status::SensorStatus::Online);
        assert_eq!(decoded.sensor_health.imu_2, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.gps_1, hermes_can::messages::board_status::SensorStatus::Online);
        assert_eq!(decoded.sensor_health.gps_2, hermes_can::messages::board_status::SensorStatus::Offline);
    }
);

// Test compatibility of BoardStatus - Pattern 2: Binary pattern 0b0011001100
// Groups of 2 bits help detect pair swaps
test_binary_compat!(
    test_board_status_pattern2,
    Message::BoardStatus(SensorCarrierStatus {
        common: datatypes::status::StatusCommonMessage {
            errors: 456,
            micros_since_restart: 111_222_333,
        },
        sensor_health: SensorsHealth {
            barometer_bus_1: datatypes::status::SensorStatus::Offline,
            barometer_bus_2: datatypes::status::SensorStatus::Offline,
            dht_bus_1: datatypes::status::SensorStatus::Online,
            dht_bus_2: datatypes::status::SensorStatus::Online,
            magnetometer_bus_1: datatypes::status::SensorStatus::Offline,
            magnetometer_bus_2: datatypes::status::SensorStatus::Offline,
            imu_1: datatypes::status::SensorStatus::Online,
            imu_2: datatypes::status::SensorStatus::Online,
            gps_1: datatypes::status::SensorStatus::Offline,
            gps_2: datatypes::status::SensorStatus::Offline,
        },
    }),
    hermes_can::messages::Message::SensorCarrierStatus(decoded) => {
        assert_eq!(decoded.common.errors, 456);
        assert_eq!(decoded.common.micros_since_restart, 111_222_333);
        assert_eq!(decoded.sensor_health.barometer_bus_1, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.barometer_bus_2, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.dht_bus_1, hermes_can::messages::board_status::SensorStatus::Online);
        assert_eq!(decoded.sensor_health.dht_bus_2, hermes_can::messages::board_status::SensorStatus::Online);
        assert_eq!(decoded.sensor_health.magnetometer_bus_1, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.magnetometer_bus_2, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.imu_1, hermes_can::messages::board_status::SensorStatus::Online);
        assert_eq!(decoded.sensor_health.imu_2, hermes_can::messages::board_status::SensorStatus::Online);
        assert_eq!(decoded.sensor_health.gps_1, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.gps_2, hermes_can::messages::board_status::SensorStatus::Offline);
    }
);

// Test compatibility of BoardStatus - Pattern 3: Binary pattern 0b0000011111
// First half off, second half on - detects position-dependent errors
test_binary_compat!(
    test_board_status_pattern3,
    Message::BoardStatus(SensorCarrierStatus {
        common: datatypes::status::StatusCommonMessage {
            errors: 789,
            micros_since_restart: 555_666_777,
        },
        sensor_health: SensorsHealth {
            barometer_bus_1: datatypes::status::SensorStatus::Offline,
            barometer_bus_2: datatypes::status::SensorStatus::Offline,
            dht_bus_1: datatypes::status::SensorStatus::Offline,
            dht_bus_2: datatypes::status::SensorStatus::Offline,
            magnetometer_bus_1: datatypes::status::SensorStatus::Offline,
            magnetometer_bus_2: datatypes::status::SensorStatus::Online,
            imu_1: datatypes::status::SensorStatus::Online,
            imu_2: datatypes::status::SensorStatus::Online,
            gps_1: datatypes::status::SensorStatus::Online,
            gps_2: datatypes::status::SensorStatus::Online,
        },
    }),
    hermes_can::messages::Message::SensorCarrierStatus(decoded) => {
        assert_eq!(decoded.common.errors, 789);
        assert_eq!(decoded.common.micros_since_restart, 555_666_777);
        assert_eq!(decoded.sensor_health.barometer_bus_1, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.barometer_bus_2, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.dht_bus_1, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.dht_bus_2, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.magnetometer_bus_1, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.magnetometer_bus_2, hermes_can::messages::board_status::SensorStatus::Online);
        assert_eq!(decoded.sensor_health.imu_1, hermes_can::messages::board_status::SensorStatus::Online);
        assert_eq!(decoded.sensor_health.imu_2, hermes_can::messages::board_status::SensorStatus::Online);
        assert_eq!(decoded.sensor_health.gps_1, hermes_can::messages::board_status::SensorStatus::Online);
        assert_eq!(decoded.sensor_health.gps_2, hermes_can::messages::board_status::SensorStatus::Online);
    }
);

// Test compatibility of BoardStatus - Pattern 4: Only one sensor online
// Uniquely identifies each field position
test_binary_compat!(
    test_board_status_pattern4,
    Message::BoardStatus(SensorCarrierStatus {
        common: datatypes::status::StatusCommonMessage {
            errors: 999,
            micros_since_restart: 888_999_000,
        },
        sensor_health: SensorsHealth {
            barometer_bus_1: datatypes::status::SensorStatus::Offline,
            barometer_bus_2: datatypes::status::SensorStatus::Offline,
            dht_bus_1: datatypes::status::SensorStatus::Offline,
            dht_bus_2: datatypes::status::SensorStatus::Offline,
            magnetometer_bus_1: datatypes::status::SensorStatus::Offline,
            magnetometer_bus_2: datatypes::status::SensorStatus::Offline,
            imu_1: datatypes::status::SensorStatus::Offline,
            imu_2: datatypes::status::SensorStatus::Online, // Only this one is online
            gps_1: datatypes::status::SensorStatus::Offline,
            gps_2: datatypes::status::SensorStatus::Offline,
        },
    }),
    hermes_can::messages::Message::SensorCarrierStatus(decoded) => {
        assert_eq!(decoded.common.errors, 999);
        assert_eq!(decoded.common.micros_since_restart, 888_999_000);
        assert_eq!(decoded.sensor_health.barometer_bus_1, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.barometer_bus_2, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.dht_bus_1, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.dht_bus_2, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.magnetometer_bus_1, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.magnetometer_bus_2, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.imu_1, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.imu_2, hermes_can::messages::board_status::SensorStatus::Online);
        assert_eq!(decoded.sensor_health.gps_1, hermes_can::messages::board_status::SensorStatus::Offline);
        assert_eq!(decoded.sensor_health.gps_2, hermes_can::messages::board_status::SensorStatus::Offline);
    }
);

// Test compatibility of PositionData
test_binary_compat!(
    test_position_data,
    Message::PositionData(PositionData {
        location_latitude: 47.376_887,
        location_longitude: 8.541_694,
        location_hamsl: 408.5,
        horizontal_accuracy: 2.5,
        vertical_accuracy: 5.0,
    }),
    hermes_can::messages::Message::PositionData(decoded) => {
        assert_eq!(decoded.location_latitude, 47.376_887);
        assert_eq!(decoded.location_longitude, 8.541_694);
        assert_eq!(decoded.location_hamsl, 408.5);
        assert_eq!(decoded.horizontal_accuracy, 2.5);
        assert_eq!(decoded.vertical_accuracy, 5.0);
    }
);

// Test compatibility of ImuData
test_binary_compat!(
    test_imu_data,
    Message::ImuData(ImuData {
        acceleration_x: 0.1,
        acceleration_y: 0.2,
        acceleration_z: 9.81,
        angular_velocity_x: 1.5,
        angular_velocity_y: -2.3,
        angular_velocity_z: 0.8,
        acceleration_north: 0.15,
        acceleration_east: 0.25,
        acceleration_down: 9.8,
        angular_velocity_north: 1.6,
        angular_velocity_east: -2.4,
        angular_velocity_down: 0.9,
    }),
    hermes_can::messages::Message::ImuData(decoded) => {
        assert_eq!(decoded.acceleration_x, 0.1);
        assert_eq!(decoded.acceleration_y, 0.2);
        assert_eq!(decoded.acceleration_z, 9.81);
        assert_eq!(decoded.angular_velocity_x, 1.5);
        assert_eq!(decoded.angular_velocity_y, -2.3);
        assert_eq!(decoded.angular_velocity_z, 0.8);
        assert_eq!(decoded.acceleration_north, 0.15);
        assert_eq!(decoded.acceleration_east, 0.25);
        assert_eq!(decoded.acceleration_down, 9.8);
        assert_eq!(decoded.angular_velocity_north, 1.6);
        assert_eq!(decoded.angular_velocity_east, -2.4);
        assert_eq!(decoded.angular_velocity_down, 0.9);
    }
);

// Test compatibility of VelocityData
test_binary_compat!(
    test_velocity_data,
    Message::VelocityData(VelocityData {
        velocity_x: 10.5,
        velocity_y: -5.2,
        velocity_z: 1.8,
        velocity_north: 12.3,
        velocity_east: -3.7,
        velocity_down: 2.1,
    }),
    hermes_can::messages::Message::VelocityData(decoded) => {
        assert_eq!(decoded.velocity_x, 10.5);
        assert_eq!(decoded.velocity_y, -5.2);
        assert_eq!(decoded.velocity_z, 1.8);
        assert_eq!(decoded.velocity_north, 12.3);
        assert_eq!(decoded.velocity_east, -3.7);
        assert_eq!(decoded.velocity_down, 2.1);
    }
);

// Test compatibility of OrientationData (identity quaternion)
test_binary_compat!(
    test_orientation_data_identity,
    Message::OrientationData(OrientationData {
        orientation_w: 1.0,
        orientation_x: 0.0,
        orientation_y: 0.0,
        orientation_z: 0.0,
    }),
    hermes_can::messages::Message::OrientationData(decoded) => {
        assert_eq!(decoded.orientation_w, 1.0);
        assert_eq!(decoded.orientation_x, 0.0);
        assert_eq!(decoded.orientation_y, 0.0);
        assert_eq!(decoded.orientation_z, 0.0);
    }
);

// Test compatibility of OrientationData (non-trivial quaternion)
test_binary_compat!(
    test_orientation_data_rotated,
    Message::OrientationData(OrientationData {
        orientation_w: 0.707,
        orientation_x: 0.0,
        orientation_y: 0.707,
        orientation_z: 0.0,
    }),
    hermes_can::messages::Message::OrientationData(decoded) => {
        assert_eq!(decoded.orientation_w, 0.707);
        assert_eq!(decoded.orientation_x, 0.0);
        assert_eq!(decoded.orientation_y, 0.707);
        assert_eq!(decoded.orientation_z, 0.0);
    }
);

// Test compatibility of EnvironmentalData
test_binary_compat!(
    test_environmental_data,
    Message::EnvironmentalData(EnvironmentalData {
        temperature: datatypes::units::Celsius(23.5),
        humidity: 65.2,
        pressure: datatypes::units::HPa(1013.25),
    }),
    hermes_can::messages::Message::EnvironmentalData(decoded) => {
        assert_eq!(decoded.temperature, 23.5);
        assert_eq!(decoded.humidity, 65.2);
        assert_eq!(decoded.pressure, 1013.25);
    }
);

// Test compatibility of MagnetometerData
test_binary_compat!(
    test_magnetometer_data,
    Message::MagnetometerData(MagnetometerData {
        magnetic_field_x: 25.3,
        magnetic_field_y: -10.8,
        magnetic_field_z: 42.1,
        magnetic_field_north: 27.5,
        magnetic_field_east: -12.3,
        magnetic_field_down: 43.8,
    }),
    hermes_can::messages::Message::MagnetometerData(decoded) => {
        assert_eq!(decoded.magnetic_field_x, 25.3);
        assert_eq!(decoded.magnetic_field_y, -10.8);
        assert_eq!(decoded.magnetic_field_z, 42.1);
        assert_eq!(decoded.magnetic_field_north, 27.5);
        assert_eq!(decoded.magnetic_field_east, -12.3);
        assert_eq!(decoded.magnetic_field_down, 43.8);
    }
);

// Test compatibility of PressureData
test_binary_compat!(
    test_pressure_data,
    Message::PressureData(datatypes::units::HPa(1013.25)),
    hermes_can::messages::Message::PressureData(decoded) => {
        assert_eq!(decoded.pressure, 1013.25);
    }
);
