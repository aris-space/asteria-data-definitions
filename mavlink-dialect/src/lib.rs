use mavio::Dialect;
use mavio::protocol::{IntoPayload, MessageSpec};
use mavlink_hal::description::DescribeMessages;

// #[derive(MavlinkDialect)]
// #[dialect(name = "ASTERIA", id = 900, version = 1)] // idk whether 900 is good
// every variant should be a newtype whose type implements `DescribeMessages` and the other things derived with it.
pub enum AnyMessage {
    PowerBoard(dp_power_board::Message),
    OxidizerControlBoard(dp_oxidizer_control_board::Message),
}

// generated
impl AnyMessage {
    const MAVLINK_MESSAGES: [mavlink_hal::description::Message;
        // fixme: of course should include dp_oxidizer_control_board, but not implemented yet
        dp_power_board::Message::MESSAGES.len()] =
        mavlink_hal::macro_util::concat_slices(&[
            dp_power_board::Message::MESSAGES,
        ]);
}
const _: () = mavlink_hal::macro_util::check_unique_ids(&AnyMessage::MAVLINK_MESSAGES);
impl DescribeMessages for AnyMessage {
    const MESSAGES: &[mavlink_hal::description::Message] = &Self::MAVLINK_MESSAGES;
}
impl IntoPayload for AnyMessage {
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
impl MessageSpec for AnyMessage {
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
impl Dialect for AnyMessage {
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
        Self::MESSAGES
            .iter()
            .find(|m| m.id == id)
            .map(|m| m as &'static dyn mavio::protocol::MessageSpec)
            .ok_or(mavio::prelude::SpecError::NotInDialect(id))
    }
    fn decode(payload: &mavio::protocol::Payload) -> Result<Self, mavio::prelude::SpecError> {
        // look up id and dispatch.
        if dp_power_board::Message::MESSAGES
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
        const MESSAGE_INFO: [mavio::protocol::MessageInfo; AnyMessage::MESSAGES.len()] =
            mavlink_hal::macro_util::into_mavio_infos::<{ AnyMessage::MESSAGES.len() }>(
                AnyMessage::MESSAGES,
            );
        const DIALECT_SPEC: mavio::protocol::DialectSpec = mavio::protocol::DialectSpec::new(
            "ASTERIA",
            core::option::Option::Some(900),
            core::option::Option::Some(1),
            &MESSAGE_INFO,
        );
        &DIALECT_SPEC
    }
}
