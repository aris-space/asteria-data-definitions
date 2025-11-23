use crate::{traits::MessageField, typing::MavType};

impl MessageField {
    const INVALID: MessageField = MessageField {
        type_: crate::typing::MavType::Scalar(crate::typing::MavSimpleType::UInt8),
        name: "",
        unit: None,
        description: "",
        enum_: None,
    };
}

/// Concatenates field sets, with final length `N`. Gives a owned array back.
/// Panics if `N` does not match the sum of the lengths of `sets`.
pub const fn concat_field_sets<const N: usize>(sets: &[&[MessageField]]) -> [MessageField; N] {
    let mut x = [MessageField::INVALID; N];
    let mut rest: &mut [MessageField] = &mut x;

    let mut i = 0;
    while i < sets.len() {
        // iterators aren't const
        let (this, new_rest) = rest.split_at_mut(sets[i].len());
        rest = new_rest;
        this.copy_from_slice(sets[i]);
        i += 1;
    }

    assert!(rest.is_empty());
    x
}

/// Refines a fieldset consisting of a single field with a new name and description, keeping the unit and type.
pub const fn refine_field(
    orig_fields: &[MessageField],
    new_name: &'static str,
    new_description: &'static str,
) -> MessageField {
    assert!(
        orig_fields.len() == 1,
        "must be exactly one field to be refined"
    );
    MessageField {
        type_: orig_fields[0].type_,
        name: new_name,
        unit: orig_fields[0].unit,
        description: new_description,
        enum_: orig_fields[0].enum_,
    }
}

/// Computes the total size of all fields in bytes.
pub const fn total_size(fields: &[MessageField]) -> usize {
    let mut size = 0;
    let mut i = 0;
    while i < fields.len() {
        size += fields[i].type_.size();
        i += 1;
    }
    size
}

// TODO: redefine as const, should be possible maybe, at least with rewriting some stuff
///
/// In maviola: <https://docs.rs/mavinspect/latest/src/mavinspect/protocol/messages/message.rs.html#356-380>
pub fn calc_extra_crc(message_name: &str, orig_fields: &mut [MessageField]) -> u8 {
    const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_MCRF4XX);
    orig_fields.sort_by_key(|t| std::cmp::Reverse(t.type_.base().size())); // non-const sort :/
    let mut digest = X25.digest();
    digest.update(message_name.as_bytes());
    digest.update(b" ");
    for field in orig_fields {
        digest.update(field.type_.base().type_name().as_bytes());
        digest.update(b" ");
        digest.update(field.name.as_bytes());
        digest.update(b" ");
        if let MavType::Array(_, len) = field.type_ {
            digest.update(&[len]);
        }
    }
    let crc = digest.finalize();
    #[allow(clippy::cast_possible_truncation)] // no truncation due to bit fiddling
    let eight_bit = ((crc & 0xFF) ^ (crc >> 8)) as u8;
    eight_bit
}

#[test]
fn check_minimal_crc() {
    use crate::typing::Typeable;
    let minimal_field_set: &mut [MessageField] = &mut [
        MessageField {
            type_: <u8>::TYPE,
            name: "type",
            unit: None,
            description: "irrelevant",
            enum_: None,
        },
        MessageField {
            type_: <u8>::TYPE,
            name: "autopilot",
            unit: None,
            description: "irrelevant",
            enum_: None,
        },
        MessageField {
            type_: <u8>::TYPE,
            name: "base_mode",
            unit: None,
            description: "irrelevant",
            enum_: None,
        },
        MessageField {
            type_: <u32>::TYPE,
            name: "custom_mode",
            unit: None,
            description: "irrelevant",
            enum_: None,
        },
        MessageField {
            type_: <u8>::TYPE,
            name: "system_status",
            unit: None,
            description: "irrelevant",
            enum_: None,
        },
        MessageField {
            type_: MavType::Scalar(crate::typing::MavSimpleType::UInt8MavlinkVersion),
            name: "mavlink_version",
            unit: None,
            description: "irrelevant",
            enum_: None,
        },
    ];
    let crc = calc_extra_crc("HEARTBEAT", minimal_field_set);
    let actual_crc = mavio::dialects::minimal::messages::Heartbeat::crc_extra();
    // Doesn't work yet :/
    assert_eq!(crc, actual_crc);
}
