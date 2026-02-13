#![cfg_attr(not(feature = "std"), no_std)]
//! This crate should be used for using the macros from the `data_macros` crate.
//! The reason for this is, that this crate reexports all crates needed by the macros
//! (unless specified otherwhise by the macro itself)

pub use data_macros::Message;

#[cfg(feature = "can")]
pub mod can {
    pub use can_hal as hal;
    pub use data_macros::{
        CanDecode as Decode, CanEncode as Encode, CanMessage as Message,
        sparse_decodable_can_message, sparse_encodable_can_message,
    };
}

#[cfg(feature = "mavlink")]
pub mod mavlink {
    pub use data_macros::{
        MavlinkDecode as Decode, MavlinkEncode as Encode, MavlinkEnum as Enum,
        MavlinkFieldSet as FieldSet, MavlinkMessage as Message,
    };
    pub use mavio;
    pub use mavlink_hal as hal;
}

#[cfg(feature = "bincode")]
pub use bincode;
