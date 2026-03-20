#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "pss")]
use data_core::pss;
use datatypes::actuator::DPRValve;
use datatypes::status::{SensorStatus, StatusCommonMessage, ValveState};
use gated_derive::gated_derive;

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[gated_derive(pss::Data)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct OxidizerControlBoardStatus {
    #[mavlink(flatten)]
    pub common: StatusCommonMessage,
    /// State of the ADS1120 device used to read thermocouples
    #[mavlink(refine)]
    #[pss(
        name = "thermocouple_status",
        label = "Thermocouple Status",
        kind = "digital_sensor"
    )]
    pub thermocouple_status: SensorStatus,
    /// State of the Keller BUS used to read pressure, indicates whether a single sensor is online or not.
    #[mavlink(refine)]
    #[pss(name = "pressure_bus", label = "Pressure Bus", kind = "digital_sensor")]
    pub pressure_bus: SensorStatus,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[gated_derive(pss::Data)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct OxidizerControlBoardValveStates {
    /// Oxidizer DPR Control loop state
    #[mavlink(refine)]
    #[pss(
        name = "oss_dpr",
        label = "Oxidizer Control Loop State",
        kind = "dpr_actuator"
    )]
    pub oss_dpr: DPRValve,
    /// Oxidizer Vent Valve state
    #[mavlink(refine)]
    #[pss(
        name = "oss_vnt_vlv",
        label = "Oxidizer Vent Valve State",
        kind = "digital_sensor"
    )]
    pub oss_vnt_vlv: ValveState,
}
