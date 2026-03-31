//! This module provides a helper for sending mavlink frames over UDP,
//! as mavio is not designed for this.
//!
//! This sends entire frames as a single UDP datagram, the same as mavlink-router does.

use std::num::Wrapping;

use mavio::{
    Endpoint, Frame,
    consts::{CHECKSUM_SIZE, HEADER_MAX_SIZE, PAYLOAD_MAX_SIZE, SIGNATURE_LENGTH},
    prelude::V2,
};

#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error("Encoding of message failed: {0}")]
    Encode(#[from] mavio::Error),
    #[error("Sending of frame to UDP failed: {0}")]
    Send(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RecvError {
    #[error("Decoding of message failed: {error}")]
    Decode {
        error: mavio::Error,
        sequence: Wrapping<u8>,
    },
    #[error("Decoding of frame failed: {0}")]
    Framing(#[from] mavio::error::FrameError),
    #[error("Receive of frame from UDP failed: {0}")]
    Recv(#[from] std::io::Error),
}

/// Sends a generic `msg` fully to the given `socket`, using the header value from `endpoint`.
pub fn send_message<M: mavio::Message>(
    msg: &M,
    endpoint: &Endpoint<V2>,
    socket: &std::net::UdpSocket,
) -> Result<(), SendError> {
    // the reason this doesn't use `mavio::Sender`,
    // is that it would send the header and payload in two different calls, potentially resulting in two datagrams.
    let frame = endpoint.next_frame(msg)?;
    let mut buf = [0; HEADER_MAX_SIZE + PAYLOAD_MAX_SIZE + CHECKSUM_SIZE + SIGNATURE_LENGTH];
    let len = frame
        .serialize(&mut buf)
        .expect("`buf` was made to accomodate all messages");
    socket.send(&buf[0..len])?;
    Ok(())
}

/// Reads a single datagram and tries to decode it as a mavlink message.
/// Also returns the sequence number for loss tracking.
pub fn recv_message<M: mavio::Message + mavio::Dialect>(
    socket: &std::net::UdpSocket,
) -> Result<(M, Wrapping<u8>), RecvError> {
    let mut buf = [0; HEADER_MAX_SIZE + PAYLOAD_MAX_SIZE + CHECKSUM_SIZE + SIGNATURE_LENGTH];
    let len = socket.recv(&mut buf)?;
    let buf = &buf[..len];
    // SAFETY: we assume that every datagram is exactly one message.
    // But it is not undefined behaviour if it weren't, it just fails with an error.
    let frame = unsafe { Frame::<V2>::deserialize(buf) }?;
    let sequence = Wrapping(frame.sequence());

    // Validate MAVLink frame before decoding
    frame
        .validate_checksum::<M>()
        .map_err(|error| RecvError::Decode { error, sequence })?;
    let msg = frame
        .decode::<M>()
        .map_err(|error| RecvError::Decode { error, sequence })?;
    Ok((msg, sequence))
}
