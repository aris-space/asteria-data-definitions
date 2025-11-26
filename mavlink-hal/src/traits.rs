use crate::typing::MavType;

pub trait DescribeFieldSet {
    const FIELDS: &[MessageField];
}

pub trait DescribeEnum {
    const ENUM: &'static Enum;
}

pub trait DescribeMessages {
    const MESSAGES: &[Message];
}

#[derive(Debug, Clone, Copy)]
pub struct Message {
    pub id: u32,
    pub name: &'static str,
    pub description: &'static str,
    pub fields: &'static [MessageField],
}

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
pub struct MessageField {
    pub type_: MavType,
    pub name: &'static str,
    pub unit: Option<&'static str>,
    pub description: &'static str,
    pub enum_: Option<&'static Enum>,
    // invalid?
}
