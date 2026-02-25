//! Actuator control types and configurations.

use gated_derive::gated_derive;

/// A trait for an actuator, such as a valve, that can be set by a single float value.
/// The value usually is the openness for valves, in the 0-1 range.
pub trait Settable: TryFrom<f32, Error = OutOfRange> + Into<f32> {
    #[inline]
    fn set_value(&mut self, value: f32) -> Result<(), OutOfRange> {
        *self = Self::try_from(value)?;
        Ok(())
    }
}

/// Error for [`Settable`].
#[derive(Debug, Clone, Copy)]
#[gated_derive(thiserror::Error(feature = "derive_serde"))] // required for [serde(try_from = ...)]
#[error("The supplied value is out of range for this settable")]
pub struct OutOfRange;

// TODO:
// /// Allows boxing a Watchable of a settable for a boxed trait object.
// impl<T: Settable> Settable for watchable::Watchable<T> {
//     fn set_value(&mut self, value: f32) -> Result<(), OutOfRange> {
//         self.write().set_value(value)
//     }
// }

/// A trait for an actuator, which can be energized using a binary signal.
pub trait Energizable {
    fn is_active(&self) -> bool;
    fn is_inactive(&self) -> bool {
        !self.is_active()
    }
    fn from_active(is_active: bool) -> Self;
}

/// The state of a valve that is normally open.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum NormallyOpenValve {
    #[default]
    Open,
    Closed,
}

impl NormallyOpenValve {
    pub fn is_open(&self) -> bool {
        matches!(self, Self::Open)
    }
    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Closed)
    }
}
impl Energizable for NormallyOpenValve {
    fn is_active(&self) -> bool {
        matches!(self, Self::Closed)
    }
    fn from_active(is_active: bool) -> Self {
        if is_active { Self::Closed } else { Self::Open }
    }
}
impl TryFrom<f32> for NormallyOpenValve {
    type Error = OutOfRange;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        match value {
            0.0 => Ok(NormallyOpenValve::Closed),
            1.0 => Ok(NormallyOpenValve::Open),
            _ => Err(OutOfRange),
        }
    }
}
impl From<NormallyOpenValve> for f32 {
    fn from(value: NormallyOpenValve) -> f32 {
        match value {
            NormallyOpenValve::Open => 1.0,
            NormallyOpenValve::Closed => 0.0,
        }
    }
}
impl Settable for NormallyOpenValve {}

/// The state of a valve that is normally closed.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum NormallyClosedValve {
    Open,
    #[default]
    Closed,
}

impl NormallyClosedValve {
    pub fn is_open(&self) -> bool {
        matches!(self, Self::Open)
    }
    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Closed)
    }
}
impl Energizable for NormallyClosedValve {
    fn is_active(&self) -> bool {
        matches!(self, Self::Open)
    }
    fn from_active(is_active: bool) -> Self {
        if is_active { Self::Open } else { Self::Closed }
    }
}
impl TryFrom<f32> for NormallyClosedValve {
    type Error = OutOfRange;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        match value {
            0.0 => Ok(NormallyClosedValve::Closed),
            1.0 => Ok(NormallyClosedValve::Open),
            _ => Err(OutOfRange),
        }
    }
}
impl From<NormallyClosedValve> for f32 {
    fn from(value: NormallyClosedValve) -> f32 {
        match value {
            NormallyClosedValve::Open => 1.0,
            NormallyClosedValve::Closed => 0.0,
        }
    }
}
impl Settable for NormallyClosedValve {}

// TODO: remove
/// Pressurization Proportional Gain in [barg/ms]
/// opening_time = kp * (target_pressure - current_pressure), in milliseconds
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(utoipa::ToSchema)]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct PressurizationGain(pub f32);

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[gated_derive(utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "f32", into = "f32")]
pub enum DPRValve {
    /// Enabled with pressure setpoint in Barg. 0-60.
    Enabled {
        /// 0-60 barg
        setpoint: f32,
    },
    /// Disabled
    #[default]
    Disabled,
}

/// Canonical mapping from f32 to [`DPRValve`]
impl TryFrom<f32> for DPRValve {
    type Error = OutOfRange;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value.is_nan() {
            Ok(DPRValve::Disabled)
        } else if (0.0..=60.0).contains(&value) {
            Ok(DPRValve::Enabled { setpoint: value })
        } else {
            Err(OutOfRange)
        }
    }
}
/// Used for csv serialization
impl From<DPRValve> for f32 {
    fn from(value: DPRValve) -> f32 {
        match value {
            DPRValve::Disabled => f32::NAN,
            DPRValve::Enabled { setpoint } => setpoint,
        }
    }
}
/// Set setpoint of DPRValve
impl Settable for DPRValve {}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CameraPowerStatus {
    /// Whether the recording camera(s) should be powered on (true=on, false=off).
    pub recording_pwr_enabled: bool,
    /// Whether the livestream camera(s) should be powered on (true=on, false=off).
    pub live_pwr_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_dpr_valve_settable() {
        // Test disabled state
        let mut valve = DPRValve::default();
        assert_eq!(valve, DPRValve::Disabled);
        assert!(f32::from(valve).is_nan());

        // Test valid setpoint
        valve.set_value(30.0).expect("is valid value");
        assert_eq!(valve, DPRValve::Enabled { setpoint: 30.0 });
        assert_eq!(f32::from(valve), 30.0);

        // Test lower bound
        valve.set_value(0.0).expect("is valid value");
        assert_eq!(valve, DPRValve::Enabled { setpoint: 0.0 });
        // Test upper bound
        valve.set_value(60.0).expect("is valid value");
        assert_eq!(valve, DPRValve::Enabled { setpoint: 60.0 });
        // Test back to disabled
        valve.set_value(f32::NAN).expect("is valid value");
        assert_eq!(valve, DPRValve::Disabled);

        // Test out of range errors
        assert!(valve.set_value(-0.1).is_err());
        assert!(valve.set_value(60.1).is_err());
    }

    mod normally_open_valve {
        use super::*;

        #[test]
        fn default_state() {
            let valve = NormallyOpenValve::default();
            assert!(valve.is_open());
            assert!(!valve.is_closed());
            assert_eq!(valve, NormallyOpenValve::Open);
        }

        #[test]
        fn set_value() {
            let mut valve = NormallyOpenValve::default();

            // Test valid values
            assert!(valve.set_value(0.0).is_ok());
            assert!(valve.is_closed());

            assert!(valve.set_value(1.0).is_ok());
            assert!(valve.is_open());

            // Test invalid values, unchanged state
            assert!(valve.set_value(0.5).is_err());
            assert!(valve.is_open());
        }

        #[test]
        fn energizable() {
            // Default is not active (normally open)
            let valve = NormallyOpenValve::default();
            assert!(!valve.is_active());

            // From active
            let active_valve = NormallyOpenValve::from_active(true);
            assert!(active_valve.is_active());
            assert!(active_valve.is_closed());

            let inactive_valve = NormallyOpenValve::from_active(false);
            assert!(!inactive_valve.is_active());
            assert!(inactive_valve.is_open());
        }
    }

    mod normally_closed_valve {
        use super::*;

        #[test]
        fn default_state() {
            let valve = NormallyClosedValve::default();
            assert!(valve.is_closed());
            assert!(!valve.is_open());
            assert_eq!(valve, NormallyClosedValve::Closed);
        }

        #[test]
        fn set_value() {
            let mut valve = NormallyClosedValve::default();

            // Test valid values
            assert!(valve.set_value(0.0).is_ok());
            assert!(valve.is_closed());

            assert!(valve.set_value(1.0).is_ok());
            assert!(valve.is_open());

            // Test invalid values, unchanged state
            assert!(valve.set_value(0.5).is_err());
            assert!(valve.is_open());
        }

        #[test]
        fn energizable() {
            // Default is not active (normally closed)
            let valve = NormallyClosedValve::default();
            assert!(!valve.is_active());

            // From active
            let active_valve = NormallyClosedValve::from_active(true);
            assert!(active_valve.is_open());
            assert!(active_valve.is_active());

            let inactive_valve = NormallyClosedValve::from_active(false);
            assert!(inactive_valve.is_closed());
            assert!(!inactive_valve.is_active());
        }
    }
}
