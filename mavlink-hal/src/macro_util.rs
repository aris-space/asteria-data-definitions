use crate::traits::MessageField;

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
