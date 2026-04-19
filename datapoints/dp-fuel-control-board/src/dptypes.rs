#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "pss")]
use data_core::pss;
use datatypes::actuator::DPRValve;
use datatypes::status::{SensorStatus, StatusCommonMessage, ValveState};
use datatypes::units::BarG;
use gated_derive::gated_derive;

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FuelControlBoardStatus {
    #[mavlink(flatten)]
    pub common: StatusCommonMessage,
    /// State of the ADS1120 device used to read thermocouples
    #[mavlink(flatten)]
    pub thermocouple_status: SensorStatus,
    /// State of the Keller BUS used to read pressure, indicates whether a single sensor is online or not.
    #[mavlink(flatten)]
    pub pressure_bus: SensorStatus,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FuelControlBoardValveStates {
    /// Fuel DPR Control loop state
    #[mavlink(flatten)]
    pub fuel_dpr_valve: DPRValve,
    /// Pressurization Valve state
    #[mavlink(flatten)]
    pub pressurization_vent_valve: ValveState,
    /// Fuel Vent Valve state
    #[mavlink(flatten)]
    pub fuel_vent_valve: ValveState,
}
