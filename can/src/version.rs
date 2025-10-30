const fn parse_u8(component: &str) -> u8 {
    match u8::from_str_radix(component, 10) {
        Ok(v) => v,
        Err(err) => {
            const_panic::concat_panic!(
                "Failed to parse version number: ",
                component,
                " with error: ",
                err
            )
        }
    }
}

/// Major version number of the crate, parsed from and synced with `CARGO_PKG_VERSION_MAJOR`
pub const VERSION_MAJOR: u8 = parse_u8(env!("CARGO_PKG_VERSION_MAJOR"));

/// Minor version number of the crate, parsed from and synced with `CARGO_PKG_VERSION_MINOR`
pub const VERSION_MINOR: u8 = parse_u8(env!("CARGO_PKG_VERSION_MINOR"));

/// Patch version number of the crate, parsed from and synced with `CARGO_PKG_VERSION_PATCH`
pub const VERSION_PATCH: u8 = parse_u8(env!("CARGO_PKG_VERSION_PATCH"));
