#[cfg(feature = "mavlink")]
use data_core::mavlink;
use gated_derive::gated_derive;

/// Enum representing the active power source.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum ActivePowerSource {
    /// Battery power is currently active.
    Battery = 0,
    /// External power is currently active.
    External = 1,
    /// No valid power source is currently active.
    #[default]
    None = 2,
}
