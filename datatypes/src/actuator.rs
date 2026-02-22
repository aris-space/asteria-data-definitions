//! Actuator control types and configurations.

#[cfg(feature = "derive_mavlink")]
use data_core::mavlink;
use gated_derive::gated_derive;

/// The state of a valve that is normally open.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema)]
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum NormallyOpenValve {
    #[default]
    Open = 0,
    Closed = 1,
}

/// The state of a valve that is normally closed.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema)]
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum NormallyClosedValve {
    Open = 0,
    #[default]
    Closed = 1,
}

/// Pressurization Proportional Gain in [barg/ms]
/// opening_time = kp * (target_pressure - current_pressure), in milliseconds
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct PressurizationGain(pub f32);

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum DprState {
    /// Enabled with pressure setpoint in Barg. 0-60.
    Enabled(f32),
    /// Disabled
    #[default]
    Disabled,
}

#[cfg(feature = "derive_mavlink")]
mod dpr_state_mavlink_impl {
    use core::f32;

    use data_core::mavlink::hal::{
        codec::{
            Decode, Encode,
            tbytes::{
                TBytesReader, TBytesReaderBackend, TBytesWriter, TBytesWriterBackend,
                errors::TBytesError,
            },
        },
        description::{DescribeFieldSet, MessageField},
        typing::Typeable,
    };

    impl DescribeFieldSet for super::DprState {
        const FIELDS: &[MessageField] = &[MessageField {
            type_: <f32 as Typeable>::TYPE,
            name: "dpr_state",
            unit: None,
            description: "",
            enum_: <f32 as Typeable>::ENUM,
        }];
    }

    impl Encode for super::DprState {
        fn encode_4_aligned(
            &self,
            writer: &mut TBytesWriter<impl TBytesWriterBackend>,
        ) -> Result<usize, TBytesError> {
            use data_core::mavlink::hal::codec::tbytes::TBytesWriterFor as _;
            match *self {
                super::DprState::Enabled(val) => writer.write(val),
                super::DprState::Disabled => writer.write(f32::NAN),
            }
        }
    }

    impl Decode for super::DprState {
        fn decode(
            _: &mut TBytesReader<impl TBytesReaderBackend>,
            reader_4_aligned: &mut TBytesReader<impl TBytesReaderBackend>,
            _: &mut TBytesReader<impl TBytesReaderBackend>,
            _: &mut TBytesReader<impl TBytesReaderBackend>,
        ) -> Result<Self, TBytesError> {
            use data_core::mavlink::hal::codec::tbytes::TBytesReaderFor as _;
            let value: f32 = reader_4_aligned.read()?;
            if value.is_nan() {
                Ok(Self::Disabled)
            } else {
                Ok(Self::Enabled(value))
            }
        }
    }
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CameraPowerStatus {
    /// Whether the recording camera(s) should be powered on (true=on, false=off).
    pub recording_pwr_enabled: bool,
    /// Whether the livestream camera(s) should be powered on (true=on, false=off).
    pub live_pwr_enabled: bool,
}
