pub mod decode;
pub mod encode;
pub mod r#enum;
pub mod fieldset;
pub mod message;

/// Primitive types supported by MavLink which are 1-byte aligned
pub const MAVLINK_PRIMITIVES_1BYTE: &[&str; 3] = &["char", "u8", "i8"];
/// Primitive types supported by MavLink which are 2-byte aligned
pub const MAVLINK_PRIMITIVES_2BYTE: &[&str; 2] = &["u16", "i16"];
/// Primitive types supported by MavLink which are 4-byte aligned
pub const MAVLINK_PRIMITIVES_4BYTE: &[&str; 3] = &["u32", "i32", "f32"];
/// Primitive types supported by MavLink which are 8-byte aligned
pub const MAVLINK_PRIMITIVES_8BYTE: &[&str; 3] = &["u64", "i64", "f64"];
