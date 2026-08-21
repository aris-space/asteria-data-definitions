//! Physical measurement units and related types.

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
            $name:ident($ty:ty)
        ),* $(,)?
    ) => {
        $(
            ///
            /// Is `NAN` by default.
            #[gated_derive(serde::Serialize, defmt::Format)]
            #[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
            #[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
            #[gated_derive(utoipa::ToSchema)]
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
            #[repr(transparent)]
            #[serde(rename_all = "camelCase")]
            // Meta attributes need to come after all gated derives in order to ensure
            // attributes are gated properly
            $(#[$meta])*
            pub struct $name(pub $ty);

            impl Default for $name {
                fn default() -> Self {
                    Self(<$ty>::NAN)
                }
            }

            impl ValueWithUnit<$ty> for $name {
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
pub trait ValueWithUnit<T> {
    /// Current value.
    fn value(&self) -> T;
}

unit_newtypes! {
    /// A temperature in degrees Celsius.
    #[unit("°C")]
    Celsius(f32),

    /// A pressure in bar relative to atmospheric pressure (`barg`).
    #[unit("barg")]
    BarG(f32),

    /// Atmospheric pressure in hPa.
    #[unit("hPa")]
    HPa(f32),
}

/// Shared rail status structure for both 5V and 24V power rails.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
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
#[gated_derive(utoipa::ToSchema)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[serde(rename_all = "camelCase")]
#[unit("us")]
pub struct InstantUs(pub u64);
