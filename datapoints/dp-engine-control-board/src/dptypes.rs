use bincode::{Decode, Encode};
use datatypes::{ArmingState, BarG, Celsius, SensorStatus, StatusCommonMessage, ValveState};

#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub struct EngineControlBoardStatus {
    pub common: StatusCommonMessage,
    /// State of the ADS1120 device used to read thermocouples
    pub thermocouple_status: SensorStatus,
    /// Hash of the Thrust Curve used for this flight. Defined by
    pub thrust_curve_hash: u64,
    /// `Armed` if the engine is armed and the main valves receive power, ie engine arming Pin is pulled
    pub armed: ArmingState,
}

#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub struct EngineControlBoardValveStates {
    /// Fuel Main Valve state
    pub fss_mnl_vlv: ValveState,
    /// Oxidizer Main Valve state
    pub oss_mnl_vlv: ValveState,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct EnginePressure {
    /// Engine Chamber pressure in barg.
    pub eng_cc_p: BarG,
    /// Injector Fuel pressure in barg.
    pub fss_inj_p: BarG,
    /// Injector Oxidizer pressure in barg.
    pub oss_inj_p: BarG,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct EngineBayTemperature {
    /// Oxidizer Runline temperature in °C.
    pub oss_rnl_t: Celsius,
    /// Oxidizer Tank Bottom temperature in °C.
    pub oss_tnk_t: Celsius,
}

#[derive(Copy, Clone, Debug, PartialEq, Default, Encode, Decode, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThrustPoint {
    /// Thrust level in percentage (0-100).
    pub thrust: u8,
    /// Time in 100ms steps after ignition when this thrust level should be reached.
    pub time: u8,
}

#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub struct ThrustCurveConfig {
    /// The thrust curve, consisting of 25 points.
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
