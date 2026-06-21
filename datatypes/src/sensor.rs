//! Sensor readings

#[cfg(feature = "derive_mavlink")]
use data_core::mavlink;
use gated_derive::gated_derive;

use crate::actuator::Energizable;

/// A digital input, mostly for proximity sensors.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DigitalIn {
    #[default]
    Off = 0,
    On = 1,
}

impl Energizable for DigitalIn {
    fn is_active(&self) -> bool {
        matches!(self, Self::On)
    }
    fn from_active(is_active: bool) -> Self {
        if is_active { Self::On } else { Self::Off }
    }
}
