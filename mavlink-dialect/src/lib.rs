use mavio::{
    Dialect,
    protocol::{IntoPayload, MessageSpec},
};
use mavlink_hal::traits::DescribeMessages as _;

// #derive MavlinkDialect
// #[dialect(id = 900, version = 1)]
enum AnyMessage {
    PowerBoard(dp_power_board::Message),
    OxidizerControlBoard(dp_oxidizer_control_board::Message),
}

// maybe use tbytes of mavspec <https://docs.rs/mavspec/latest/mavspec/rust/spec/tbytes/index.html>

// generated
impl AnyMessage {
    pub const MESSAGES: &[mavlink_hal::traits::Message] = dp_power_board::Message::MESSAGES;
}
impl IntoPayload for AnyMessage {
    fn encode(
        &self,
        version: mavio::prelude::MavLinkVersion,
    ) -> Result<mavio::protocol::Payload, mavio::prelude::SpecError> {
        todo!()
    }
}
impl MessageSpec for AnyMessage {
    fn id(&self) -> mavio::protocol::MessageId {
        todo!()
    }
    fn min_supported_mavlink_version(&self) -> mavio::prelude::MavLinkVersion {
        todo!()
    }
    fn crc_extra(&self) -> mavio::protocol::CrcExtra {
        todo!()
    }
}
impl Dialect for AnyMessage {
    fn name() -> &'static str {
        "ASTERIA"
    }
    fn dialect() -> Option<mavio::protocol::DialectId> {
        None
    }
    fn version() -> Option<mavio::protocol::DialectVersion> {
        Some(1)
    }
    fn message_info(
        id: mavio::protocol::MessageId,
    ) -> Result<&'static dyn mavio::protocol::MessageSpec, mavio::prelude::SpecError> {
        todo!()
    }
    fn decode(payload: &mavio::protocol::Payload) -> Result<Self, mavio::prelude::SpecError> {
        todo!()
    }
    fn spec() -> &'static mavio::protocol::DialectSpec {
        todo!()
    }
}
