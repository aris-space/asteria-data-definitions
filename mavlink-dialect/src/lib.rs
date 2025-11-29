//! This crate defines the full mavlink dialect by including all relevant subdialects.
//!
//! [`AnyMessage`] can then be decoded using [`mavio`], and to encode one can directly use the subdialects.
//! See the `roundtrip example`.

// #[derive(MavlinkDialect)]
// #[dialect(name = "ASTERIA", id = 900, version = 1)] // idk whether 900 is good
// every variant should be a newtype whose type implements `DescribeMessages` and the other things derived with it.
pub enum AnyMessage {
    PowerBoard(dp_power_board::Message),
    OxidizerControlBoard(dp_oxidizer_control_board::Message),
}

// generated: collect the messages
impl mavlink_hal::description::DescribeMessages for AnyMessage {
    const MESSAGES: &[mavlink_hal::description::Message] =
        &mavlink_hal::macro_util::concat_slices::<_, { dp_power_board::Message::MESSAGES.len() }>(
            &[dp_power_board::Message::MESSAGES],
        );
}
const _: () = mavlink_hal::macro_util::check_unique_ids(
    <AnyMessage as mavlink_hal::description::DescribeMessages>::MESSAGES,
);
// generated: dipatched mavio message info
impl mavio::protocol::MessageSpec for AnyMessage {
    fn id(&self) -> mavio::protocol::MessageId {
        match self {
            AnyMessage::PowerBoard(message) => message.id(),
            AnyMessage::OxidizerControlBoard(_message) => todo!(),
        }
    }
    fn min_supported_mavlink_version(&self) -> mavio::prelude::MavLinkVersion {
        match self {
            AnyMessage::PowerBoard(message) => message.min_supported_mavlink_version(),
            AnyMessage::OxidizerControlBoard(_message) => todo!(),
        }
    }
    fn crc_extra(&self) -> mavio::protocol::CrcExtra {
        match self {
            AnyMessage::PowerBoard(message) => message.crc_extra(),
            AnyMessage::OxidizerControlBoard(_message) => todo!(),
        }
    }
}
// generated: dispatched encoding
impl mavio::protocol::IntoPayload for AnyMessage {
    fn encode(
        &self,
        version: mavio::prelude::MavLinkVersion,
    ) -> Result<mavio::protocol::Payload, mavio::prelude::SpecError> {
        match self {
            AnyMessage::PowerBoard(message) => message.encode(version),
            AnyMessage::OxidizerControlBoard(_message) => todo!("dispatch"),
        }
    }
}
// generated: dialect info and dispatched decoding
impl mavio::Dialect for AnyMessage {
    fn name() -> &'static str {
        "ASTERIA" // from macro attribute
    }
    fn dialect() -> Option<mavio::protocol::DialectId> {
        core::option::Option::Some(900) // from macro attribute
    }
    fn version() -> Option<mavio::protocol::DialectVersion> {
        core::option::Option::Some(1) // from macro attribute
    }
    fn message_info(
        id: mavio::protocol::MessageId,
    ) -> Result<&'static dyn mavio::protocol::MessageSpec, mavio::prelude::SpecError> {
        // linear search is maybe not the nicest option, but I think mavio does the same.
        <Self as mavlink_hal::description::DescribeMessages>::MESSAGES
            .iter()
            .find(|m| m.id == id)
            .map(|m| m as &'static dyn mavio::protocol::MessageSpec)
            .ok_or(mavio::prelude::SpecError::NotInDialect(id))
    }
    fn decode(payload: &mavio::protocol::Payload) -> Result<Self, mavio::prelude::SpecError> {
        // look up id and dispatch.
        if <dp_power_board::Message as mavlink_hal::description::DescribeMessages>::MESSAGES
            .iter()
            .any(|m| m.id == payload.id())
        {
            return dp_power_board::Message::try_from(payload).map(Self::PowerBoard);
        }
        // ...

        Err(mavio::prelude::SpecError::NotInDialect(payload.id()))
    }
    fn spec() -> &'static mavio::protocol::DialectSpec {
        // convert to the mavio message info type, which would also implement their trait, but here dyn isn't used...
        const DIALECT_SPEC: mavio::protocol::DialectSpec = mavio::protocol::DialectSpec::new(
            "ASTERIA",
            core::option::Option::Some(900),
            core::option::Option::Some(1),
            &mavlink_hal::macro_util::into_mavio_infos::<
                { <AnyMessage as mavlink_hal::description::DescribeMessages>::MESSAGES.len() },
            >(<AnyMessage as mavlink_hal::description::DescribeMessages>::MESSAGES),
        );
        &DIALECT_SPEC
    }
}
