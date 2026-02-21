//! System status, state information, and identifiers.

use gated_derive::gated_derive;

/// The ID of a board in the rocket.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(arbitrary::Arbitrary)]
#[gated_derive(utoipa::ToSchema)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub enum BoardId {
    FlightComputer = 0,
    RfsComputer = 1,
    SensorCarrier = 2,
    CommunicationBoard = 3,
    EngineControlBoard = 4,
    PowerBoard = 5,
    RecoveryBoard = 6,
    FuelControlBoard = 7,
    OxidizerControlBoard = 8,
}

/// UTC timestamp given in microseconds since the Unix epoch (`1970-01-01T00:00:00Z`).
///
/// This value is derived from the GPS time sensors and is therefore highly accurate.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct InstantUs(pub u64);

/// Represents the current arming state of the subsystem
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum ArmingState {
    /// System is safe to approach, arming action is necessary to use the system.
    /// This can mean for example that actuators do not have power.
    #[default]
    Safe,
    /// System is not safe to approach anymore. System can be used to its full capacity
    /// This can mean for example that previously unpowered actuators now have power.
    Armed,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(utoipa::ToSchema)]
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
#[gated_derive(utoipa::ToSchema)]
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

/// Enum representing the status of a sensor.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(arbitrary::Arbitrary)]
#[gated_derive(utoipa::ToSchema)]
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum SensorStatus {
    /// Sensor is not providing data or is not responding.
    #[default]
    Offline = 0,
    /// Sensor is generating data and responding.
    Online = 1,
}

/// Enum representing the state of a valve.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(arbitrary::Arbitrary)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum ValveState {
    /// Power should be enabled for the valve
    Active,
    /// Power should not be disabled for the valve
    #[default]
    Inactive,
}

#[cfg(feature = "derive_bincode")]
/// We implement these traits manually, otherwise bincode uses a u32 for the enum ¯\_(ツ)_/¯
mod encoding_impls {
    use crate::status::{BoardId, SensorStatus, ValveState};

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
                5u8 => Ok(Self::PowerBoard {}),
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
                5u8 => Ok(Self::PowerBoard {}),
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

    impl ::bincode::Encode for ValveState {
        fn encode<__E: ::bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), ::bincode::error::EncodeError> {
            match self {
                Self::Active => {
                    <u8 as ::bincode::Encode>::encode(&(0u8), encoder)?;
                    core::result::Result::Ok(())
                }
                Self::Inactive => {
                    <u8 as ::bincode::Encode>::encode(&(1u8), encoder)?;
                    core::result::Result::Ok(())
                }
            }
        }
    }
    impl<__Context> ::bincode::Decode<__Context> for ValveState {
        fn decode<__D: ::bincode::de::Decoder<Context = __Context>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, ::bincode::error::DecodeError> {
            let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
            match variant_index {
                0u8 => core::result::Result::Ok(Self::Active {}),
                1u8 => core::result::Result::Ok(Self::Inactive {}),
                variant => {
                    core::result::Result::Err(::bincode::error::DecodeError::UnexpectedVariant {
                        found: u32::from(variant),
                        type_name: "ValveState",
                        allowed: &::bincode::error::AllowedEnumVariants::Range { min: 0, max: 1 },
                    })
                }
            }
        }
    }
    impl<'__de, __Context> ::bincode::BorrowDecode<'__de, __Context> for ValveState {
        fn borrow_decode<__D: ::bincode::de::BorrowDecoder<'__de, Context = __Context>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, ::bincode::error::DecodeError> {
            let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
            match variant_index {
                0u8 => core::result::Result::Ok(Self::Active {}),
                1u8 => core::result::Result::Ok(Self::Inactive {}),
                variant => {
                    core::result::Result::Err(::bincode::error::DecodeError::UnexpectedVariant {
                        found: u32::from(variant),
                        type_name: "ValveState",
                        allowed: &::bincode::error::AllowedEnumVariants::Range { min: 0, max: 1 },
                    })
                }
            }
        }
    }
}
