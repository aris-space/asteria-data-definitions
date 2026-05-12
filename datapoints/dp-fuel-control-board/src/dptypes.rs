#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "pss")]
use data_core::pss;
use datatypes::status::{DprGainInfo, DprLoopInfo, SensorStatus, StatusCommonMessage};
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
    #[mavlink(refine)]
    pub thermocouple_status: SensorStatus,
    /// State of the Keller BUS used to read pressure, indicates whether a single sensor is online or not.
    #[mavlink(refine)]
    pub pressure_bus: SensorStatus,
    /// State of the DPR Control Loop
    #[mavlink(refine)]
    pub dpr_loop_info: DprLoopInfo,
    /// State of the DPR Control Gain
    #[mavlink(refine)]
    pub dpr_gain_info: DprGainInfo,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[gated_derive(pss::Data)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FuelTankPressure {
    /// Fuel Tank pressure sensor 1 in barg.
    #[mavlink(refine)]
    #[pss(name = "fss_tnk_p1", label = "Fuel Tank Pressure Sensor 1")]
    pub fuel_tank_pressure_sensor_1: BarG,
    /// Fuel Tank pressure sensor 2 in barg.
    #[mavlink(refine)]
    #[pss(name = "fss_tnk_p2", label = "Fuel Tank Pressure Sensor 2")]
    pub fuel_tank_pressure_sensor_2: BarG,
    /// Filtered Fuel Tank pressure used as DPR control loop input in barg.
    #[mavlink(refine)]
    #[pss(name = "fss_tnk_p_flt", label = "Fuel Tank Pressure Filtered")]
    pub fuel_tank_pressure_filtered: BarG,
}
