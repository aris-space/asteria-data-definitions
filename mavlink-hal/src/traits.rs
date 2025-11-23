pub trait DescribeEnum {}

#[derive(Debug, Clone, Copy)]
pub struct Enum {
    pub name: &'static str,
    pub description: &'static str,
    pub variants: &'static [EnumVariant],
}

#[derive(Debug, Clone, Copy)]
pub struct EnumVariant {
    pub value: u8,
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub enum MavType {
    Simple(MavSimpleType),
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

#[derive(Debug, Clone, Copy)]
pub struct MessageField {
    pub type_: MavType,
    pub name: &'static str,
    pub unit: Option<&'static str>,
    pub description: &'static str,
    pub enum_: Option<&'static Enum>,
    // invalid?
}

impl MessageField {
    pub const INVALID: MessageField = MessageField {
        type_: MavType::Simple(MavSimpleType::UInt8),
        name: "",
        unit: None,
        description: "",
        enum_: None,
    };
}
