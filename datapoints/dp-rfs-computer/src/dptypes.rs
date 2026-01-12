use datatypes::status::{ArmingState, StatusCommonMessage, ValveState};
use gated_derive::gated_derive;

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct RfsComputerStatus {
    pub common: StatusCommonMessage,
    /// `true` if the remote filling station asserts control over the PSS valves via CAN.
    /// In other words, if passive mode is disabled.
    pub has_pss_control: bool,
    /// `Armed` if the remote filling station is in a `Armed` state for ignition
    /// This is a software arming state of the RFS state machine that is part of the RFS hardware arming
    pub is_ignition_armed: ArmingState,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct RfsComputerValveStates {
    /// The current active state of the normally closed GOX igniter valve.
    pub gox_ign_vlv: ValveState,
    /// The current active state of the normally closed H2 igniter valve.
    pub h2_ign_vlv: ValveState,
    /// The current active state of the normally closed GOX purge valve.
    pub gox_prg_vlv: ValveState,
    /// The current active state of the normally closed H2 purge valve.
    pub h2_prg_vlv: ValveState,
    /// The current active state of the normally off spark plug.
    pub ign_spk_plg: ValveState,
}
