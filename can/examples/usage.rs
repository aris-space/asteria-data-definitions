#![allow(clippy::uninlined_format_args)]

use hermes_can::messages::{BoardId, Message};
use hermes_can::{CanMessage, next_valid_length};

fn main() {
    println!(
        "This is Hermes CAN version {}.{}.{}",
        hermes_can::VERSION_MAJOR,
        hermes_can::VERSION_MINOR,
        hermes_can::VERSION_PATCH
    );

    {
        let msg = hermes_can::messages::system_management::ResetAll;

        let mut buf = [0u8; 64];
        let (id, written) = msg
            .try_write_into(&mut buf)
            .expect("Failed to encode message");

        // payload is exactly the length of the message, no padding!
        assert!(written <= 64);
        println!("id = {:?}, data = {:?}", id, written);

        // to transmit the message, you'd usually have to pad the payload to the next valid length
        let next_valid = next_valid_length(written).expect("Invalid payload length");
        let payload = &buf[..next_valid];

        println!("{:?}", payload);

        let decoded = Message::try_from_parts(id, payload).expect("Failed to decode message");
        println!("{:?}", decoded);
    }

    {
        let msg: Message = hermes_can::messages::system_management::ResetSpecific {
            board_id: BoardId::PowerBoard,
        }
        .into();
        println!("{:?}", msg);

        let mut buf = [0u8; 64];
        let (id, written) = msg
            .try_write_into(&mut buf)
            .expect("Failed to encode message");

        // payload is exactly the length of the message, no padding!
        assert!(written <= 64);
        println!("id = {:?}, data = {:?}", id, written);

        // to transmit the message, you'd usually have to pad the payload to the next valid length
        let next_valid = next_valid_length(written).expect("Invalid payload length");
        let payload = &buf[..next_valid];

        println!("{:?}", payload);

        let decoded = Message::try_from_parts(id, payload).expect("Failed to decode message");
        println!("{:?}", decoded);
    }
}
