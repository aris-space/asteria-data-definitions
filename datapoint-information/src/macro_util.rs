use crate::datapoint::DatapointInformation;

/// Manually compare bytes of two static strings for equality.
/// This is needed as `PartialEq` is not (yet) const
const fn check_str_eq(a: &'static str, b: &'static str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    if a_bytes.len() != b_bytes.len() {
        return false;
    }

    let mut i = 0;
    while i < a_bytes.len() {
        if a_bytes[i] != b_bytes[i] {
            return false;
        }
        i += 1;
    }

    true
}

/// Checks that all names of datapoint infos are unique.
/// Otherwise, panics to be used at compile time.
pub const fn check_unique_names(infos: &[DatapointInformation]) {
    let mut i = 0;
    while i < infos.len() {
        let mut j = i + 1;
        while j < infos.len() {
            assert!(
                !check_str_eq(infos[i].name, infos[j].name),
                "Duplicate name detected",
            );
            j += 1;
        }
        i += 1;
    }
}

/// Check that all names of datapoint infos fit in a 20 character buffer.
/// Otherwise, panics to be used at compile time.
pub const fn check_name_len(infos: &[DatapointInformation]) {
    let mut i = 0;
    while i < infos.len() {
        assert!(infos[i].name.len() <= 20, "Name is longer than 20 bytes");
        i += 1;
    }
}
