//! Actuator control types and configurations.

use gated_derive::gated_derive;

/// A trait for an actuator, such as a valve, that can be set by a single float value.
/// The value usually is the openness for valves, in the 0-1 range.
pub trait Settable {
    fn set_value(&mut self, value: f32) -> Result<(), OutOfRange>;
}

/// Error for [`Settable`].
#[derive(Debug, Clone, Copy)]
#[gated_derive(thiserror::Error(feature = "derive_serde"))] // required for [serde(try_from = ...)]
#[error("The supplied value is out of range for this settable")]
pub struct OutOfRange;

/// Allows boxing a Watchable of a settable for a boxed trait object.
#[cfg(feature = "watchable_settable")]
impl<T: Settable> Settable for watchable::Watchable<T> {
    fn set_value(&mut self, value: f32) -> Result<(), OutOfRange> {
        self.write().set_value(value)
    }
}

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
impl Settable for NormallyOpenValve {
    fn set_value(&mut self, value: f32) -> Result<(), OutOfRange> {
        *self = match value {
            0.0 => Ok(NormallyOpenValve::Closed),
            1.0 => Ok(NormallyOpenValve::Open),
            _ => Err(OutOfRange),
        }?;
        Ok(())
    }
}

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
impl Settable for NormallyClosedValve {
    fn set_value(&mut self, value: f32) -> Result<(), OutOfRange> {
        *self = match value {
            0.0 => Ok(NormallyClosedValve::Closed),
            1.0 => Ok(NormallyClosedValve::Open),
            _ => Err(OutOfRange),
        }?;
        Ok(())
    }
}

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
impl Settable for DPRValve {
    fn set_value(&mut self, value: f32) -> Result<(), OutOfRange> {
        *self = Self::try_from(value)?;
        Ok(())
    }
}

/// The state of a throttleable valve.
/// This is a [`Settable`] with the value being 0.0 for fully closed up to 1.0 for fully open.
///
/// Note that the default behaviour is a bit weird here, as the valves don't reset on powerloss,
/// but they start closed (`Default` gives a closed state).
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Debug, Default, Clone, Copy, PartialEq)] // defaults to 0 (closed).
pub struct ThrottleableValve {
    /// The openness in percent (0 being fully closed, 100 being fully open)
    pub openness_percent: u8,
}

/// Canonical mapping from f32 to [`ThrottleableValve`]
impl TryFrom<f32> for ThrottleableValve {
    type Error = OutOfRange;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        #[allow(clippy::cast_possible_truncation)] // checked by the branch condition
        #[allow(clippy::cast_sign_loss)] // checked by the branch condition
        if (0.0..=1.0).contains(&value) {
            #[cfg(feature = "std")]
            let openness_percent = (value * 100.).round() as u8;
            #[cfg(not(feature = "std"))]
            let openness_percent = (value * 100. + 0.5) as u8;

            Ok(Self { openness_percent })
        } else {
            Err(OutOfRange)
        }
    }
}
/// Used for csv serialization
impl From<ThrottleableValve> for f32 {
    fn from(value: ThrottleableValve) -> f32 {
        f32::from(value.openness_percent) / 100.
    }
}
/// Set openness of throttleable valve
impl Settable for ThrottleableValve {
    fn set_value(&mut self, value: f32) -> Result<(), OutOfRange> {
        *self = Self::try_from(value)?;
        Ok(())
    }
}

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

    #[test]
    fn test_throttleable_valve_settable() {
        let mut valve = ThrottleableValve::default();

        // Test valid values
        valve.set_value(0.0).expect("is valid value");
        assert_eq!(valve.openness_percent, 0);
        valve.set_value(0.5).expect("is valid value");
        assert_eq!(valve.openness_percent, 50);
        valve.set_value(1.0).expect("is valid value");
        assert_eq!(valve.openness_percent, 100);

        // Test rounding
        valve.set_value(0.258).expect("is valid value");
        assert_eq!(valve.openness_percent, 26);
        valve.set_value(0.251).expect("is valid value");
        assert_eq!(valve.openness_percent, 25);
        valve.set_value(0.269).expect("is valid value");
        assert_eq!(valve.openness_percent, 27);

        // Test out of range errors
        assert!(valve.set_value(-0.1).is_err());
        assert!(valve.set_value(1.1).is_err());
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
