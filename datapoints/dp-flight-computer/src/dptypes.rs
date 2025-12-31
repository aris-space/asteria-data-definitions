use bincode::{Decode, Encode};
use datatypes::{ArmingState, StatusCommonMessage};

#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub struct FlightComputerStatus {
    pub common: StatusCommonMessage,
    /// `true` if the flight computer asserts control over the PSS valves via CAN.
    /// In other words, if passive mode is disabled.
    pub has_pss_control: bool,
    /// `Armed` if the flight computer is in the `Armed` state for launch,
    /// which enables REC actuator power and allows an ignition command.
    pub is_software_armed: ArmingState,
}
