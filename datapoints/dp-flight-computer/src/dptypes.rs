use datatypes::status::{ArmingState, StatusCommonMessage};
use gated_derive::gated_derive;

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlightComputerStatus {
    pub common: StatusCommonMessage,
    /// `true` if the flight computer asserts control over the PSS valves via CAN.
    /// In other words, if passive mode is disabled.
    pub has_pss_control: bool,
    /// `Armed` if the flight computer is in the `Armed` state for launch,
    /// which enables REC actuator power and allows an ignition command.
    pub is_software_armed: ArmingState,
}
