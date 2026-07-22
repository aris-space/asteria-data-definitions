//! System status, state information, and identifiers.

#[cfg(feature = "derive_mavlink")]
use data_core::mavlink;
use gated_derive::gated_derive;

use crate::{actuator::Energizable, units::InstantUs};

/// The ID of a board in the rocket.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub enum BoardId {
    FlightComputer = 0,
    RfsComputer = 1,
    SensorCarrier = 2,
    CommunicationBoard = 3,
    EngineControlBoard = 4,
    Backplane = 5,
    RecoveryBoard = 6,
    FuelControlBoard = 7,
    OxidizerControlBoard = 8,
}

/// Represents the current arming state of the subsystem
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum ArmingState {
    /// System is safe to approach, arming action is necessary to use the system.
    /// This can mean for example that actuators do not have power.
    #[default]
    Safe = 0,
    /// System is not safe to approach anymore. System can be used to its full capacity
    /// This can mean for example that previously unpowered actuators now have power.
    Armed = 1,
}

impl Energizable for ArmingState {
    fn is_active(&self) -> bool {
        matches!(self, Self::Armed)
    }
    fn from_active(is_active: bool) -> Self {
        if is_active { Self::Armed } else { Self::Safe }
    }
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusCommonMessage {
    /// Number of errors encountered since startup.
    pub errors: u32,
    /// Number of microseconds since the last board restart.
    pub micros_since_restart: u64,
}

/// Shared firmware build information used by all boards.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[derive(Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BuildInformationCommon {
    /// Unix timestamp when the build was compiled, provided in seconds since
    /// the Unix epoch `(1970-01-01 00:00:00 UTC)`. While this is a 32-bit integer,
    /// overflow is not a concern as we don't intend to use this code up to 2038.
    pub unix_timestamp: u32,
    /// Initials of the person who compiled the firmware, or `\0\0` if not set.
    pub author_initials: [u8; 2],
    /// Whether the build is a release build.
    pub is_release: bool,
    /// Whether the build includes debugging defmt over RTT.
    pub debug_defmt_rtt: bool,
    /// Firmware version information.
    pub commit_hash: [u8; 7],
    /// Indicates if the git repository was dirty at build time.
    pub is_git_dirty: bool,
    /// CAN library version information (major, minor, patch).
    pub can_semver: [u8; 3],
}

/// First 5 hex characters of the config file Git Object Hash
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigHash {
    pub hash: [u8; 5],
}

/// Shared state machine state used by the FC and RFSC.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StateMachineState {
    /// The index in the state path where we are currently at.
    /// This equals the number of transitions that have occured.
    pub path_index: u32,
    /// The name of the current state in zero-padded utf-8.
    pub state: [u8; 30],
}

/// Shared manual state transition used by the FC and RFSC.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ManualStateTransition {
    /// The index in the state path where the new state should end up at.
    pub transition_index: u32,
    /// The name of the state to which the state machine should transition.
    pub state: [u8; 30],
}

/// Shared manual actuator change used by the FC and RFSC.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ManualActuatorChange {
    /// The target value of the actuator.
    /// This depends on the actuator type, for valves 0.0 is fully closed
    /// and 1.0 is fully open.
    #[serde(deserialize_with = "f32_nan_deser")]
    pub value: f32,
    /// The index in the state path where the current state is.
    /// For the initial state this is 0 and increments with every transition.
    pub path_index: u32,
    /// The name of the actuator whose value should be changed.
    pub actuator: [u8; 20],
}

#[cfg(feature = "derive_serde")]
fn f32_nan_deser<'de, D>(de: D) -> Result<f32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let x = <Option<f32> as serde::Deserialize>::deserialize(de)?;
    Ok(x.unwrap_or(f32::NAN))
}

/// Telemetry health statistics for an uplink.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UplinkTelemetryHealth {
    /// The timestamp of the last message received, including malformed ones.
    #[mavlink(refine)]
    pub time_of_last_msg: InstantUs,
    /// The total number of the messages received, including malformed ones.
    pub num_msg_received: u32,
    /// The number of message that were inferred to have been lost by the sequence number.
    pub num_msg_lost: u32,

    /// The timestamp of the last message that has been understood and processed successfully.
    #[mavlink(refine)]
    pub time_of_last_processed_msg: InstantUs,
    /// The number of the messages received that have been understood and processed successfully.
    pub num_msg_processed: u32,
}

/// Telemetry statistics for a downlink.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DownlinkTelemetryStats {
    /// The total number of bytes sent.
    pub num_bytes_sent: u64,
    /// The total number of messages sent.
    pub num_messages_sent: u32,
}

/// Enum representing the status of a sensor.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum SensorStatus {
    /// Sensor is not providing data or is not responding.
    #[default]
    Offline = 0,
    /// Sensor is generating data and responding.
    Online = 1,
}

/// Enum representing the active power source.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum ActivePowerSource {
    /// Battery power is currently active.
    Battery = 0,
    /// External power is currently active.
    External = 1,
    /// Currently no 24V available.
    #[default]
    None = 2,
}

/// Enum representing the state of a dpr loop.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum DprLoopInfo {
    /// The DPR Loop is Active under Nominal Operations
    ActiveNominal = 0,
    /// The DPR Loop is Active but disabled due to Overpressurization
    ActiveOverPressure = 1,
    /// The DPR Loop is Passive, the corresponding valve is in a constant `Disabled` state
    #[default]
    Passive = 2,
}
/// Struct representing the gains of the dpr controller
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DprGainInfo {
    /// The DPR proportional gain
    pub p: f32,
    /// The DPR integral gain
    pub i: f32,
    /// The DPR derivative gain
    pub d: f32,
    /// The DPR min opening time in ms
    pub min_ms: f32,
    /// The DPR max opening time in ms
    pub max_ms: f32,
}

#[cfg(feature = "derive_bincode")]
/// We implement these traits manually, otherwise bincode uses a u32 for the enum ¯\_(ツ)_/¯
mod encoding_impls {
    use crate::status::{ActivePowerSource, BoardId, DprLoopInfo, SensorStatus};

    impl ::bincode::Encode for BoardId {
        fn encode<__E: ::bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> Result<(), ::bincode::error::EncodeError> {
            <u8 as ::bincode::Encode>::encode(&(*self as u8), encoder)?;
            Ok(())
        }
    }
    impl<__Context> ::bincode::Decode<__Context> for BoardId {
        fn decode<__D: ::bincode::de::Decoder<Context = __Context>>(
            decoder: &mut __D,
        ) -> Result<Self, ::bincode::error::DecodeError> {
            let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
            match variant_index {
                0u8 => Ok(Self::FlightComputer {}),
                1u8 => Ok(Self::RfsComputer {}),
                2u8 => Ok(Self::SensorCarrier {}),
                3u8 => Ok(Self::CommunicationBoard {}),
                4u8 => Ok(Self::EngineControlBoard {}),
                5u8 => Ok(Self::Backplane {}),
                6u8 => Ok(Self::RecoveryBoard {}),
                7u8 => Ok(Self::FuelControlBoard {}),
                8u8 => Ok(Self::OxidizerControlBoard {}),
                variant => Err(::bincode::error::DecodeError::UnexpectedVariant {
                    found: u32::from(variant),
                    type_name: "BoardId",
                    allowed: &::bincode::error::AllowedEnumVariants::Allowed(&[
                        0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32,
                    ]),
                }),
            }
        }
    }
    impl<'__de, __Context> ::bincode::BorrowDecode<'__de, __Context> for BoardId {
        fn borrow_decode<__D: ::bincode::de::BorrowDecoder<'__de, Context = __Context>>(
            decoder: &mut __D,
        ) -> Result<Self, ::bincode::error::DecodeError> {
            let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
            match variant_index {
                0u8 => Ok(Self::FlightComputer {}),
                1u8 => Ok(Self::RfsComputer {}),
                2u8 => Ok(Self::SensorCarrier {}),
                3u8 => Ok(Self::CommunicationBoard {}),
                4u8 => Ok(Self::EngineControlBoard {}),
                5u8 => Ok(Self::Backplane {}),
                6u8 => Ok(Self::RecoveryBoard {}),
                7u8 => Ok(Self::FuelControlBoard {}),
                8u8 => Ok(Self::OxidizerControlBoard {}),
                variant => Err(::bincode::error::DecodeError::UnexpectedVariant {
                    found: u32::from(variant),
                    type_name: "BoardId",
                    allowed: &::bincode::error::AllowedEnumVariants::Allowed(&[
                        0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32,
                    ]),
                }),
            }
        }
    }

    impl ::bincode::Encode for SensorStatus {
        fn encode<__E: ::bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> Result<(), ::bincode::error::EncodeError> {
            match self {
                Self::Offline => {
                    <u8 as ::bincode::Encode>::encode(&(0u8), encoder)?;
                    Ok(())
                }
                Self::Online => {
                    <u8 as ::bincode::Encode>::encode(&(1u8), encoder)?;
                    Ok(())
                }
            }
        }
    }
    impl<__Context> ::bincode::Decode<__Context> for SensorStatus {
        fn decode<__D: ::bincode::de::Decoder<Context = __Context>>(
            decoder: &mut __D,
        ) -> Result<Self, ::bincode::error::DecodeError> {
            let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
            match variant_index {
                0u8 => Ok(Self::Offline {}),
                1u8 => Ok(Self::Online {}),
                variant => Err(::bincode::error::DecodeError::UnexpectedVariant {
                    found: u32::from(variant),
                    type_name: "SensorStatus",
                    allowed: &::bincode::error::AllowedEnumVariants::Range { min: 0, max: 1 },
                }),
            }
        }
    }
    impl<'__de, __Context> ::bincode::BorrowDecode<'__de, __Context> for SensorStatus {
        fn borrow_decode<__D: ::bincode::de::BorrowDecoder<'__de, Context = __Context>>(
            decoder: &mut __D,
        ) -> Result<Self, ::bincode::error::DecodeError> {
            let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
            match variant_index {
                0u8 => Ok(Self::Offline {}),
                1u8 => Ok(Self::Online {}),
                variant => Err(::bincode::error::DecodeError::UnexpectedVariant {
                    found: u32::from(variant),
                    type_name: "SensorStatus",
                    allowed: &::bincode::error::AllowedEnumVariants::Range { min: 0, max: 1 },
                }),
            }
        }
    }

    impl ::bincode::Encode for ActivePowerSource {
        fn encode<__E: ::bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), ::bincode::error::EncodeError> {
            match self {
                Self::Battery => {
                    <u8 as ::bincode::Encode>::encode(&(0u8), encoder)?;
                    core::result::Result::Ok(())
                }
                Self::External => {
                    <u8 as ::bincode::Encode>::encode(&(1u8), encoder)?;
                    core::result::Result::Ok(())
                }
                Self::None => {
                    <u8 as ::bincode::Encode>::encode(&(2u8), encoder)?;
                    core::result::Result::Ok(())
                }
            }
        }
    }
    impl<__Context> ::bincode::Decode<__Context> for ActivePowerSource {
        fn decode<__D: ::bincode::de::Decoder<Context = __Context>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, ::bincode::error::DecodeError> {
            let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
            match variant_index {
                0u8 => core::result::Result::Ok(Self::Battery {}),
                1u8 => core::result::Result::Ok(Self::External {}),
                2u8 => core::result::Result::Ok(Self::None {}),
                variant => {
                    core::result::Result::Err(::bincode::error::DecodeError::UnexpectedVariant {
                        found: u32::from(variant),
                        type_name: "ActivePowerSource",
                        allowed: &::bincode::error::AllowedEnumVariants::Range { min: 0, max: 2 },
                    })
                }
            }
        }
    }
    impl<'__de, __Context> ::bincode::BorrowDecode<'__de, __Context> for ActivePowerSource {
        fn borrow_decode<__D: ::bincode::de::BorrowDecoder<'__de, Context = __Context>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, ::bincode::error::DecodeError> {
            let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
            match variant_index {
                0u8 => core::result::Result::Ok(Self::Battery {}),
                1u8 => core::result::Result::Ok(Self::External {}),
                2u8 => core::result::Result::Ok(Self::None {}),
                variant => {
                    core::result::Result::Err(::bincode::error::DecodeError::UnexpectedVariant {
                        found: u32::from(variant),
                        type_name: "ActivePowerSource",
                        allowed: &::bincode::error::AllowedEnumVariants::Range { min: 0, max: 2 },
                    })
                }
            }
        }
    }

    impl ::bincode::Encode for DprLoopInfo {
        fn encode<__E: ::bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), ::bincode::error::EncodeError> {
            match self {
                Self::ActiveNominal => {
                    <u8 as ::bincode::Encode>::encode(&(0u8), encoder)?;
                    core::result::Result::Ok(())
                }
                Self::ActiveOverPressure => {
                    <u8 as ::bincode::Encode>::encode(&(1u8), encoder)?;
                    core::result::Result::Ok(())
                }
                Self::Passive => {
                    <u8 as ::bincode::Encode>::encode(&(2u8), encoder)?;
                    core::result::Result::Ok(())
                }
            }
        }
    }
    impl<__Context> ::bincode::Decode<__Context> for DprLoopInfo {
        fn decode<__D: ::bincode::de::Decoder<Context = __Context>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, ::bincode::error::DecodeError> {
            let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
            match variant_index {
                0u8 => core::result::Result::Ok(Self::ActiveNominal {}),
                1u8 => core::result::Result::Ok(Self::ActiveOverPressure {}),
                2u8 => core::result::Result::Ok(Self::Passive {}),
                variant => {
                    core::result::Result::Err(::bincode::error::DecodeError::UnexpectedVariant {
                        found: u32::from(variant),
                        type_name: "DprLoopInfo",
                        allowed: &::bincode::error::AllowedEnumVariants::Allowed(&[
                            0u32, 1u32, 2u32,
                        ]),
                    })
                }
            }
        }
    }
    impl<'__de, __Context> ::bincode::BorrowDecode<'__de, __Context> for DprLoopInfo {
        fn borrow_decode<__D: ::bincode::de::BorrowDecoder<'__de, Context = __Context>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, ::bincode::error::DecodeError> {
            let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
            match variant_index {
                0u8 => core::result::Result::Ok(Self::ActiveNominal {}),
                1u8 => core::result::Result::Ok(Self::ActiveOverPressure {}),
                2u8 => core::result::Result::Ok(Self::Passive {}),
                variant => {
                    core::result::Result::Err(::bincode::error::DecodeError::UnexpectedVariant {
                        found: u32::from(variant),
                        type_name: "DprLoopInfo",
                        allowed: &::bincode::error::AllowedEnumVariants::Allowed(&[
                            0u32, 1u32, 2u32,
                        ]),
                    })
                }
            }
        }
    }
}
