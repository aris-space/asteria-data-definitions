use can_hal::{CanDecode, CanEncode, FD_DATA_LENGTH, StandardId, next_valid_length};
use datatypes::{BuildInformationCommon, CameraPower, RailStatus, StatusCommonMessage};

/// The default bincode configuration for CAN messages.
/// Uses little-endian, fixed integer encoding, and a 64-byte limit.
const BINCODE_CONFIG: bincode::config::Configuration<
    bincode::config::LittleEndian,
    bincode::config::Varint,
    bincode::config::Limit<FD_DATA_LENGTH>,
> = bincode::config::standard()
    .with_variable_int_encoding()
    .with_limit();

/// Error type for CAN operations.
#[allow(unused)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Encoding CAN message failed: {0}")]
    Encode(#[from] bincode::error::EncodeError),

    #[error("Decoding CAN message failed: {0}")]
    Decode(#[from] bincode::error::DecodeError),

    #[error("Invalid frame or Id sent/received")]
    Other,

    #[error("Unknown Id to decode")]
    UnknownId,
}

pub enum Message {
    ResetPowerBoard,
    // #[can_id(0x020)]
    PowerBoardStatus(StatusCommonMessage),
    CameraPowerStatus(CameraPower),
    RailStatus5V(RailStatus),
    RailStatus24V(RailStatus),
    PowerBoardBuildInfo(BuildInformationCommon),
}

#[allow(non_upper_case_globals)]
pub mod __message_meta_ids {
    use ::can_hal::StandardId;

    pub const ResetPowerBoard: StandardId = StandardId::new(0x001).expect("must be in range");
    pub const PowerBoardStatus: StandardId = StandardId::new(0x002).expect("must be in range");
    pub const CameraPowerStatus: StandardId = StandardId::new(0x003).expect("must be in range");
    pub const RailStatus5V: StandardId = StandardId::new(0x004).expect("must be in range");
    pub const RailStatus24V: StandardId = StandardId::new(0x005).expect("must be in range");
    pub const PowerBoardBuildInfo: StandardId = StandardId::new(0x006).expect("must be in range");

    pub const ALL_IDS: [StandardId; 6] = [
        ResetPowerBoard,
        PowerBoardStatus,
        CameraPowerStatus,
        RailStatus5V,
        RailStatus24V,
        PowerBoardBuildInfo,
    ];
}

pub mod __message_meta_types {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    pub type ResetPowerBoard = ();
    pub type PowerBoardStatus = StatusCommonMessage;
    pub type CameraPowerStatus = CameraPower;
    pub type RailStatus5V = RailStatus;
    pub type RailStatus24V = RailStatus;
    pub type PowerBoardBuildInfo = BuildInformationCommon;
}

// generated:
impl CanDecode for Message {
    type Error = Error;

    fn from_parts(id: StandardId, data: &[u8]) -> Result<Self, Self::Error> {
        match id {
            // random ids
            __message_meta_ids::ResetPowerBoard => Ok(Self::ResetPowerBoard),
            __message_meta_ids::PowerBoardStatus => {
                bincode::decode_from_slice(data, BINCODE_CONFIG)
                    .map(|(x, _)| Self::PowerBoardStatus(x))
                    .map_err(Error::Decode)
            }
            __message_meta_ids::CameraPowerStatus => {
                bincode::decode_from_slice(data, BINCODE_CONFIG)
                    .map(|(x, _)| Self::CameraPowerStatus(x))
                    .map_err(Error::Decode)
            }
            _ => Err(Error::UnknownId), // TODO
        }
    }
}

// generated:
impl CanEncode for Message {
    type Error = Error;

    fn encode_into(&self, buf: &mut [u8; 64]) -> Result<(StandardId, u8), Self::Error> {
        match self {
            Message::ResetPowerBoard => Ok((__message_meta_ids::ResetPowerBoard, 0)),
            Message::PowerBoardStatus(x) => {
                let len = bincode::encode_into_slice(x, buf, BINCODE_CONFIG)?;
                Ok((
                    __message_meta_ids::PowerBoardStatus,
                    next_valid_length(len).ok_or(Error::Other)?,
                ))
            }
            _ => todo!(),
        }
    }
}
