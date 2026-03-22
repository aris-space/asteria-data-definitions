//! Sensor readings

#[cfg(feature = "derive_mavlink")]
use data_core::mavlink;
use gated_derive::gated_derive;

/// The state of a valve that is normally open.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema)]
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DigitalIn {
    #[default]
    Off = 0,
    On = 1,
}
