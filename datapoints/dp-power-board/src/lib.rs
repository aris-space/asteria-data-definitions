use data_macros::{CanMessage, Message};
use datatypes::{BuildInformationCommon, CameraPower, RailStatus, StatusCommonMessage};

#[derive(Message, CanMessage)]
pub enum Message {
    /// Tells the board that it should reset itself.
    ///
    /// # CAN
    /// - **Kind**: Event message
    /// - **Sent by**: Flight computer
    #[can(id = 0x001)]
    ResetBoard,

    /// Sent by the power board to give its current health status.
    ///
    /// # CAN
    /// - **Kind**: Periodic at 1Hz
    /// - **Sent by**: Power Board
    #[can(id = 0x002)]
    // #[mavlink_id(1_000_002)]
    // #[mavlink_name("POWERBOARD_STATUS")]
    BoardStatus(StatusCommonMessage),

    #[can(id = 0x003)]
    // #[mavlink_id(2_000_003)]
    CameraPowerStatus(CameraPower),
    #[can(id = 0x004)]
    // #[mavlink_id(1_000_004)]
    RailStatus5V(RailStatus),
    #[can(id = 0x005)]
    // #[mavlink_id(1_000_005)]
    RailStatus24V(RailStatus),
    #[can(id = 0x006)]
    // #[mavlink_id(1_000_006)]
    BoardBuildInfo(BuildInformationCommon),
}
// }
/// generated
#[allow(non_upper_case_globals)]
pub mod __message_meta_mavlink {
    use datatypes::StatusCommonMessage;
    use mavlink_hal::description::{DescribeFieldSet, DescribeMessages};

    // generated: specify which messages exist.
    impl DescribeMessages for super::Message {
        const MESSAGES: &[mavlink_hal::description::Message] = &[
            mavlink_hal::description::Message {
                id: mavlink_hal::macro_util::sanitize_id(1_000_002),
                name: "POWERBOARD_STATUS", // defaults to the uppercased variant name unless overriden with `mavlink_name` attr.
                description: "Sent by the power board to give its current health status ...", // from doc comment
                fields: StatusCommonMessage::FIELDS, // because inner must derive `DescribeFieldSet`
            },
            //...
        ];
    }
    const _: () = mavlink_hal::macro_util::check_unique_ids(super::Message::MESSAGES);
    impl mavio::protocol::MessageSpec for super::Message {
        fn id(&self) -> mavio::protocol::MessageId {
            match self {
                crate::Message::ResetBoard => {
                    panic!("ResetBoard is not a mavlink compatible variant") // FIXME: a non-panicky way would also be nice
                }
                crate::Message::BoardStatus(_) => 1_000_002,
                _ => todo!(),
            }
        }
        #[inline]
        fn min_supported_mavlink_version(&self) -> mavio::prelude::MavLinkVersion {
            mavio::prelude::MavLinkVersion::V2
        }
        fn crc_extra(&self) -> mavio::protocol::CrcExtra {
            match self {
                crate::Message::ResetBoard => {
                    panic!("ResetBoard is not a mavlink compatible variant")
                }
                crate::Message::BoardStatus(_) => mavlink_hal::macro_util::calc_extra_crc(
                    "POWERBOARD_STATUS",
                    StatusCommonMessage::FIELDS,
                ),
                _ => todo!(),
            }
        }
    }
    impl mavio::protocol::IntoPayload for super::Message {
        #[inline]
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
                    // somewhat scuffed error, but there are no better ones. Mavio just panics.

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
    impl TryFrom<&mavio::protocol::Payload> for super::Message {
        type Error = mavio::prelude::SpecError;
        #[inline]
        fn try_from(payload: &mavio::protocol::Payload) -> Result<Self, Self::Error> {
            if payload.version() != mavio::prelude::MavLinkVersion::V2 {
                return Err(mavio::prelude::SpecError::UnsupportedMavLinkVersion {
                    actual: payload.version(),
                    minimal: mavio::prelude::MavLinkVersion::V2,
                });
            }
            match payload.id() {
                1_000_002 => mavlink_hal::codec::decode_body::<
                    _,
                    { mavlink_hal::macro_util::total_size(StatusCommonMessage::FIELDS) },
                >(payload.bytes())
                .map(Self::BoardStatus)
                .map_err(|_e| mavio::prelude::SpecError::V2PayloadIsTooLong {
                    actual: 0,
                    maximum: 0,
                }), // there are no good fitting errors :/
                2_000_003 => todo!(),
                1_000_004 => todo!(),
                1_000_005 => todo!(),
                1_000_006 => todo!(),
                id => Err(mavio::prelude::SpecError::NotInDialect(id)),
            }
        }
    }
    impl mavio::Message for super::Message {}
}



}
