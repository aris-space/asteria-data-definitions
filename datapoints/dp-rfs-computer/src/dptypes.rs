#[cfg(feature = "mavlink")]
use data_core::mavlink;
use datatypes::status::{ArmingState, SensorStatus, StatusCommonMessage};
use gated_derive::gated_derive;

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RfsComputerStatus {
    #[mavlink(flatten)]
    pub common: StatusCommonMessage,
    /// `Armed` if the remote filling station is in a `Armed` state for ignition
    /// This is a software arming state of the RFS state machine that is part of the RFS hardware arming
    #[mavlink(refine)]
    pub is_ignition_armed: ArmingState,
    /// `Online` if connection exists, `Offline` otherwise.
    #[mavlink(refine)]
    pub controllino_connection: SensorStatus,
}
