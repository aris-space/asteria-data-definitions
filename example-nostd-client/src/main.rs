#![no_std]
#![allow(dead_code)]

use data_core::{CanDecode, CanEncode, sparse_decodable_can_message};

#[derive(CanDecode, CanEncode)]
#[can(dispatch)]
enum AnyMessage {
    PowerBoard(dp_power_board::Message),
    OxidizerControlBoard(dp_oxidizer_control_board::Message),
}

pub mod sparse_decode {
    use crate::sparse_decodable_can_message;
    sparse_decodable_can_message! {
        enum ReceivedMessage {
            PowerBoardBuildInfo(dp_power_board::Message::BuildInfo),
            PowerBoardStatus(dp_power_board::Message::BoardStatus),
            OxidizerControlBoardStatus(dp_oxidizer_control_board::Message::BoardStatus)
        }
    }
}

fn main() {}
