#[derive(Debug, Clone, Copy)]
pub enum MavType {
    Scalar(MavSimpleType),
    Array(MavSimpleType, usize),
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
    const TYPE: MavType = MavType::Array(T::SCALAR_TYPE, N);
}
