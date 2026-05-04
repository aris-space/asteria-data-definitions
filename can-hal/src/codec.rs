//! This module provides a canonical implementation to decode and encode bincode compatible types.
//! This is mostly used inside of macros.

use crate::next_valid_length;

/// The bincode configuration for CAN messages.
/// Uses little-endian, fixed integer encoding, and a 64-byte limit.
pub const BINCODE_CONFIG: bincode::config::Configuration<
    bincode::config::LittleEndian,
    bincode::config::Fixint,
    bincode::config::Limit<{ crate::FD_DATA_LENGTH }>,
> = bincode::config::standard()
    .with_fixed_int_encoding()
    .with_limit();

/// Error type for CAN operations.
#[allow(unused)]
#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    #[error("Encoding CAN message failed: {0}")]
    Encode(#[cfg_attr(feature = "defmt", defmt(Display2Format))] bincode::error::EncodeError),

    #[error("Decoding CAN message failed: {0}")]
    Decode(#[cfg_attr(feature = "defmt", defmt(Display2Format))] bincode::error::DecodeError),

    #[error("Invalid frame or Id sent/received")]
    Other,

    #[error("Unknown Id to decode")]
    UnknownId,

    #[error("Unknown variant to encode")]
    UnknownVariant,
}

/// Encodes a type as a can message into `buf` and returns the amount of bytes written,
/// st `buf[..result]` is a valid can message.
pub fn encode<T: bincode::Encode>(val: &T, buf: &mut [u8; 64]) -> Result<u8, Error> {
    let len = bincode::encode_into_slice(val, buf, BINCODE_CONFIG).map_err(Error::Encode)?;
    next_valid_length(len).ok_or(Error::Other)
}

/// Decodes a type from can message data.
pub fn decode<T: bincode::Decode<()>>(data: &[u8]) -> Result<T, Error> {
    bincode::decode_from_slice(data, BINCODE_CONFIG)
        .map_err(Error::Decode)
        .map(|(x, _)| x)
}
