use data_core::can_hal::CanEncode;
use dp_recovery_board::SteeringPositions;

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
