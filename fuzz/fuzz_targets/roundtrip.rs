#![no_main]

use hermes_can::messages::Message;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|msg: Message| {
    // we cannot cleanly compare the messages directly, since the arbitrary generation very
    // quickly finds NANs and other invalid values that compare as not equal. Therefore, we
    // reencode the message after one round trip and compare the encoded data.

    let mut buf = [0u8; 64];
    let (id, written) = msg.try_write_into(&mut buf).expect("Failed to encode message");

    assert!(written <= 64, "Message too long: {} bytes written", written);
    let can_len = hermes_can::next_valid_length(written).expect("Message length is not valid");

    let payload = &buf[..can_len];
    let msg2 = Message::try_from_parts(id, payload).expect("Failed to decode message");

    let mut buf2 = [0u8; 64];
    let (id2, written2) = msg2.try_write_into(&mut buf2).expect("Failed to encode message");
    assert!(written2 <= 64, "Message too long after round trip: {} bytes written", written2);

    let can_len2 = hermes_can::next_valid_length(written2).expect("Message length is not valid");
    let payload2 = &buf2[..can_len2];

    assert_eq!(id, id2, "mismatch in id");
    assert_eq!(payload, payload2, "mismatch in data");
});
