#![cfg(feature = "can")]
#![allow(clippy::bool_assert_comparison)] // nicer to read here

use data_core::can_hal::CanEncode;
use dp_system_management::*;

#[test]
fn basic_test() {
    let x = dp_system_management::Message::ResetAll;

    let mut buf = [0; 64];
    let (id, len) = x.encode_into(&mut buf).expect("must encode");

    let x = hermes_can::messages::Message::try_from_parts(id, &buf[..(len as usize)])
        .expect("must decode");
    let hermes_can::messages::Message::ResetAll(_) = x else {
        panic!("must be right variant");
    };
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

// Test compatibility of ResetAll (unit variant)
test_binary_compat!(
    test_reset_all,
    Message::ResetAll,
    hermes_can::messages::Message::ResetAll(_decoded) => {}
);

// Test compatibility of ResetSpecific - FlightComputer
test_binary_compat!(
    test_reset_specific_flight_computer,
    Message::ResetSpecific(datatypes::status::BoardId::FlightComputer),
    hermes_can::messages::Message::ResetSpecific(decoded) => {
        assert_eq!(decoded.board_id, hermes_can::messages::BoardId::FlightComputer);
    }
);

// Test compatibility of ResetSpecific - PowerBoard
test_binary_compat!(
    test_reset_specific_power_board,
    Message::ResetSpecific(datatypes::status::BoardId::PowerBoard),
    hermes_can::messages::Message::ResetSpecific(decoded) => {
        assert_eq!(decoded.board_id, hermes_can::messages::BoardId::PowerBoard);
    }
);

// Test compatibility of ResetSpecific - RecoveryBoard
test_binary_compat!(
    test_reset_specific_recovery_board,
    Message::ResetSpecific(datatypes::status::BoardId::RecoveryBoard),
    hermes_can::messages::Message::ResetSpecific(decoded) => {
        assert_eq!(decoded.board_id, hermes_can::messages::BoardId::RecoveryBoard);
    }
);

// Test compatibility of ResetSpecific - OxidizerControlBoard
test_binary_compat!(
    test_reset_specific_oxidizer_control_board,
    Message::ResetSpecific(datatypes::status::BoardId::OxidizerControlBoard),
    hermes_can::messages::Message::ResetSpecific(decoded) => {
        assert_eq!(decoded.board_id, hermes_can::messages::BoardId::OxidizerControlBoard);
    }
);

// Test compatibility of UTCTimeUpdate - Test 1
test_binary_compat!(
    test_utc_time_update_1,
    Message::UTCTimeUpdate(datatypes::status::InstantUs(1_704_067_200_000_000)),
    hermes_can::messages::Message::UTCTimeUpdate(decoded) => {
        assert_eq!(decoded.timestamp_micros, 1_704_067_200_000_000);
    }
);

// Test compatibility of UTCTimeUpdate - Test 2
test_binary_compat!(
    test_utc_time_update_2,
    Message::UTCTimeUpdate(datatypes::status::InstantUs(1_735_689_600_123_456)),
    hermes_can::messages::Message::UTCTimeUpdate(decoded) => {
        assert_eq!(decoded.timestamp_micros, 1_735_689_600_123_456);
    }
);
