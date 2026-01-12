use gated_derive::gated_derive;
use datatypes::actuator::DprState;
use datatypes::status::{SensorStatus, StatusCommonMessage, ValveState};

#[gated_derive(bincode::Encode, bincode::Decode)]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct OxidizerControlBoardStatus {
    pub common: StatusCommonMessage,
    /// State of the ADS1120 device used to read thermocouples
    pub thermocouple_status: SensorStatus,
    /// State of the Keller BUS used to read pressure, indicates whether a single sensor is online or not.
    pub pressure_bus: SensorStatus,
}

#[gated_derive(bincode::Encode, bincode::Decode)]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct OxidizerControlBoardValveStates {
    /// Oxidizer DPR Control loop state
    pub oss_dpr: DprState,
    /// Oxidizer Vent Valve state
    pub oss_vnt_vlv: ValveState,
}
