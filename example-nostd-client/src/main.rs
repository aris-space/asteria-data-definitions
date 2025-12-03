#![no_std]
#![allow(dead_code)]

use data_core::{CanDecode, CanEncode, sparse_decodable_can_message};

#[derive(CanDecode, CanEncode)]
#[can(dispatch)]
enum AnyMessage {
    PowerBoard(dp_power_board::Message),
    OxidizerControlBoard(dp_oxidizer_control_board::Message),
}

// mod generated {
//     use super::AnyMessage;
//     use can_hal::{CanDecode, CanEncode};

//     #[derive(Debug, thiserror::Error)]
//     pub enum AnyError {
//         #[error(transparent)]
//         PowerBoard(<dp_power_board::Message as CanEncode>::Error),
//         #[error(transparent)]
//         OxidizerControlBoard(<dp_oxidizer_control_board::Message as CanEncode>::Error),
//     }
//     impl CanEncode for AnyMessage {
//         type Error = AnyError;

//         fn encode_into(
//             &self,
//             buf: &mut [u8; 64],
//         ) -> Result<(can_hal::StandardId, u8), Self::Error> {
//             match self {
//                 AnyMessage::PowerBoard(message) => {
//                     message.encode_into(buf).map_err(AnyError::PowerBoard)
//                 }
//                 AnyMessage::OxidizerControlBoard(message) => message
//                     .encode_into(buf)
//                     .map_err(AnyError::OxidizerControlBoard),
//             }
//         }
//     }

//     // if #[derive(CanDecode)]
//     // impl CanDecode for AnyMessage {
//     //     type Error = AnyError;

//     //     fn from_parts(id: can_hal::StandardId, data: &[u8]) -> Result<Self, Self::Error> {
//     //         if dp_power_board::__message_meta_can_ids::ALL_IDS.contains(&id) {
//     //             return dp_power_board::Message::from_parts(id, data)
//     //                 .map(Self::PowerBoard)
//     //                 .map_err(AnyError::PowerBoard);
//     //         }
//     //         if dp_oxidizer_control_board::__message_meta_can_ids::ALL_IDS.contains(&id) {
//     //             return dp_oxidizer_control_board::Message::from_parts(id, data)
//     //                 .map(Self::OxidizerControlBoard)
//     //                 .map_err(AnyError::OxidizerControlBoard);
//     //         }
//     //         todo!()
//     //     }
//     // }
// }
pub mod other {
    use crate::sparse_decodable_can_message;
    sparse_decodable_can_message! {
        enum ReceivedMessage {
            ResetPowerBoard(dp_power_board::Message::ResetBoard),
            PowerBoardStatus(dp_power_board::Message::BoardStatus),
            OxidizerControlBoardStatus(dp_oxidizer_control_board::Message::BoardStatus)
        }
    }
}

// mod generated2 {

//     use can_hal::{CanDecode, StandardId};

//     use crate::ReceivedMessage;

//     impl CanDecode for ReceivedMessage {
//         type Error = can_hal::codec::Error;

//         #[allow(non_upper_case_globals)]
//         fn from_parts(id: StandardId, data: &[u8]) -> Result<Self, Self::Error> {
//             match id {
//                 dp_power_board::__message_meta_can_ids::ResetBoard => Ok(Self::ResetPowerBoard),
//                 dp_power_board::__message_meta_can_ids::BoardStatus => {
//                     can_hal::codec::decode(data).map(Self::PowerBoardStatus)
//                 }
//                 dp_oxidizer_control_board::__message_meta_can_ids::BoardStatus => {
//                     can_hal::codec::decode(data).map(Self::OxidizerControlBoardStatus)
//                 }
//                 _ => Err(Self::Error::UnknownId),
//             }
//         }
//     }

//     impl ReceivedMessage {
//         pub const ALL_IDS: [StandardId; 3] = [
//             dp_power_board::__message_meta_can_ids::ResetBoard,
//             dp_power_board::__message_meta_can_ids::BoardStatus,
//             dp_oxidizer_control_board::__message_meta_can_ids::BoardStatus,
//         ];
//     }

//     // todo: maybe also CanEncode
// }

fn main() {}
