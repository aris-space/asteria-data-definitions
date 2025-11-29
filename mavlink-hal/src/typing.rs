//! Provides definitions and helpers to interact between primitive mavlink types and rust types.

use crate::description::Enum;

#[derive(Debug, Clone, Copy)]
pub enum MavType {
    Scalar(MavScalarType),
    Array(MavScalarType, u8),
}

#[derive(Debug, Clone, Copy)]
pub enum MavScalarType {
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float,
    Double,
    Char,
    UInt8MavlinkVersion,
}

impl MavScalarType {
    /// Returns the size in bytes
    pub const fn size(self) -> usize {
        match self {
            MavScalarType::UInt8
            | MavScalarType::Int8
            | MavScalarType::Char
            | MavScalarType::UInt8MavlinkVersion => 1,
            MavScalarType::UInt16 | MavScalarType::Int16 => 2,
            MavScalarType::UInt32 | MavScalarType::Int32 | MavScalarType::Float => 4,
            MavScalarType::UInt64 | MavScalarType::Int64 | MavScalarType::Double => 8,
        }
    }

    /// Returns the name of the type as mavlink needs it
    pub const fn type_name(self) -> &'static str {
        match self {
            MavScalarType::UInt8 => "uint8_t",
            MavScalarType::UInt16 => "uint16_t",
            MavScalarType::UInt32 => "uint32_t",
            MavScalarType::UInt64 => "uint64_t",
            MavScalarType::Int8 => "int8_t",
            MavScalarType::Int16 => "int16_t",
            MavScalarType::Int32 => "int32_t",
            MavScalarType::Int64 => "int64_t",
            MavScalarType::Float => "float",
            MavScalarType::Double => "double",
            MavScalarType::Char => "char",
            MavScalarType::UInt8MavlinkVersion => "uint8_t_mavlink_version",
        }
    }

    /// Returns the name of the type as the crc calculation needs it
    pub const fn c_type(self) -> &'static str {
        match self {
            MavScalarType::UInt8MavlinkVersion | MavScalarType::UInt8 => "uint8_t",
            MavScalarType::UInt16 => "uint16_t",
            MavScalarType::UInt32 => "uint32_t",
            MavScalarType::UInt64 => "uint64_t",
            MavScalarType::Int8 => "int8_t",
            MavScalarType::Int16 => "int16_t",
            MavScalarType::Int32 => "int32_t",
            MavScalarType::Int64 => "int64_t",
            MavScalarType::Float => "float",
            MavScalarType::Double => "double",
            MavScalarType::Char => "char",
        }
    }
}

impl MavType {
    /// Returns the base type, so either the scalar type itself or the element type of the array.
    pub const fn base(self) -> MavScalarType {
        match self {
            MavType::Scalar(mav_simple_type) | MavType::Array(mav_simple_type, _) => {
                mav_simple_type
            }
        }
    }

    /// Returns the size in bytes of this type.
    pub const fn size(self) -> usize {
        match self {
            MavType::Scalar(mav_simple_type) => mav_simple_type.size(),
            MavType::Array(mav_simple_type, len) => mav_simple_type.size() * (len as usize),
        }
    }
}

/// A type that describes a primitive mavlink type.
pub trait Typeable {
    const TYPE: MavType;
    /// The enum of the type, applied to all elements of the array.
    const ENUM: Option<&'static Enum>;
}

/// A type that describes a scalar mavlink type, i.e. not an array.
pub trait ScalarTypeable {
    const SCALAR_TYPE: MavScalarType;
    const SCALAR_ENUM: Option<&'static Enum>;
}

impl ScalarTypeable for u8 {
    const SCALAR_TYPE: MavScalarType = MavScalarType::UInt8;
    const SCALAR_ENUM: Option<&'static Enum> = None;
}
impl ScalarTypeable for u16 {
    const SCALAR_TYPE: MavScalarType = MavScalarType::UInt16;
    const SCALAR_ENUM: Option<&'static Enum> = None;
}
impl ScalarTypeable for u32 {
    const SCALAR_TYPE: MavScalarType = MavScalarType::UInt32;
    const SCALAR_ENUM: Option<&'static Enum> = None;
}
impl ScalarTypeable for u64 {
    const SCALAR_TYPE: MavScalarType = MavScalarType::UInt64;
    const SCALAR_ENUM: Option<&'static Enum> = None;
}
impl ScalarTypeable for i8 {
    const SCALAR_TYPE: MavScalarType = MavScalarType::Int8;
    const SCALAR_ENUM: Option<&'static Enum> = None;
}
impl ScalarTypeable for i16 {
    const SCALAR_TYPE: MavScalarType = MavScalarType::Int16;
    const SCALAR_ENUM: Option<&'static Enum> = None;
}
impl ScalarTypeable for i32 {
    const SCALAR_TYPE: MavScalarType = MavScalarType::Int32;
    const SCALAR_ENUM: Option<&'static Enum> = None;
}
impl ScalarTypeable for i64 {
    const SCALAR_TYPE: MavScalarType = MavScalarType::Int64;
    const SCALAR_ENUM: Option<&'static Enum> = None;
}
impl ScalarTypeable for f32 {
    const SCALAR_TYPE: MavScalarType = MavScalarType::Float;
    const SCALAR_ENUM: Option<&'static Enum> = None;
}
impl ScalarTypeable for f64 {
    const SCALAR_TYPE: MavScalarType = MavScalarType::Double;
    const SCALAR_ENUM: Option<&'static Enum> = None;
}

// note that rust's char is not typeable directly (would be u32, not char)

impl<T: ScalarTypeable> Typeable for T {
    const TYPE: MavType = MavType::Scalar(Self::SCALAR_TYPE);
    const ENUM: Option<&'static Enum> = Self::SCALAR_ENUM;
}

impl<T: ScalarTypeable, const N: usize> Typeable for [T; N] {
    #[allow(clippy::cast_possible_truncation)] // checked by assert
    const TYPE: MavType = {
        assert!(N < 256, "mavlink only supports arrays up to 255 elements");
        MavType::Array(T::SCALAR_TYPE, N as u8)
    };
    const ENUM: Option<&'static Enum> = T::SCALAR_ENUM;
}
