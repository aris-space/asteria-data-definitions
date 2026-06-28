const fn parse_u8(s: &str) -> u8 {
    let bytes = s.as_bytes();
    let mut result = 0u8;
    let mut i = 0;
    while i < bytes.len() {
        result = result * 10 + (bytes[i] - b'0');
        i += 1;
    }
    result
}

/// Major version number of the crate, parsed from and synced with `CARGO_PKG_VERSION_MAJOR`
pub const VERSION_MAJOR: u8 = parse_u8(env!("CARGO_PKG_VERSION_MAJOR"));

/// Minor version number of the crate, parsed from and synced with `CARGO_PKG_VERSION_MINOR`
pub const VERSION_MINOR: u8 = parse_u8(env!("CARGO_PKG_VERSION_MINOR"));

/// Patch version number of the crate, parsed from and synced with `CARGO_PKG_VERSION_PATCH`
pub const VERSION_PATCH: u8 = parse_u8(env!("CARGO_PKG_VERSION_PATCH"));
