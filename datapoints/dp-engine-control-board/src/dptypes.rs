#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "pss")]
use data_core::pss;
use datatypes::status::{ArmingState, SensorStatus, StatusCommonMessage};
use datatypes::units::{BarG, Celsius};
use gated_derive::gated_derive;

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EngineControlBoardStatus {
    #[mavlink(flatten)]
    pub common: StatusCommonMessage,
    /// State of the ADS1120 device used to read thermocouples
    #[mavlink(flatten)]
    pub thermocouple_status: SensorStatus,
    /// Hash of the Thrust Curve used for this flight. Defined by
    pub thrust_curve_hash: u64,
    /// `Armed` if the engine is armed and the main valves receive power, ie engine arming Pin is pulled
    #[mavlink(flatten)]
    pub armed: ArmingState,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[gated_derive(pss::Data)]
#[derive(Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnginePressure {
    /// Engine Chamber pressure in barg.
    #[mavlink(refine)]
    #[pss(name = "eng_cc_p", label = "Chamber Pressure Sensor")]
    pub eng_cc_p: BarG,
    /// Injector Fuel pressure in barg.
    #[mavlink(refine)]
    #[pss(name = "eng_inj_p", label = "Injector Fuel Pressure Sensor")]
    pub eng_inj_p: BarG,
    /// Injector Oxidizer pressure in barg.
    #[mavlink(refine)]
    #[pss(name = "oss_inj_p", label = "Injector Oxidizer Pressure Sensor")]
    pub oss_inj_p: BarG,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[gated_derive(pss::Data)]
#[derive(Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EngineBayTemperature {
    /// Injector Fuel temperature in °C.
    #[mavlink(refine)]
    #[pss(name = "eng_inj_t", label = "Injector Fuel Temperature Sensor")]
    pub eng_inj_t: Celsius,
    /// Oxidizer Tank Bottom temperature in °C.
    #[mavlink(refine)]
    #[pss(name = "oss_tnk_t", label = "Oxidizer Tank Temperature Sensor")]
    pub oss_tnk_t: Celsius,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Copy, Clone, Debug, PartialEq, Default, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ThrustPoint {
    /// Thrust level in percentage (0-100).
    pub thrust: u8,
    /// Time in 100ms steps after ignition when this thrust level should be reached.
    pub time: u8,
}
