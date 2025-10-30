#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

pub mod macros;
pub mod messages;
pub mod option_set;
mod version;

use bincode::config::{Fixint, Limit, LittleEndian, standard};
use bincode::{Decode, Encode};
use embedded_can::StandardId;
use thiserror::Error;
pub use version::{VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH};

const FD_DATA_LENGTH: usize = 64;

/// Maps a length in bytes to the next valid Data Length Code or returns `None`
/// if the length is too long.
#[must_use]
pub const fn next_valid_length(length: usize) -> Option<usize> {
    match length {
        0..=8 => Some(length),
        9..=12 => Some(12),
        13..=16 => Some(16),
        17..=20 => Some(20),
        21..=24 => Some(24),
        25..=32 => Some(32),
        33..=48 => Some(48),
        49..=64 => Some(64),
        65.. => None,
    }
}

/// Error type for encoding CAN messages.
#[derive(Debug, Error)]
pub enum CanEncodeError {
    #[error("Serialization failed: {0}")]
    Serialization(#[from] bincode::error::EncodeError),

    #[error("Message length {0} exceeds allowed CAN frame length")]
    MessageTooLong(usize),
}

/// Error type for decoding CAN messages.
#[derive(Debug, Error)]
pub enum CanDecodeError {
    #[error("Deserialization failed: {0}")]
    Deserialization(#[from] bincode::error::DecodeError),

    #[error("Unknown CAN message id: {0}")]
    UnknownId(u16),

    #[error("Frame has CAN ID 0x{id:03X}, but this message expects 0x{expected:03X}")]
    MismatchedId { expected: u16, id: u16 },
}

// manual implementation because bincode doesn't support `defmt::Format`
#[cfg(feature = "defmt")]
impl defmt::Format for CanEncodeError {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            CanEncodeError::Serialization(err) => {
                defmt::write!(fmt, "Serialization failed: {:?}", defmt::Debug2Format(err));
            }
            CanEncodeError::MessageTooLong(len) => {
                defmt::write!(
                    fmt,
                    "Message length {} exceeds allowed CAN frame length",
                    len
                );
            }
        }
    }
}

// manual implementation because bincode doesn't support `defmt::Format`
#[cfg(feature = "defmt")]
impl defmt::Format for CanDecodeError {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            CanDecodeError::Deserialization(err) => {
                defmt::write!(
                    fmt,
                    "Deserialization failed: {:?}",
                    defmt::Debug2Format(err)
                );
            }
            CanDecodeError::UnknownId(id) => {
                defmt::write!(fmt, "Unknown CAN message id: {}", id);
            }
            CanDecodeError::MismatchedId { expected, id } => {
                defmt::write!(
                    fmt,
                    "Frame has CAN ID 0x{:03X}, but this message expects 0x{:03X}",
                    id,
                    expected
                );
            }
        }
    }
}

/// The default bincode configuration for CAN messages.
/// Uses little-endian, fixed integer encoding, and a 64-byte limit.
const BINCODE_CONFIG: bincode::config::Configuration<LittleEndian, Fixint, Limit<FD_DATA_LENGTH>> =
    standard().with_fixed_int_encoding().with_limit();

/// A trait for types that can be encoded into and decoded from a byte slice.
///
/// This trait abstracts the binary encoding used for CAN message. Implementors can be serialized
/// into a caller-provided buffer and deserialized from a byte slice. The encoding is not required
/// to have a fixed length.
///
/// The blanket implementation uses [`bincode`] with fixed integer encoding and a 64-byte limit for
/// all types that implement [`bincode::Encode`] and [`bincode::Decode`]. This should be preferred
/// for most use cases, but this trait allows for custom serialization implementations if needed.
pub trait CanEncodeDecode: Sized {
    /// Encode `self` into the provided buffer, returning the number of bytes written.
    ///
    /// The result is a compact binary representation of the message, suitable for use as
    /// a CAN frame payload.
    ///
    /// # Errors
    /// Returns [`CanEncodeError`] if serialization fails, for example due to insufficient buffer space.
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, CanEncodeError>;

    /// Decode an instance of `Self` from the provided byte slice.
    ///
    /// Only the bytes actually needed for decoding are consumed; trailing padding is ignored
    /// (as is typical with `bincode`). This allows for safe deserialization of messages from
    /// padded buffers used to match a valid DLC.
    ///
    /// # Errors
    /// Returns [`CanDecodeError`] if deserialization fails due to malformed or incompatible input.
    fn decode_from(bytes: &[u8]) -> Result<Self, CanDecodeError>;
}

/// Blanket implementation for types that implement bincode's `Encode` and `Decode`.
impl<T: Encode + Decode<()>> CanEncodeDecode for T {
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, CanEncodeError> {
        let written = bincode::encode_into_slice(self, buf, BINCODE_CONFIG)?;
        Ok(written)
    }

    fn decode_from(bytes: &[u8]) -> Result<Self, CanDecodeError> {
        let (value, _) = bincode::decode_from_slice(bytes, BINCODE_CONFIG)?;
        Ok(value)
    }
}

/// A CAN message with a fixed 11-bit identifier.
pub trait CanMessage: CanEncodeDecode {
    /// The raw 11-bit identifier for this message.
    const ID_RAW: u16;

    /// A `StandardId` constructed from `ID_RAW`. Checked at compile-time.
    const ID: StandardId = StandardId::new(Self::ID_RAW).expect("Invalid CAN ID");

    /// Construct a message from a CAN ID and data buffer.
    ///
    /// This method verifies that the provided CAN ID matches the expected value and attempts to decode
    /// a message from the data slice. It assumes that the slice contains only the message payload and
    /// that decoding can proceed without issue.
    ///
    /// This can be problematic if padding is present to satisfy the CAN FD data length code (DLC), but
    /// if the decoder consumes only the necessary bytes and ignores any trailing data (as `bincode` does),
    /// this should not interfere with deserialization.
    ///
    /// # Errors
    /// Returns [`CanDecodeError::MismatchedId`] if the provided CAN ID does not match [`Self::ID`],
    /// or another variant of [`CanDecodeError`] if deserialization fails.
    fn try_from_parts(id: StandardId, data: &[u8]) -> Result<Self, CanDecodeError> {
        if id != Self::ID {
            return Err(CanDecodeError::MismatchedId {
                expected: Self::ID.as_raw(),
                id: id.as_raw(),
            });
        }
        Self::decode_from(data)
    }

    /// Serialize the message into a buffer and return the CAN ID and number of bytes written.
    ///
    /// The method encodes the message into the provided buffer and returns the number of bytes
    /// written, which corresponds exactly to the serialized data without any padding.
    ///
    /// # Important
    /// - Some CAN frameworks require that the data length is representable by a valid FD-CAN
    ///   data length code (DLC).
    /// - This function does not verify or add padding to meet DLC requirements; it only ensures
    ///   that the encoded message does not exceed the maximum CAN FD length (64 bytes).
    /// - It is the caller’s responsibility to pad the output if necessary.
    ///
    /// # Errors
    /// Returns [`CanEncodeError::MessageTooLong`] if the encoded message exceeds 64 bytes,
    /// or another variant of [`CanEncodeError`] if serialization fails.
    fn try_write_into(&self, buf: &mut [u8]) -> Result<(StandardId, usize), CanEncodeError> {
        let written = self.encode_into(buf)?;
        if written > FD_DATA_LENGTH {
            return Err(CanEncodeError::MessageTooLong(written));
        }
        Ok((Self::ID, written))
    }
}
