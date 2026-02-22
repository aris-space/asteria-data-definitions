pub mod decode;
pub mod dialect;
pub mod encode;
pub mod r#enum;
pub mod fieldset;
pub mod message;

/// Primitive types supported by MavLink which are 1-byte aligned
pub const MAVLINK_PRIMITIVES_1BYTE: &[&str] = &["bool", "u8", "i8"];
/// Primitive types supported by MavLink which are 2-byte aligned
pub const MAVLINK_PRIMITIVES_2BYTE: &[&str] = &["u16", "i16"];
/// Primitive types supported by MavLink which are 4-byte aligned
pub const MAVLINK_PRIMITIVES_4BYTE: &[&str] = &["u32", "i32", "f32"];
/// Primitive types supported by MavLink which are 8-byte aligned
pub const MAVLINK_PRIMITIVES_8BYTE: &[&str] = &["u64", "i64", "f64"];
