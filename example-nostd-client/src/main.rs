#![no_std]
#![allow(dead_code)]

use can_hal::FD_DATA_LENGTH;
use datatypes::StatusCommonMessage;

/// The default bincode configuration for CAN messages.
/// Uses little-endian, fixed integer encoding, and a 64-byte limit.
const BINCODE_CONFIG: bincode::config::Configuration<
    bincode::config::LittleEndian,
    bincode::config::Varint,
    bincode::config::Limit<FD_DATA_LENGTH>,
> = bincode::config::standard()
    .with_variable_int_encoding()
    .with_limit();

// #[derive(CanEncode)]
// #[can_encode(dispatch)]
enum AnyMessage {
    PowerBoard(dp_power_board::Message),
    OxidizerControlBoard(dp_oxidizer_control_board::Message),
}

mod generated {
    use super::AnyMessage;
    use can_hal::{CanDecode, CanEncode};

    #[derive(Debug, thiserror::Error)]
    pub enum AnyError {
        #[error(transparent)]
        PowerBoard(<dp_power_board::Message as CanEncode>::Error),
        #[error(transparent)]
        OxidizerControlBoard(<dp_oxidizer_control_board::Message as CanEncode>::Error),
    }
    impl CanEncode for AnyMessage {
        type Error = AnyError;

        fn encode_into(
            &self,
            buf: &mut [u8; 64],
        ) -> Result<(can_hal::StandardId, u8), Self::Error> {
            match self {
                AnyMessage::PowerBoard(message) => {
                    message.encode_into(buf).map_err(AnyError::PowerBoard)
                }
                AnyMessage::OxidizerControlBoard(message) => message
                    .encode_into(buf)
                    .map_err(AnyError::OxidizerControlBoard),
            }
        }
    }

    // if #[derive(CanDecode)]
    impl CanDecode for AnyMessage {
        type Error = AnyError;

        fn from_parts(id: can_hal::StandardId, data: &[u8]) -> Result<Self, Self::Error> {
            if dp_power_board::__message_meta_can_ids::ALL_IDS.contains(&id) {
                return dp_power_board::Message::from_parts(id, data)
                    .map(Self::PowerBoard)
                    .map_err(AnyError::PowerBoard);
            }
            todo!()
        }
    }
}

// #[sparse_can_decode]
enum ReceivedMessage {
    // needs to know whether it is unit type
    /// `PowerBoardStatus = dp_power_board::Message::PowerBoardStatus(..)`
    PowerBoardPowerBoardStatus(dp_power_board::__message_meta_types::PowerBoardStatus),
    /// `dp_oxidizer_control_board::Message::BoardStatus(StatusCommonMessage)`
    OxidizerControlBoardBoardStatus(StatusCommonMessage),
    // ...
}

mod generated2 {

    use can_hal::{CanDecode, StandardId};

    use crate::{BINCODE_CONFIG, ReceivedMessage};

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("Decoding CAN message failed: {0}")]
        Decode(bincode::error::DecodeError),
        #[error("unknown id")]
        UnknownId,
    }

    impl CanDecode for ReceivedMessage {
        type Error = Error;

        #[allow(non_upper_case_globals)]
        fn from_parts(id: StandardId, data: &[u8]) -> Result<Self, Self::Error> {
            const OxidizerControlBoardBoardStatus_ID: StandardId =
                dp_oxidizer_control_board::MessageVariant::BoardStatus.id();
            match id {
                dp_power_board::__message_meta_can_ids::PowerBoardStatus => {
                    bincode::decode_from_slice(data, BINCODE_CONFIG)
                        .map(|(x, _)| Self::PowerBoardPowerBoardStatus(x))
                        .map_err(Error::Decode)
                }
                OxidizerControlBoardBoardStatus_ID => {
                    bincode::decode_from_slice(data, BINCODE_CONFIG)
                        .map(|(x, _)| Self::OxidizerControlBoardBoardStatus(x))
                        .map_err(Error::Decode)
                }
                _ => Err(Error::UnknownId),
            }
        }
    }

    impl ReceivedMessage {
        pub const ALL_IDS: [StandardId; 2] = [
            dp_power_board::__message_meta_can_ids::PowerBoardStatus,
            dp_oxidizer_control_board::MessageVariant::BoardStatus.id(),
        ];
    }
}

fn main() {}
