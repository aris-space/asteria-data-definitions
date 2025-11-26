#[derive(Debug, Clone, Copy)]
pub enum MavType {
    Scalar(MavSimpleType),
    Array(MavSimpleType, u8),
}

#[derive(Debug, Clone, Copy)]
pub enum MavSimpleType {
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

impl MavSimpleType {
    /// Returns the size in bytes
    pub const fn size(self) -> usize {
        match self {
            MavSimpleType::UInt8
            | MavSimpleType::Int8
            | MavSimpleType::Char
            | MavSimpleType::UInt8MavlinkVersion => 1,
            MavSimpleType::UInt16 | MavSimpleType::Int16 => 2,
            MavSimpleType::UInt32 | MavSimpleType::Int32 | MavSimpleType::Float => 4,
            MavSimpleType::UInt64 | MavSimpleType::Int64 | MavSimpleType::Double => 8,
        }
    }

    /// Returns the name of the type as mavlink needs it
    pub const fn type_name(self) -> &'static str {
        match self {
            MavSimpleType::UInt8 => "uint8_t",
            MavSimpleType::UInt16 => "uint16_t",
            MavSimpleType::UInt32 => "uint32_t",
            MavSimpleType::UInt64 => "uint64_t",
            MavSimpleType::Int8 => "int8_t",
            MavSimpleType::Int16 => "int16_t",
            MavSimpleType::Int32 => "int32_t",
            MavSimpleType::Int64 => "int64_t",
            MavSimpleType::Float => "float",
            MavSimpleType::Double => "double",
            MavSimpleType::Char => "char",
            MavSimpleType::UInt8MavlinkVersion => "uint8_t_mavlink_version",
        }
    }

    /// Returns the name of the type as the crc calculation needs it
    pub const fn c_type(self) -> &'static str {
        match self {
            MavSimpleType::UInt8MavlinkVersion | MavSimpleType::UInt8 => "uint8_t",
            MavSimpleType::UInt16 => "uint16_t",
            MavSimpleType::UInt32 => "uint32_t",
            MavSimpleType::UInt64 => "uint64_t",
            MavSimpleType::Int8 => "int8_t",
            MavSimpleType::Int16 => "int16_t",
            MavSimpleType::Int32 => "int32_t",
            MavSimpleType::Int64 => "int64_t",
            MavSimpleType::Float => "float",
            MavSimpleType::Double => "double",
            MavSimpleType::Char => "char",
        }
    }
}

impl MavType {
    /// Returns the base type, so either the scalar type itself or the element type of the array.
    pub const fn base(self) -> MavSimpleType {
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

pub trait Typeable {
    const TYPE: MavType;
}

pub trait ScalarTypeable {
    const SCALAR_TYPE: MavSimpleType;
}

impl ScalarTypeable for u8 {
    const SCALAR_TYPE: MavSimpleType = MavSimpleType::UInt8;
}
impl ScalarTypeable for u16 {
    const SCALAR_TYPE: MavSimpleType = MavSimpleType::UInt16;
}

impl ScalarTypeable for u32 {
    const SCALAR_TYPE: MavSimpleType = MavSimpleType::UInt32;
}

impl ScalarTypeable for u64 {
    const SCALAR_TYPE: MavSimpleType = MavSimpleType::UInt64;
}

impl ScalarTypeable for i8 {
    const SCALAR_TYPE: MavSimpleType = MavSimpleType::Int8;
}

impl ScalarTypeable for i16 {
    const SCALAR_TYPE: MavSimpleType = MavSimpleType::Int16;
}

impl ScalarTypeable for i32 {
    const SCALAR_TYPE: MavSimpleType = MavSimpleType::Int32;
}

impl ScalarTypeable for i64 {
    const SCALAR_TYPE: MavSimpleType = MavSimpleType::Int64;
}

impl ScalarTypeable for f32 {
    const SCALAR_TYPE: MavSimpleType = MavSimpleType::Float;
}

impl ScalarTypeable for f64 {
    const SCALAR_TYPE: MavSimpleType = MavSimpleType::Double;
}

// note that char is not typeable directly (would be u32, not char)

impl<T: ScalarTypeable> Typeable for T {
    const TYPE: MavType = MavType::Scalar(Self::SCALAR_TYPE);
}

impl<T: ScalarTypeable, const N: usize> Typeable for [T; N] {
    #[allow(clippy::cast_possible_truncation)] // checked by assert
    const TYPE: MavType = {
        assert!(N < 256, "mavlink only supports arrays up to 255 elements");
        MavType::Array(T::SCALAR_TYPE, N as u8)
    };
}
