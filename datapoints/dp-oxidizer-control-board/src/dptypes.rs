use bincode::{Decode, Encode};
use datatypes::{DprState, SensorStatus, StatusCommonMessage, ValveState};

#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub struct OxidizerControlBoardStatus {
    pub common: StatusCommonMessage,
    /// State of the ADS1120 device used to read thermocouples
    pub thermocouple_status: SensorStatus,
    /// State of the Keller BUS used to read pressure, indicates whether a single sensor is online or not.
    pub pressure_bus: SensorStatus,
}

#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub struct OxidizerControlBoardValveStates {
    /// Oxidizer DPR Control loop state
    pub oss_dpr: DprState,
    /// Oxidizer Vent Valve state
    pub oss_vnt_vlv: ValveState,
}
