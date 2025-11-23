use can_hal::{CanDecode, CanEncode, StandardId};
use datatypes::{BuildInformationCommon, CameraPower, RailStatus, StatusCommonMessage};

// define_messages! {
pub enum Message {
    // #[can_id(0x001)]
    ResetBoard,
    // #[can_id(0x002)]
    BoardStatus(StatusCommonMessage),
    // #[can_id(0x003)]
    CameraPowerStatus(CameraPower),
    // #[can_id(0x004)]
    RailStatus5V(RailStatus),
    // #[can_id(0x005)]
    RailStatus24V(RailStatus),
    // #[can_id(0x006)]
    BoardBuildInfo(BuildInformationCommon),
}
// }

#[derive(Debug, mavio::derive::Message)]
#[message_id(1_000_001)]
pub struct StatusCommonMessageML {
    /// Number of errors encountered since startup.
    pub errors: u32,
    /// Number of microseconds since the last board restart.
    pub micros_since_restart: u64,
}

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
