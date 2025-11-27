use mavio::{
    Dialect,
    protocol::{IntoPayload, MessageSpec},
};
use mavlink_hal::traits::DescribeMessages as _;

mod experiment;

// #[derive(MavlinkDialect)]
// #[dialect(name = "ASTERIA", id = 900, version = 1)] // idk whether 900 is good
enum AnyMessage {
    // every newtype variant should implement `DescribeMessages`
    PowerBoard(dp_power_board::Message),
    OxidizerControlBoard(dp_oxidizer_control_board::Message),
}

// generated
impl AnyMessage {
    pub const MESSAGES: [mavlink_hal::traits::Message;
        // fixme: of course should be once dp_oxidizer_control_board, but not implemented yet
        dp_power_board::Message::MESSAGES.len() + dp_power_board::Message::MESSAGES.len()] =
        mavlink_hal::macro_util::concat_slices(&[
            dp_power_board::Message::MESSAGES,
            dp_power_board::Message::MESSAGES,
        ]);
    // todo: uniqueness checks
}
impl IntoPayload for AnyMessage {
    fn encode(
        &self,
        version: mavio::prelude::MavLinkVersion,
    ) -> Result<mavio::protocol::Payload, mavio::prelude::SpecError> {
        match self {
            AnyMessage::PowerBoard(message) => todo!(),
            AnyMessage::OxidizerControlBoard(message) => todo!(),
        }
    }
}
impl MessageSpec for AnyMessage {
    fn id(&self) -> mavio::protocol::MessageId {
        todo!("can be dispatched to variant")
    }
    fn min_supported_mavlink_version(&self) -> mavio::prelude::MavLinkVersion {
        todo!("can be dispatched to variant")
    }
    fn crc_extra(&self) -> mavio::protocol::CrcExtra {
        todo!("can be dispatched to variant")
    }
}
impl Dialect for AnyMessage {
    fn name() -> &'static str {
        "ASTERIA"
    }
    fn dialect() -> Option<mavio::protocol::DialectId> {
        Some(900)
    }
    fn version() -> Option<mavio::protocol::DialectVersion> {
        Some(1)
    }
    fn message_info(
        id: mavio::protocol::MessageId,
    ) -> Result<&'static dyn mavio::protocol::MessageSpec, mavio::prelude::SpecError> {
        // needs to lookup id in variants, and then dispatch.
        // But not to `Dialect` trait, needs another trait and const stuff maybe.
        todo!()
    }
    fn decode(payload: &mavio::protocol::Payload) -> Result<Self, mavio::prelude::SpecError> {
        // Can also look up id and dispatch.
        todo!()
    }
    fn spec() -> &'static mavio::protocol::DialectSpec {
        // need to generate dummy type I think.
        todo!()
    }
}
