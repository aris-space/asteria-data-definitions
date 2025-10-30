use hermes_mavlink::examples::*;
use mavlink_core::{MavlinkVersion, Message, MessageData};
use std::time::SystemTime;

/// This example demonstrates how to encode and decode a MAVLink message.
fn main() {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let heartbeat = MavMessage::HEARTBEAT(HEARTBEAT_DATA {
        system_type: MavSystemType::ROCKET,
        uptime: now.as_millis() as u64,
    });

    let mut bytes = vec![0; 100];
    let len = heartbeat.ser(MavlinkVersion::V2, &mut bytes);
    println!("Encoded: {:?}", &bytes[..len]);

    let decoded = MavMessage::parse(MavlinkVersion::V2, HEARTBEAT_DATA::ID, &bytes[..len])
        .expect("Failed to decode mavlink message");

    assert_eq!(
        decoded, heartbeat,
        "Decoded message does not match original message"
    );

    println!("Decoded: {decoded:?}");
}
