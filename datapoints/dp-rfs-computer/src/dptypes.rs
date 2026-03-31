#[cfg(feature = "mavlink")]
use data_core::mavlink;
use datatypes::status::{ArmingState, StatusCommonMessage, ValveState};
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
    /// `true` if the remote filling station asserts control over the PSS valves via CAN.
    /// In other words, if passive mode is disabled.
    pub has_pss_control: bool,
    /// `Armed` if the remote filling station is in a `Armed` state for ignition
    /// This is a software arming state of the RFS state machine that is part of the RFS hardware arming
    #[mavlink(refine)]
    pub is_ignition_armed: ArmingState,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RfsComputerValveStates {
    /// The current active state of the normally closed GOX igniter valve.
    #[mavlink(refine)]
    pub gox_ign_vlv: ValveState,
    /// The current active state of the normally closed H2 igniter valve.
    #[mavlink(refine)]
    pub h2_ign_vlv: ValveState,
    /// The current active state of the normally closed GOX purge valve.
    #[mavlink(refine)]
    pub gox_prg_vlv: ValveState,
    /// The current active state of the normally closed H2 purge valve.
    #[mavlink(refine)]
    pub h2_prg_vlv: ValveState,
    /// The current active state of the normally off spark plug.
    #[mavlink(refine)]
    pub ign_spk_plg: ValveState,
}
