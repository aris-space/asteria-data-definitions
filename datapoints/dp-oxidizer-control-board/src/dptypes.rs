#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "pss")]
use data_core::pss;
use datatypes::status::{DprLoopInfo, SensorStatus, StatusCommonMessage};
use datatypes::units::BarG;
use gated_derive::gated_derive;

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct OxidizerControlBoardStatus {
    #[mavlink(flatten)]
    pub common: StatusCommonMessage,
    /// State of the ADS1120 device used to read thermocouples
    #[mavlink(refine)]
    pub thermocouple_status: SensorStatus,
    /// State of the Keller BUS used to read pressure, indicates whether a single sensor is online or not.
    #[mavlink(refine)]
    pub pressure_bus: SensorStatus,
    /// State of the DPR Control Loop
    #[mavlink(refine)]
    pub dpr_info: DprLoopInfo,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[gated_derive(pss::Data)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct OxidizerTankPressure {
    /// Oxidizer Tank pressure sensor 1 in barg.
    #[mavlink(refine)]
    #[pss(name = "oss_tnk_p1", label = "Oxidizer Tank Pressure Sensor 1")]
    pub oxidizer_tank_pressure_sensor_1: BarG,
    /// Oxidizer Tank pressure sensor 2 in barg.
    #[mavlink(refine)]
    #[pss(name = "oss_tnk_p2", label = "Oxidizer Tank Pressure Sensor 2")]
    pub oxidizer_tank_pressure_sensor_2: BarG,
    /// Absolute valued Oxidizer Tank differential pressure in barg.
    #[mavlink(refine)]
    #[pss(name = "oss_tnk_dps", label = "Oxidizer Differential Pressure Sensor")]
    pub oxidizer_tank_differential_pressure: BarG,
}
