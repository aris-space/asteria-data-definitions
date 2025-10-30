#![allow(dead_code)]
#![allow(
    clippy::uninlined_format_args,
    clippy::missing_errors_doc,
    clippy::cast_possible_truncation
)]

use crate::can::{BusError, LoopbackCan, LoopbackCanRx, LoopbackCanTx, TimeoutError, frame};
use hermes_can::messages::Message;
use hermes_can::{CanDecodeError, CanEncodeError, CanMessage};

fn main() {
    // Create a new loopback CAN bus that loops back messages to itself and split it
    // into a transmitter and receiver.
    let can = LoopbackCan::new();
    let (mut tx, mut rx) = can.split();

    // Transmit a reset message. This converts the message to a CAN frame and sends it
    let msg = hermes_can::messages::system_management::ResetAll;
    tx.transmit(msg).expect("failed to transmit message");
    println!("Sending reset message");

    // Receive a message from the bus
    let (msg, timestamp) = rx.recv().expect("failed to receive message");
    println!("{:?} at {:?}", msg, timestamp);

    match msg {
        Message::ResetAll(received) => {
            assert_eq!(received, hermes_can::messages::system_management::ResetAll);
        }
        _ => panic!("Unexpected message"),
    }
}

/// Error type for CAN operations.
#[derive(Debug, thiserror::Error)]
pub enum CanError {
    #[error("CAN bus error")]
    Bus(BusError),

    #[error("CAN timeout")]
    Timeout(TimeoutError),

    #[error("Encoding CAN message failed: {0}")]
    Encode(#[from] CanEncodeError),

    #[error("Decoding CAN message failed: {0}")]
    Decode(#[from] CanDecodeError),

    #[error("Invalid frame or Id sent/received")]
    Other,
}

/// Trait for types that can transmit CAN messages asynchronously.
pub trait CanTransmitter {
    /// Transmit a CAN message.
    ///
    /// Returns `Err(CanError)` if encoding or bus transmission fails.
    fn transmit<M: CanMessage>(&mut self, msg: M) -> Result<(), CanError>;
}

/// Trait for types that can receive CAN messages asynchronously.
pub trait CanReceiver {
    /// Receive the next CAN message and its timestamp.
    ///
    /// Returns `Err(CanError)` if the frame is invalid or bus read fails.
    fn recv(&mut self) -> Result<(Message, frame::Timestamp), CanError>;
}

macro_rules! impl_can_transmitter {
    ($ty:ty) => {
        impl CanTransmitter for $ty {
            fn transmit<M: CanMessage>(&mut self, msg: M) -> Result<(), CanError> {
                let mut buf = [0u8; 64];
                let (id, len) = msg.try_write_into(&mut buf)?;
                let dlc = hermes_can::next_valid_length(len).ok_or(CanError::Other)?;
                let payload = &buf[..dlc];

                let frame = frame::FdFrame::new(id.into(), dlc as u8, payload);
                self.write_fd(&frame).ok_or(CanError::Other)?;
                Ok(())
            }
        }
    };
}

macro_rules! impl_can_receiver {
    ($ty:ty) => {
        impl CanReceiver for $ty {
            fn recv(&mut self) -> Result<(Message, frame::Timestamp), CanError> {
                let frame = self.read_fd().ok_or(CanError::Other)?;
                let id = frame.id.into();
                let payload = &frame.data.bytes()[..frame.length as usize];
                let msg = Message::try_from_parts(id, payload).map_err(CanError::Decode)?;
                Ok((msg, frame::Timestamp::now()))
            }
        }
    };
}

// Implement the CanTransmitter and CanReceiver traits for the LoopbackCanTx and LoopbackCanRx types.
impl_can_transmitter!(LoopbackCanTx);
impl_can_receiver!(LoopbackCanRx);
impl_can_transmitter!(LoopbackCan);
impl_can_receiver!(LoopbackCan);

/// A can stub implementation for testing, that resembles the api of
/// the `embassy_stm32::can` module.
mod can {
    use crate::can::frame::FdFrame;
    use std::sync::mpsc;

    /// A CAN bus that can transmit and receive messages. Loops back messages to itself.
    /// This is useful for testing and simulation.
    pub struct LoopbackCan {
        rx: mpsc::Receiver<FdFrame>,
        tx: mpsc::Sender<FdFrame>,
    }

    /// A CAN receiver that can receive messages from a CAN bus.
    pub struct LoopbackCanRx {
        rx: mpsc::Receiver<FdFrame>,
    }

    /// A CAN transmitter that can send messages to a CAN bus.
    pub struct LoopbackCanTx {
        tx: mpsc::Sender<FdFrame>,
    }

    impl LoopbackCanRx {
        pub fn new(rx: mpsc::Receiver<FdFrame>) -> Self {
            Self { rx }
        }

        pub fn read_fd(&mut self) -> Option<FdFrame> {
            self.rx.recv().ok()
        }
    }

    impl LoopbackCanTx {
        pub fn new(tx: mpsc::Sender<FdFrame>) -> Self {
            Self { tx }
        }

        pub fn write_fd(&mut self, frame: &FdFrame) -> Option<FdFrame> {
            self.tx.send(frame.clone()).ok().map(|()| frame.clone())
        }
    }

    impl LoopbackCan {
        pub fn new() -> Self {
            let (tx, rx) = mpsc::channel();
            Self { rx, tx }
        }

        pub fn split(self) -> (LoopbackCanTx, LoopbackCanRx) {
            (LoopbackCanTx::new(self.tx), LoopbackCanRx::new(self.rx))
        }

        pub fn write_fd(&mut self, frame: &FdFrame) -> Option<FdFrame> {
            self.tx.send(frame.clone()).ok().map(|()| frame.clone())
        }

        pub fn read_fd(&mut self) -> Option<FdFrame> {
            self.rx.recv().ok()
        }
    }

    pub type BusError = ();
    pub type TimeoutError = ();

    pub mod frame {
        use embedded_can::StandardId;
        use std::time::SystemTime;
        pub type Timestamp = SystemTime;

        #[derive(Debug, Clone)]
        pub struct FdFrame {
            pub id: StandardId,
            pub length: u8,
            pub data: FdData,
        }

        impl FdFrame {
            pub fn new(id: StandardId, len: u8, data: &[u8]) -> Self {
                Self {
                    id,
                    length: len,
                    data: FdData::new(data).expect("Invalid data length"),
                }
            }
        }

        #[derive(Debug, Clone)]
        pub struct FdData {
            bytes: [u8; 64],
        }
        impl FdData {
            pub fn new(data: &[u8]) -> Option<Self> {
                if !FdData::is_valid_len(data.len()) {
                    return None;
                }

                let mut bytes = [0; 64];
                bytes[..data.len()].copy_from_slice(data);

                Some(Self { bytes })
            }
            const fn is_valid_len(len: usize) -> bool {
                matches!(len, 0..=8 | 12 | 16 | 20 | 24 | 32 | 48 | 64)
            }

            pub fn bytes(&self) -> &[u8] {
                &self.bytes
            }
        }
    }
}
