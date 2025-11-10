use embedded_can::StandardId;

pub const FD_DATA_LENGTH: usize = 64;

/// Maps a length in bytes to the next valid Data Length Code or returns `None`
/// if the length is too long.
#[must_use]
pub const fn next_valid_length(length: usize) -> Option<u8> {
    match length {
        #[allow(clippy::cast_possible_truncation)] // is in range
        0..=8 => Some(length as u8),
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

/// Trait for types that can transmit CAN messages asynchronously.
pub trait TypedCanTransmit {
    type Error: std::error::Error;

    /// Transmit a CAN message.
    ///
    /// Returns `Err(CanError)` if encoding or bus transmission fails.
    #[allow(async_fn_in_trait)] // We'll try and maybe change it later
    async fn transmit<M: CanEncode>(&mut self, msg: M) -> Result<(), Self::Error>;
}

/// Trait for types that can receive CAN messages asynchronously.
pub trait TypedCanReceive {
    type Error: std::error::Error;

    /// Receive the next CAN message and its timestamp.
    ///
    /// Returns `Err(CanError)` if the frame is invalid or bus read fails.
    #[allow(async_fn_in_trait)] // We'll try and maybe change it later
    async fn recv<M: CanDecode>(&mut self) -> Result<M, Self::Error>;
}

/// Implementors of this trait can be created from a CAN id and raw data.
pub trait CanDecode: Sized {
    type Error: std::error::Error;

    /// Tries to decode an instance of `Self` from the provided byte slice.
    ///
    /// Only the bytes actually needed for decoding are consumed; trailing padding is ignored
    /// (as is typical with `bincode`). This allows for safe deserialization of messages from
    /// padded buffers used to match a valid DLC.
    fn from_parts(id: StandardId, data: &[u8]) -> Result<Self, Self::Error>;
}

/// Implementors of this trait can be turned into a CAN frame.
pub trait CanEncode {
    type Error: std::error::Error;

    /// Tries to encode a message `self` into `buf`, returning the id and the number of bytes written.
    ///
    /// The number of bytes written must be a valid data length code.
    fn encode_into(&self, buf: &mut [u8]) -> Result<(StandardId, u8), Self::Error>;
}
