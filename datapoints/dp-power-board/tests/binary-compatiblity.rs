//! These tests check binary compatiblity with the hermes-can library 0.3.0.
//! They can be removed once a new major version is released
#![cfg(feature = "can")]
#![allow(clippy::bool_assert_comparison)] // nicer to read here
#![allow(clippy::float_cmp)] // we want binary compatiblity here

use data_core::can_hal::CanEncode;
use dp_power_board::*;

#[test]
fn basic_test() {
    let x = dp_power_board::Message::RailStatus5V(datatypes::units::RailStatus {
        voltage: 5.1,
        current: 2.3,
        power: 11.73,
    });

    let mut buf = [0; 64];
    let (id, len) = x.encode_into(&mut buf).expect("must encode");

    let x = hermes_can::messages::Message::try_from_parts(id, &buf[..(len as usize)])
        .expect("must decode");
    let hermes_can::messages::Message::RailStatus5V(x) = x else {
        panic!("must be right variant");
    };
    assert_eq!(x.data.voltage, 5.1);
    assert_eq!(x.data.current, 2.3);
    assert_eq!(x.data.power, 11.73);
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
        author_initials: [b'P', b'B'],
        is_release: true,
        debug_defmt_rtt: false,
        commit_hash: [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77],
        is_git_dirty: false,
        can_semver: [1, 0, 0],
    }),
    hermes_can::messages::Message::PowerBoardBuildInfo(decoded) => {
        assert_eq!(decoded.data.unix_timestamp, 1_704_067_200);
        assert_eq!(decoded.data.author_initials, [b'P', b'B']);
        assert_eq!(decoded.data.is_release, true);
        assert_eq!(decoded.data.debug_defmt_rtt, false);
        assert_eq!(decoded.data.commit_hash, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77]);
        assert_eq!(decoded.data.is_git_dirty, false);
        assert_eq!(decoded.data.can_semver, [1, 0, 0]);
    }
);

// Test compatibility of BoardStatus
test_binary_compat!(
    test_board_status,
    Message::BoardStatus(datatypes::status::StatusCommonMessage {
        errors: 42,
        micros_since_restart: 123_456_789,
    }),
    hermes_can::messages::Message::PowerBoardStatus(decoded) => {
        assert_eq!(decoded.common.errors, 42);
        assert_eq!(decoded.common.micros_since_restart, 123_456_789);
    }
);

// Test compatibility of RailStatus5V
// Distinct values ensure voltage/current/power aren't swapped
test_binary_compat!(
    test_rail_status_5v,
    Message::RailStatus5V(datatypes::units::RailStatus {
        voltage: 5.25,
        current: 2.75,
        power: 14.4375,
    }),
    hermes_can::messages::Message::RailStatus5V(decoded) => {
        assert_eq!(decoded.data.voltage, 5.25);
        assert_eq!(decoded.data.current, 2.75);
        assert_eq!(decoded.data.power, 14.4375);
    }
);

// Test compatibility of RailStatus24V
// Distinct values ensure voltage/current/power aren't swapped
test_binary_compat!(
    test_rail_status_24v,
    Message::RailStatus24V(datatypes::units::RailStatus {
        voltage: 25.2,
        current: 4.8,
        power: 120.96,
    }),
    hermes_can::messages::Message::RailStatus24V(decoded) => {
        assert_eq!(decoded.data.voltage, 25.2);
        assert_eq!(decoded.data.current, 4.8);
        assert_eq!(decoded.data.power, 120.96);
    }
);

// Test compatibility of CameraPowerControl - Pattern 1: (0, 1)
test_binary_compat!(
    test_camera_power_control_01,
    Message::CameraPowerControl(datatypes::actuator::CameraPowerStatus {
        recording_pwr_enabled: false,
        live_pwr_enabled: true,
    }),
    hermes_can::messages::Message::CameraPowerControl(decoded) => {
        assert_eq!(decoded.recording_pwr_enabled, false);
        assert_eq!(decoded.live_pwr_enabled, true);
    }
);

// Test compatibility of CameraPowerControl - Pattern 2: (1, 0)
test_binary_compat!(
    test_camera_power_control_10,
    Message::CameraPowerControl(datatypes::actuator::CameraPowerStatus {
        recording_pwr_enabled: true,
        live_pwr_enabled: false,
    }),
    hermes_can::messages::Message::CameraPowerControl(decoded) => {
        assert_eq!(decoded.recording_pwr_enabled, true);
        assert_eq!(decoded.live_pwr_enabled, false);
    }
);

// Test compatibility of CameraPowerStatus - Pattern 1: (0, 1)
test_binary_compat!(
    test_camera_power_status_01,
    Message::CameraPowerStatus(datatypes::actuator::CameraPowerStatus {
        recording_pwr_enabled: false,
        live_pwr_enabled: true,
    }),
    hermes_can::messages::Message::CameraPowerStatus(decoded) => {
        assert_eq!(decoded.recording_pwr_enabled, false);
        assert_eq!(decoded.live_pwr_enabled, true);
    }
);

// Test compatibility of CameraPowerStatus - Pattern 2: (1, 0)
test_binary_compat!(
    test_camera_power_status_10,
    Message::CameraPowerStatus(datatypes::actuator::CameraPowerStatus {
        recording_pwr_enabled: true,
        live_pwr_enabled: false,
    }),
    hermes_can::messages::Message::CameraPowerStatus(decoded) => {
        assert_eq!(decoded.recording_pwr_enabled, true);
        assert_eq!(decoded.live_pwr_enabled, false);
    }
);
