use crate::{traits::MessageField, typing::MavType};

pub trait InvalidInitializable: Copy {
    const INVALID: Self;
}

impl InvalidInitializable for MessageField {
    const INVALID: MessageField = MessageField {
        type_: crate::typing::MavType::Scalar(crate::typing::MavSimpleType::UInt8),
        name: "",
        unit: None,
        description: "",
        enum_: None,
    };
}
impl InvalidInitializable for crate::traits::Message {
    const INVALID: Self = Self {
        id: 0,
        name: "",
        description: "",
        fields: &[],
    };
}

/// Concatenates slices at compile time, with final length `N`. Gives a owned array back.
/// Panics if `N` does not match the sum of the lengths of `sets`,
/// but you can easily compute this in a const fashion as well.
pub const fn concat_slices<T: InvalidInitializable, const N: usize>(sets: &[&[T]]) -> [T; N] {
    let mut x = [T::INVALID; N];
    let mut rest: &mut [T] = &mut x;

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
/// Panics if `orig_fields` doesn't contain exactly one entry.
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

/// Computes the total size of the fields that are `align`-aligned in bytes.
/// Note that this is only non null for `align` in 1, 2, 4, 8.
pub const fn total_size_by_align(fields: &[MessageField], align: usize) -> usize {
    let mut size = 0;
    let mut i = 0;
    while i < fields.len() {
        if fields[i].type_.base().size() == align {
            size += fields[i].type_.size();
        }
        i += 1;
    }
    size
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

// TODO: redefine as const, should be possible with rewriting the for loops, and taking a patched
// CRC crate with this PR <https://github.com/mrhooray/crc-rs/pull/128>.
///
/// In maviola: <https://docs.rs/mavinspect/latest/src/mavinspect/protocol/messages/message.rs.html#356-380>
pub fn calc_extra_crc(message_name: &str, orig_fields: &[MessageField]) -> u8 {
    const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_MCRF4XX);
    let mut digest = X25.digest();
    digest.update(message_name.as_bytes());
    digest.update(b" ");
    // instead of reordering, do the calculation 4x, once for every alignment, as it is done in our encoding.
    for align in [8, 4, 2, 1] {
        for field in orig_fields {
            if field.type_.base().size() != align {
                continue;
            }
            digest.update(field.type_.base().c_type().as_bytes());
            digest.update(b" ");
            digest.update(field.name.as_bytes());
            digest.update(b" ");
            if let MavType::Array(_, len) = field.type_ {
                digest.update(&[len]);
            }
        }
    }
    let crc = digest.finalize();
    #[allow(clippy::cast_possible_truncation)] // no truncation due to bit fiddling
    let eight_bit = ((crc & 0xFF) ^ (crc >> 8)) as u8;
    eight_bit
}

#[cfg(test)]
mod tests {
    use crate::macro_util::{calc_extra_crc, total_size, total_size_by_align};
    use crate::traits::MessageField;
    use crate::typing::Typeable as _;

    const MINIMAL_FIELD_SET: &[MessageField] = &[
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
            type_: crate::typing::MavType::Scalar(
                crate::typing::MavSimpleType::UInt8MavlinkVersion,
            ),
            name: "mavlink_version",
            unit: None,
            description: "irrelevant",
            enum_: None,
        },
    ];

    #[test]
    fn check_minimal_crc() {
        let crc = calc_extra_crc("HEARTBEAT", MINIMAL_FIELD_SET);
        let actual_crc = mavio::dialects::minimal::messages::Heartbeat::crc_extra();
        assert_eq!(crc, actual_crc);
    }

    #[allow(clippy::identity_op)]
    #[test]
    fn check_length_consistent() {
        let full_len = total_size(MINIMAL_FIELD_SET);
        let len_1 = total_size_by_align(MINIMAL_FIELD_SET, 1);
        let len_2 = total_size_by_align(MINIMAL_FIELD_SET, 2);
        let len_4 = total_size_by_align(MINIMAL_FIELD_SET, 4);
        let len_8 = total_size_by_align(MINIMAL_FIELD_SET, 8);
        assert_eq!(full_len, 0 + 4 + 1 + 1 + 1 + 1 + 1); // copied from mavio macro expansion
        assert_eq!(full_len, len_1 + len_2 + len_4 + len_8); // copied from mavio macro expansion
    }
}
