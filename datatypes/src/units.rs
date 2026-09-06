//! Physical measurement units and related types.

use core::fmt::Display;
#[cfg(feature = "derive_mavlink")]
use data_core::mavlink;
use gated_derive::gated_derive;

/// Macro to generate unit newtype structs with standard derives and Default implementation.
///
/// # Usage
/// ```ignore
/// unit_newtypes! {
///     /// Documentation for the first unit type.
///     FirstType(f32),
///
///     /// Documentation for the second unit type.
///     SecondType(f64),
/// }
/// ```
macro_rules! unit_newtypes {
    (
        $(
            $(#[$meta:meta])*
            $name:ident($ty:ty as $unit:literal)
        ),* $(,)?
    ) => {
        $(
            #[gated_derive(serde::Serialize, defmt::Format)]
            #[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
            #[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
            #[gated_derive(utoipa::ToSchema, facet::Facet)]
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
            #[repr(transparent)]
            #[serde(rename_all = "camelCase")]
            #[unit($unit)]
            // Meta attributes need to come after all gated derives in order to ensure
            // attributes are gated properly
            $(#[$meta])*
            ///
            /// Is `NAN` by default.
            pub struct $name(pub $ty);

            impl Default for $name {
                fn default() -> Self {
                    Self(<$ty>::NAN)
                }
            }

            impl Display for $name {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{}{}", self.0, Self::UNIT)
                }
            }

            impl ValueWithUnit<$ty> for $name {
                const UNIT: &'static str = $unit;

                fn value(&self) -> $ty {
                    self.0
                }
            }

            #[cfg(feature = "derive_serde")]
            impl<'de> serde::Deserialize<'de> for $name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    let x = <Option<f32> as serde::Deserialize>::deserialize(deserializer)?;
                    Ok($name(x.unwrap_or(f32::NAN)))
                }
            }
        )*
    };
}

/// Trait for generalizing values like [`Celsius`], [`BarG`], etc. such
/// that their inner value can be retrieved.
pub trait ValueWithUnit<T: Display>: Display {
    /// Unit of the value.
    const UNIT: &'static str;

    /// Current value.
    fn value(&self) -> T;
}

unit_newtypes! {
    /// A temperature in degrees Celsius.
    Celsius(f32 as "°C"),

    /// A pressure in bar relative to atmospheric pressure (`barg`).
    BarG(f32 as "barg"),

    /// Atmospheric pressure in hPa.
    HPa(f32 as "hPa"),
}

/// Shared rail status structure for both 5V and 24V power rails.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RailStatus {
    /// Voltage in volts.
    pub voltage: f32,
    /// Current in amps.
    pub current: f32,
    /// Power in watts.
    pub power: f32,
}

/// UTC timestamp given in microseconds since the Unix epoch (`1970-01-01T00:00:00Z`).
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[serde(rename_all = "camelCase")]
#[unit("us")]
pub struct InstantUs(pub u64);
