use can_hal::{CanDecode, CanEncode, StandardId};
use datatypes::{BuildInformationCommon, Celsius, NormallyOpenValve, StatusCommonMessage};

// define_messages! {
pub enum Message {
    // #[can_id(0x021)]
    ResetBoard,
    // #[can_id(0x022)]
    BoardStatus(StatusCommonMessage),
    // #[can_id(0x023)]
    BuildInfo(BuildInformationCommon),
    // maybe generate those multiples
    // #[can_id(0x024)]
    OxidizerVentControlRFS(NormallyOpenValve),
    // #[can_id(0x025)]
    OxidizerVentControlFC(NormallyOpenValve),
    // #[can_id(0x026)]
    OxidizerVentControlECU(NormallyOpenValve),
    // #[can_id(0x027)]
    /// Didn't actually exist in HERMES
    OxidizerTankTemperature(Celsius),
    // ...
}
// }

/// generated
pub mod __message_meta_types {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    pub type ResetBoard = ();
    pub type BoardStatus = StatusCommonMessage;
    pub type BuildInfo = BuildInformationCommon;
    pub type OxidizerVentControlRFS = NormallyOpenValve;
    pub type OxidizerVentControlFC = NormallyOpenValve;
    pub type OxidizerVentControlECU = NormallyOpenValve;
    pub type OxidizerTankTemperature = Celsius;
}

// generated:
#[allow(non_upper_case_globals)]
pub mod __message_meta_can_ids {
    use ::can_hal::StandardId;

    pub const ResetBoard: StandardId = StandardId::new(0x021).expect("must be in range");
    pub const BoardStatus: StandardId = StandardId::new(0x022).expect("must be in range");
    pub const BuildInfo: StandardId = StandardId::new(0x023).expect("must be in range");
    pub const OxidizerVentControlRFS: StandardId =
        StandardId::new(0x024).expect("must be in range");
    pub const OxidizerVentControlFC: StandardId = StandardId::new(0x025).expect("must be in range");
    pub const OxidizerVentControlECU: StandardId =
        StandardId::new(0x026).expect("must be in range");
    pub const OxidizerTankTemperature: StandardId =
        StandardId::new(0x027).expect("must be in range");

    pub const ALL_IDS: [StandardId; 7] = [
        ResetBoard,
        BoardStatus,
        BuildInfo,
        OxidizerVentControlRFS,
        OxidizerVentControlFC,
        OxidizerVentControlECU,
        OxidizerTankTemperature,
    ];
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
            __message_meta_can_ids::BuildInfo => can_hal::codec::decode(data).map(Self::BuildInfo),
            __message_meta_can_ids::OxidizerVentControlRFS => {
                can_hal::codec::decode(data).map(Self::OxidizerVentControlRFS)
            }
            __message_meta_can_ids::OxidizerVentControlFC => {
                can_hal::codec::decode(data).map(Self::OxidizerVentControlFC)
            }
            __message_meta_can_ids::OxidizerVentControlECU => {
                can_hal::codec::decode(data).map(Self::OxidizerVentControlECU)
            }
            __message_meta_can_ids::OxidizerTankTemperature => {
                can_hal::codec::decode(data).map(Self::OxidizerTankTemperature)
            }
            _ => Err(Self::Error::UnknownId),
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
            Message::BuildInfo(x) => Ok((
                __message_meta_can_ids::BuildInfo,
                can_hal::codec::encode(x, buf)?,
            )),
            Message::OxidizerVentControlRFS(x) => Ok((
                __message_meta_can_ids::OxidizerVentControlRFS,
                can_hal::codec::encode(x, buf)?,
            )),
            Message::OxidizerVentControlFC(x) => Ok((
                __message_meta_can_ids::OxidizerVentControlFC,
                can_hal::codec::encode(x, buf)?,
            )),
            Message::OxidizerVentControlECU(x) => Ok((
                __message_meta_can_ids::OxidizerVentControlECU,
                can_hal::codec::encode(x, buf)?,
            )),
            Message::OxidizerTankTemperature(x) => Ok((
                __message_meta_can_ids::OxidizerTankTemperature,
                can_hal::codec::encode(x, buf)?,
            )),
        }
    }
}
