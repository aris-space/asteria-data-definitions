//! Static (compile-time) information about mavlink messages.
//!
//! Contains 4 levels:
//! - at the bottom a [`MessageField`] specifies a single field.
//! - a [`DescribeFieldSet`] implementor then groups a list of fields together (a part of a message).
//! - a [`Message`] takes the list of messages and adds a message name and id, to make it a message.
//! - finally, a [`DescribeMessages`] implementation groups a list of messages.
//!
//! See also the mavlink schema defintion: <https://github.com/ArduPilot/pymavlink/blob/master/generator/mavschema.xsd>

use mavio::protocol::MessageSpec;

use crate::macro_util::calc_extra_crc;
use crate::typing::MavType;

/// Implementors describe a set of messages, such as part of a dialect.
/// The messages must be unique in ID.
pub trait DescribeMessages {
    const MESSAGES: &[Message];
}

/// A single mavlink message
#[derive(Debug, Clone, Copy)]
pub struct Message {
    /// The id of the message. Should not collide with other dialects unless it is compatible.
    /// See [`crate::macro_util::sanitize_id`].
    pub id: u32,
    /// The message name. Should be `UPPER_SNAKE_CASE`.
    pub name: &'static str,
    /// The message description, human readable.
    pub description: &'static str,
    /// The fields of the message, in order as they appear in the definition.
    /// The names should be unique.
    pub fields: &'static [MessageField],
}

/// Implementors describe a set of fields, in the order as given in the list.
pub trait DescribeFieldSet {
    const FIELDS: &[MessageField];
}

/// A single mavlink field
#[derive(Debug, Clone, Copy)]
pub struct MessageField {
    /// The mavlink type specified. For enums this must be an integer type.
    pub type_: MavType,
    /// The name of the field. Should be `snake_case`.
    pub name: &'static str,
    /// The optional `units` attribute.
    /// See <https://github.com/ArduPilot/pymavlink/blob/master/generator/mavschema.xsd#L85-L180> for officially supported units.
    pub unit: Option<&'static str>,
    /// The field description, human readable.
    pub description: &'static str,
    /// The optional enum restriction of an integer field (currectly restricted to u8).
    pub enum_: Option<&'static Enum>,
    // todo: add the invalid value?
}

/// A enum definition
#[derive(Debug, Clone, Copy)]
pub struct Enum {
    /// The name of the enum. Should be `UPPER_SNAKE_CASE`.
    /// Must not collide with other defined enums, as the semantics of merging enums is not implemented.
    pub name: &'static str,
    /// The description of the enum.
    pub description: &'static str,
    /// The available variants.
    pub variants: &'static [EnumVariant],
    // todo: add bitflag support?
}

#[derive(Debug, Clone, Copy)]
pub struct EnumVariant {
    /// The value of the enum.
    /// *Note*: We currently restrict to u8, as I don't see other usage in our application.
    pub value: u8,
    /// The variant name. Should be `UPPER_SNAKE_CASE`.
    pub name: &'static str,
    /// The description of the variant.
    pub description: &'static str,
}

// fixme: not sure if needed
/// Implementors are enums that are mavlink compatible.
pub trait DescribeEnum {
    const ENUM: &'static Enum;
}

/// Convert to the static information required by mavio
impl MessageSpec for Message {
    fn id(&self) -> mavio::protocol::MessageId {
        self.id
    }
    fn min_supported_mavlink_version(&self) -> mavio::prelude::MavLinkVersion {
        mavio::prelude::MavLinkVersion::V2
    }
    fn crc_extra(&self) -> mavio::protocol::CrcExtra {
        calc_extra_crc(self.name, self.fields)
    }
}
