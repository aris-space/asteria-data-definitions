use can_hal::{CanDecode, CanEncode, StandardId};
use datatypes::{BuildInformationCommon, CameraPower, RailStatus, StatusCommonMessage};

// define_messages! {
pub enum Message {
    /// Tells the board that it should reset itself.
    ///
    /// # CAN
    /// - **Kind**: Event message
    /// - **Sent by**: Flight computer
    // #[can_id(0x001)]
    ResetBoard,

    /// Sent by the power board to give its current health status.
    ///
    /// # CAN
    /// - **Kind**: Periodic at 1Hz
    /// - **Sent by**: Power Board
    // #[can_id(0x002)]
    // #[mavlink_id(1_000_002)]
    BoardStatus(StatusCommonMessage),

    // #[can_id(0x003)]
    // #[mavlink_id(2_000_003)]
    CameraPowerStatus(CameraPower),
    // #[can_id(0x004)]
    // #[mavlink_id(1_000_004)]
    RailStatus5V(RailStatus),
    // #[can_id(0x005)]
    // #[mavlink_id(1_000_005)]
    RailStatus24V(RailStatus),
    // #[can_id(0x006)]
    // #[mavlink_id(1_000_006)]
    BoardBuildInfo(BuildInformationCommon),
}
// }

#[allow(non_upper_case_globals)]
pub mod __message_meta_can_ids {
    use ::can_hal::StandardId;

    pub const ResetBoard: StandardId = StandardId::new(0x001).expect("must be in range");
    pub const BoardStatus: StandardId = StandardId::new(0x002).expect("must be in range");
    pub const CameraPowerStatus: StandardId = StandardId::new(0x003).expect("must be in range");
    pub const RailStatus5V: StandardId = StandardId::new(0x004).expect("must be in range");
    pub const RailStatus24V: StandardId = StandardId::new(0x005).expect("must be in range");
    pub const BoardBuildInfo: StandardId = StandardId::new(0x006).expect("must be in range");

    pub const ALL_IDS: [StandardId; 6] = [
        ResetBoard,
        BoardStatus,
        CameraPowerStatus,
        RailStatus5V,
        RailStatus24V,
        BoardBuildInfo,
    ];
}

/// generated
#[allow(non_upper_case_globals)]
pub mod __message_meta_mavlink {
    use datatypes::StatusCommonMessage;
    use mavlink_hal::traits::{DescribeFieldSet, DescribeMessages};

    // TODO: maybe do range checks on ids, such as 24 bit limit and no collision with other dialects.
    // TODO: maybe name collision checks of enum names?

    // generated: specify which messages exist.
    const MESSAGES: [mavlink_hal::traits::Message; 1] = [
        mavlink_hal::traits::Message {
            id: 1_000_002,
            name: "BOARD_STATUS", // TODO: how to make this board specific
            description: "Sent by the power board to give its current health status ...",
            fields: StatusCommonMessage::FIELDS,
        },
        //...
    ];
    impl DescribeMessages for super::Message {
        const MESSAGES: &[mavlink_hal::traits::Message] = &MESSAGES;
    }
    impl mavio::protocol::MessageSpec for super::Message {
        fn id(&self) -> mavio::protocol::MessageId {
            match self {
                crate::Message::ResetBoard => {
                    panic!("ResetBoard is not a mavlink compatible variant")
                }
                crate::Message::BoardStatus(_) => 1_000_002,
                _ => todo!(),
            }
        }
        fn min_supported_mavlink_version(&self) -> mavio::prelude::MavLinkVersion {
            mavio::prelude::MavLinkVersion::V2
        }
        fn crc_extra(&self) -> mavio::protocol::CrcExtra {
            match self {
                crate::Message::ResetBoard => {
                    panic!("ResetBoard is not a mavlink compatible variant")
                }
                crate::Message::BoardStatus(_) => mavlink_hal::macro_util::calc_extra_crc(
                    "BOARD_STATUS",
                    StatusCommonMessage::FIELDS,
                ),
                _ => todo!(),
            }
        }
    }
    impl mavio::protocol::IntoPayload for super::Message {
        fn encode(
            &self,
            version: mavio::prelude::MavLinkVersion,
        ) -> Result<mavio::protocol::Payload, mavio::prelude::SpecError> {
            if version != mavio::prelude::MavLinkVersion::V2 {
                return Err(mavio::prelude::SpecError::UnsupportedMavLinkVersion {
                    actual: version,
                    minimal: mavio::prelude::MavLinkVersion::V2,
                });
            }
            match self {
                crate::Message::ResetBoard => Err(mavio::prelude::SpecError::NotInDialect(0)), // somewhat scuffed error
                crate::Message::BoardStatus(value) => {
                    let buffer = mavlink_hal::codec::encode_body::<
                        _,
                        { mavlink_hal::macro_util::total_size(StatusCommonMessage::FIELDS) },
                    >(value)
                    .map_err(|_e| mavio::prelude::SpecError::V2PayloadIsTooLong {
                        actual: 0,
                        maximum: 0,
                    })?;
                    // somewhat scuffed error

                    Ok(mavio::protocol::Payload::new(
                        1_000_002,
                        &buffer,
                        mavio::prelude::MavLinkVersion::V2,
                    ))
                }
                _ => todo!("has to also be generated"),
            }
        }
    }
    impl mavio::Message for super::Message {}
}

/// generated
pub mod __message_meta_types {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    pub type ResetBoard = ();
    pub type BoardStatus = StatusCommonMessage;
    pub type CameraPowerStatus = CameraPower;
    pub type RailStatus5V = RailStatus;
    pub type RailStatus24V = RailStatus;
    pub type BoardBuildInfo = BuildInformationCommon;
}

// generated:
impl CanDecode for Message {
    type Error = can_hal::codec::Error;

    fn from_parts(id: StandardId, data: &[u8]) -> Result<Self, Self::Error> {
        match id {
            __message_meta_can_ids::ResetBoard => Ok(Self::ResetBoard),
            __message_meta_can_ids::BoardStatus => {
                can_hal::codec::decode(data).map(Self::BoardStatus)
            }
            __message_meta_can_ids::CameraPowerStatus => {
                can_hal::codec::decode(data).map(Self::CameraPowerStatus)
            }
            _ => Err(Self::Error::UnknownId), // TODO
        }
    }
}

// generated:
impl CanEncode for Message {
    type Error = can_hal::codec::Error;

    fn encode_into(&self, buf: &mut [u8; 64]) -> Result<(StandardId, u8), Self::Error> {
        match self {
            Message::ResetBoard => Ok((__message_meta_can_ids::ResetBoard, 0)),
            Message::BoardStatus(x) => Ok((
                __message_meta_can_ids::BoardStatus,
                can_hal::codec::encode(x, buf)?,
            )),
            _ => todo!(),
        }
    }
}
