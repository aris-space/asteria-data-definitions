use datatypes::actuator::DPRValve;
use datatypes::status::{SensorStatus, StatusCommonMessage, ValveState};
use gated_derive::gated_derive;

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FuelControlBoardStatus {
    pub common: StatusCommonMessage,
    /// State of the ADS1120 device used to read thermocouples
    pub thermocouple_status: SensorStatus,
    /// State of the Keller BUS used to read pressure, indicates whether a single sensor is online or not.
    pub pressure_bus: SensorStatus,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FuelControlBoardValveStates {
    /// Fuel DPR Control loop state
    pub fss_dpr: DPRValve,
    /// Pressurization Valve state
    pub prz_vnt_vlv: ValveState,
    /// Fuel Vent Valve state
    pub fss_vnt_vlv: ValveState,
}
