#[cfg(feature = "mavlink")]
use data_core::mavlink;
use datatypes::status::{ArmingState, SensorStatus, StatusCommonMessage, ValveState};
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
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EngineControlBoardValveStates {
    /// Fuel Main Valve state
    #[mavlink(flatten)]
    pub fss_mnl_vlv: ValveState,
    /// Oxidizer Main Valve state
    #[mavlink(flatten)]
    pub oss_mnl_vlv: ValveState,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnginePressure {
    /// Engine Chamber pressure in barg.
    #[mavlink(flatten)]
    pub eng_cc_p: BarG,
    /// Injector Fuel pressure in barg.
    #[mavlink(flatten)]
    pub fss_inj_p: BarG,
    /// Injector Oxidizer pressure in barg.
    #[mavlink(flatten)]
    pub oss_inj_p: BarG,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EngineBayTemperature {
    /// Oxidizer Runline temperature in °C.
    #[mavlink(flatten)]
    pub oss_rnl_t: Celsius,
    /// Oxidizer Tank Bottom temperature in °C.
    #[mavlink(flatten)]
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

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
// #[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ThrustCurveConfig {
    /// The thrust curve, consisting of 25 points.
    // #[mavlink(flatten)]
    pub thrust_curve: [ThrustPoint; 25],
    /// The length of the thrust curve in points (0-25).
    pub thrust_curve_length: u8,
}
impl ThrustCurveConfig {
    /// Calculates a unique hash for the entire thrust curve array.
    /// This uses `SipHasher` which is deprecated, but still used because we don't care about
    /// cryptographic security here.
    #[must_use]
    pub fn calculate_unique_hash(&self) -> u64 {
        #![allow(deprecated)]
        use core::hash::{Hash, Hasher, SipHasher};
        let mut hasher = SipHasher::new();
        self.thrust_curve[..self.thrust_curve_length as usize].hash(&mut hasher);
        hasher.finish()
    }
}
