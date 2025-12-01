pub use data_macros::*;

#[cfg(feature = "can")]
pub use can_hal;

#[cfg(feature = "bincode")]
pub use bincode;
