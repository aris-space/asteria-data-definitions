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

// generated:
impl CanDecode for Message {
    type Error = Error;

    fn from_parts(id: StandardId, data: &[u8]) -> Result<Self, Self::Error> {
        match id.as_raw() {
            // random ids
            0x00a => Ok(Self::ResetPowerBoard),
            0x020 => bincode::decode_from_slice(data, BINCODE_CONFIG)
                .map(|(x, _)| Self::PowerBoardStatus(x))
                .map_err(Error::Decode),
            0x030 => bincode::decode_from_slice(data, BINCODE_CONFIG)
                .map(|(x, _)| Self::CameraPowerStatus(x))
                .map_err(Error::Decode),
            _ => Err(Error::UnknownId), // TODO
        }
    }
}

// generated:
impl CanEncode for Message {
    type Error = Error;

    fn encode_into(&self, buf: &mut [u8; 64]) -> Result<(StandardId, u8), Self::Error> {
        match self {
            Message::ResetPowerBoard => {
                const ID: StandardId =
                    StandardId::new(0x00a).expect("must be in range 0x000-0x7ff");
                Ok((ID, 0))
            }
            Message::PowerBoardStatus(x) => {
                const ID: StandardId =
                    StandardId::new(0x020).expect("must be in range 0x000-0x7ff");
                let len = bincode::encode_into_slice(x, buf, BINCODE_CONFIG)?;
                Ok((ID, next_valid_length(len).ok_or(Error::Other)?))
            }
            _ => todo!(),
        }
    }
}
