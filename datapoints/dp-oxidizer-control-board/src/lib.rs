use can_hal::{CanDecode, CanEncode, FD_DATA_LENGTH, StandardId, next_valid_length};
use datatypes::{BuildInformationCommon, Celsius, NormallyOpenValve, StatusCommonMessage};

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
    ResetBoard,
    // #[can_id(0x020)]
    BoardStatus(StatusCommonMessage),
    BuildInfo(BuildInformationCommon),
    // maybe generate those multiples
    OxidizerVentControlRFS(NormallyOpenValve),
    OxidizerVentControlFC(NormallyOpenValve),
    OxidizerVentControlECU(NormallyOpenValve),
    /// Didn't actually exist in HERMES
    OxidizerTankTemperature(Celsius),
    // ...
}

pub enum MessageVariant {
    ResetBoard,
    BoardStatus,
    BuildInfo,
    OxidizerVentControlRFS,
    OxidizerVentControlFC,
    OxidizerVentControlECU,
    OxidizerTankTemperature,
}

// generated:
impl CanDecode for Message {
    type Error = Error;

    fn from_parts(id: StandardId, data: &[u8]) -> Result<Self, Self::Error> {
        match id.as_raw() {
            // random ids
            0x00a => Ok(Self::ResetBoard),
            0x020 => bincode::decode_from_slice(data, BINCODE_CONFIG)
                .map(|(x, _)| Self::BoardStatus(x))
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
            Message::ResetBoard => {
                const ID: StandardId =
                    StandardId::new(0x00a).expect("must be in range 0x000-0x7ff");
                Ok((ID, 0))
            }
            Message::BoardStatus(x) => {
                const ID: StandardId =
                    StandardId::new(0x020).expect("must be in range 0x000-0x7ff");
                let len = bincode::encode_into_slice(x, buf, BINCODE_CONFIG)?;
                Ok((ID, next_valid_length(len).ok_or(Error::Other)?))
            }
            _ => todo!(),
        }
    }
}

// generated:
impl MessageVariant {
    pub const ALL_IDS: [StandardId; 2] = [Self::BoardStatus.id(), Self::BuildInfo.id()];

    pub const fn id(self) -> StandardId {
        let raw_id: u16 = match self {
            MessageVariant::BoardStatus => 0x0e0,
            MessageVariant::BuildInfo => 0x101,
            _ => todo!(),
        };
        StandardId::new(raw_id).expect("must be in range 0x000-0x7ff")
    }
}
