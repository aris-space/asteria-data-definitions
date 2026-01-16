//! These tests check binary compatiblity with the hermes-can library 0.3.0.
//! They can be removed once a new major version is released
#![cfg(feature = "can")]
#![allow(clippy::bool_assert_comparison)] // nicer to read here

use data_core::can_hal::CanEncode;
use dp_recovery_board::*;

#[test]
fn basic_test() {
    let x = dp_recovery_board::Message::SteeringActualPositions(SteeringPositions {
        left_pos: 42,
        right_pos: -1,
    });

    let mut buf = [0; 64];
    let (id, len) = x.encode_into(&mut buf).expect("must encode");

    let x = hermes_can::messages::Message::try_from_parts(id, &buf[..(len as usize)])
        .expect("must decode");
    let hermes_can::messages::Message::SteeringActualPositions(x) = x else {
        panic!("must be right variant");
    };
    assert_eq!(x.left_pos, 42);
    assert_eq!(x.right_pos, -1);
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

// Test compatiblity of SteeringActualPositions
test_binary_compat!(
    test_steering_actual_positions,
    Message::SteeringActualPositions(SteeringPositions {
        left_pos: 42,
        right_pos: -1,
    }),
    hermes_can::messages::Message::SteeringActualPositions(decoded) => {
        assert_eq!(decoded.left_pos, 42);
        assert_eq!(decoded.right_pos, -1);
    }
);

// Test compatiblity of SteeringTargetPositions
test_binary_compat!(
    test_steering_target_positions,
    Message::SteeringTargetPositions(SteeringPositions {
        left_pos: 100,
        right_pos: -50,
    }),
    hermes_can::messages::Message::SteeringTargetPositions(decoded) => {
        assert_eq!(decoded.left_pos, 100);
        assert_eq!(decoded.right_pos, -50);
    }
);

// Test compatiblity of RecoveryPowerConfig
test_binary_compat!(
    test_recovery_power_config1,
    Message::RecoveryPowerConfig(RecoveryPowerConfig {
        separation_enabled: true,
        deployment_enabled: false,
        steering_enabled: false,
    }),
    hermes_can::messages::Message::RecoveryPowerConfig(decoded) => {
        assert_eq!(decoded.separation_enabled, true);
        assert_eq!(decoded.deployment_enabled, false);
        assert_eq!(decoded.steering_enabled, false);
    }
);
test_binary_compat!(
    test_recovery_power_config2,
    Message::RecoveryPowerConfig(RecoveryPowerConfig {
        separation_enabled: false,
        deployment_enabled: false,
        steering_enabled: true,
    }),
    hermes_can::messages::Message::RecoveryPowerConfig(decoded) => {
        assert_eq!(decoded.separation_enabled, false);
        assert_eq!(decoded.deployment_enabled, false);
        assert_eq!(decoded.steering_enabled, true);
    }
);
test_binary_compat!(
    test_recovery_power_config3,
    Message::RecoveryPowerConfig(RecoveryPowerConfig {
        separation_enabled: false,
        deployment_enabled: true,
        steering_enabled: false,
    }),
    hermes_can::messages::Message::RecoveryPowerConfig(decoded) => {
        assert_eq!(decoded.separation_enabled, false);
        assert_eq!(decoded.deployment_enabled, true);
        assert_eq!(decoded.steering_enabled, false);
    }
);

// Test compatiblity of SeparationTrigger (unit variant)
test_binary_compat!(
    test_separation_trigger,
    Message::SeparationTrigger,
    hermes_can::messages::Message::SeparationTrigger(_decoded) => {}
);

// Test compatiblity of DeploymentTrigger (unit variant)
test_binary_compat!(
    test_deployment_trigger,
    Message::DeploymentTrigger,
    hermes_can::messages::Message::DeploymentTrigger(_decoded) => {}
);

// Test compatiblity of SeparationOccurred (unit variant)
test_binary_compat!(
    test_separation_occurred,
    Message::SeparationOccurred,
    hermes_can::messages::Message::SeparationOccurred(_decoded) => {}
);

// Test compatiblity of DeploymentOccurred (unit variant)
test_binary_compat!(
    test_deployment_occurred,
    Message::DeploymentOccurred,
    hermes_can::messages::Message::DeploymentOccurred(_decoded) => {}
);

// Test compatibility of BuildInfo
test_binary_compat!(
    test_build_info,
    Message::BuildInfo(datatypes::status::BuildInformationCommon {
        unix_timestamp: 1_704_067_200, // 2024-01-01 00:00:00 UTC
        author_initials: [b'C', b'S'],
        is_release: true,
        debug_defmt_rtt: false,
        commit_hash: [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE],
        is_git_dirty: false,
        can_semver: [1, 2, 3],
    }),
    hermes_can::messages::Message::RecoveryBoardBuildInfo(decoded) => {
        assert_eq!(decoded.data.unix_timestamp, 1_704_067_200);
        assert_eq!(decoded.data.author_initials, [b'C', b'S']);
        assert_eq!(decoded.data.is_release, true);
        assert_eq!(decoded.data.debug_defmt_rtt, false);
        assert_eq!(decoded.data.commit_hash, [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE]);
        assert_eq!(decoded.data.is_git_dirty, false);
        assert_eq!(decoded.data.can_semver, [1, 2, 3]);
    }
);

// Test compatibility of BoardStatus
test_binary_compat!(
    test_board_status1,
    Message::BoardStatus(RecoveryBoardStatus {
        common: datatypes::status::StatusCommonMessage {
            errors: 42,
            micros_since_restart: 123_456_780,
        },
        sep1_status: ActuatorStatus::Connected,
        sep2_status: ActuatorStatus::PowerOn,
        depl1_status: ActuatorStatus::NotConnected,
        depl2_status: ActuatorStatus::Connected,
        steering_general: ActuatorStatus::PowerOn,
        steering_left_connected: true,
        steering_right_connected: false,
        steering_watchdog_status: WatchdogState::Active,
        arming_state: datatypes::status::ArmingState::Armed,
    }),
    hermes_can::messages::Message::RecoveryBoardStatus(decoded) => {
        assert_eq!(decoded.common.errors, 42);
        assert_eq!(decoded.common.micros_since_restart, 123_456_780);
        assert_eq!(decoded.sep1_status, hermes_can::messages::board_status::ActuatorStatus::Connected);
        assert_eq!(decoded.sep2_status, hermes_can::messages::board_status::ActuatorStatus::PowerOn);
        assert_eq!(decoded.depl1_status, hermes_can::messages::board_status::ActuatorStatus::NotConnected);
        assert_eq!(decoded.depl2_status, hermes_can::messages::board_status::ActuatorStatus::Connected);
        assert_eq!(decoded.steering_general, hermes_can::messages::board_status::ActuatorStatus::PowerOn);
        assert_eq!(decoded.steering_left_connected, true);
        assert_eq!(decoded.steering_right_connected, false);
        assert_eq!(decoded.steering_watchdog_status, hermes_can::messages::board_status::WatchdogState::Active);
        assert_eq!(decoded.arming_state, hermes_can::messages::board_status::ArmingState::Armed);
    }
);

// Test compatibility of BoardStatus
test_binary_compat!(
    test_board_status2,
    Message::BoardStatus(RecoveryBoardStatus {
        common: datatypes::status::StatusCommonMessage {
            errors: 938,
            micros_since_restart: 123_456_789,
        },
        sep1_status: ActuatorStatus::PowerOn,
        sep2_status: ActuatorStatus::Connected,
        depl1_status: ActuatorStatus::PowerOn,
        depl2_status: ActuatorStatus::NotConnected,
        steering_general: ActuatorStatus::Connected,
        steering_left_connected: false,
        steering_right_connected: true,
        steering_watchdog_status: WatchdogState::Expired,
        arming_state: datatypes::status::ArmingState::Safe,
    }),
    hermes_can::messages::Message::RecoveryBoardStatus(decoded) => {
        assert_eq!(decoded.common.errors, 938);
        assert_eq!(decoded.common.micros_since_restart, 123_456_789);
        assert_eq!(decoded.sep1_status, hermes_can::messages::board_status::ActuatorStatus::PowerOn);
        assert_eq!(decoded.sep2_status, hermes_can::messages::board_status::ActuatorStatus::Connected);
        assert_eq!(decoded.depl1_status, hermes_can::messages::board_status::ActuatorStatus::PowerOn);
        assert_eq!(decoded.depl2_status, hermes_can::messages::board_status::ActuatorStatus::NotConnected);
        assert_eq!(decoded.steering_general, hermes_can::messages::board_status::ActuatorStatus::Connected);
        assert_eq!(decoded.steering_left_connected, false);
        assert_eq!(decoded.steering_right_connected, true);
        assert_eq!(decoded.steering_watchdog_status, hermes_can::messages::board_status::WatchdogState::Expired);
        assert_eq!(decoded.arming_state, hermes_can::messages::board_status::ArmingState::Safe);
    }
);
