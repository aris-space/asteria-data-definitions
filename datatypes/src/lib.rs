use bincode::{Decode, Encode};
use mavlink_hal::codec::tbytes::TBytesWriterFor as _;
use mavlink_hal::codec::tbytes::errors::TBytesError;
use mavlink_hal::macro_util::calc_extra_crc;
use mavlink_hal::traits::{DescribeEnum, DescribeFieldSet, Enum, EnumVariant, MessageField};
use mavlink_hal::typing::{MavSimpleType, MavType};
use serde::{Deserialize, Serialize};

/// A temperature in degrees Celsius.
///
/// Is `NAN` by default.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Encode, Decode)]
pub struct Celsius(pub f32);

// maybe generated, maybe explicit
impl Celsius {
    pub const MAVLINK_FIELDS: [MessageField; 1] = [MessageField {
        type_: MavType::Scalar(MavSimpleType::Float),
        name: "temperature",
        unit: Some("°C"),
        description: "The measured temperature",
        enum_: None,
    }];
}

// generated
impl DescribeFieldSet for Celsius {
    const FIELDS: &[MessageField] = &Self::MAVLINK_FIELDS;
}

pub struct OSSTemperatures {
    /// This description and field name would be overwritten by the ones of `Celsius`.
    /// In this case not useful, but with other nested structures it should be.
    // #[mavlink(flatten)]
    pub tank: Celsius,
    /// This description and name would overide the generic one in `Celsius`,
    /// which would require exactly one field in the fieldset.
    // #[mavlink(refine)]
    pub runline: Celsius,
}

// generated
impl OSSTemperatures {
    pub const MAVLINK_FIELDS: [MessageField; Celsius::MAVLINK_FIELDS.len() + 1] =
        mavlink_hal::macro_util::concat_slices(&[
            &Celsius::MAVLINK_FIELDS,
            &[mavlink_hal::macro_util::refine_field(
                &Celsius::MAVLINK_FIELDS,
                "runline",
                "This description ...",
            )],
        ]);
}

// generated
impl DescribeFieldSet for OSSTemperatures {
    const FIELDS: &[MessageField] = &Self::MAVLINK_FIELDS;
}

/// A pressure in bar relative to atmospheric pressure (`barg`).
///
/// Is `NAN` by default.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Encode, Decode)]
pub struct BarG(pub f32);

/// The state of a valve that is normally open.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub enum NormallyOpenValve {
    #[default]
    Open = 0,
    Closed = 1,
}

// generated:
impl NormallyOpenValve {
    pub const MAVLINK_ENUM_VARIANTS: [EnumVariant; 2] = [
        EnumVariant {
            value: 0,
            name: "OPEN",
            description: "",
        },
        EnumVariant {
            value: 1,
            name: "CLOSED",
            description: "",
        },
    ];
    pub const MAVLINK_ENUM: Enum = Enum {
        name: "NORMALLY_OPEN_VALVE",
        description: "The state of a valve that is normally open. ...",
        variants: &Self::MAVLINK_ENUM_VARIANTS,
    };
}
impl DescribeEnum for NormallyOpenValve {
    const ENUM: &'static Enum = &Self::MAVLINK_ENUM;
}

/// The state of a valve that is normally closed.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub enum NormallyClosedValve {
    Open,
    #[default]
    Closed,
}

#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub struct StatusCommonMessage {
    /// Number of errors encountered since startup.
    pub errors: u32,
    /// Number of microseconds since the last board restart.
    pub micros_since_restart: u64,
}

// generated
impl StatusCommonMessage {
    pub const MAVLINK_FIELDS: [MessageField; 2] = [
        MessageField {
            type_: MavType::Scalar(MavSimpleType::UInt32),
            name: "errors",
            unit: None,
            description: "Number of errors encountered since startup.",
            enum_: None,
        },
        MessageField {
            type_: MavType::Scalar(MavSimpleType::UInt64),
            name: "micros_since_restart",
            unit: None,
            description: "Number of microseconds since the last board restart.",
            enum_: None,
        },
    ];
}

// generated
impl DescribeFieldSet for StatusCommonMessage {
    const FIELDS: &[MessageField] = &Self::MAVLINK_FIELDS;
}
impl mavlink_hal::codec::Encode for StatusCommonMessage {
    #[inline]
    fn encode_8_aligned(
        &self,
        writer: &mut mavlink_hal::codec::tbytes::TBytesWriter<
            impl mavlink_hal::codec::tbytes::TBytesWriterBackend,
        >,
    ) -> Result<usize, TBytesError> {
        let mut written = 0;
        written += writer.write(self.micros_since_restart)?;
        Ok(written)
    }
    #[inline]
    fn encode_4_aligned(
        &self,
        writer: &mut mavlink_hal::codec::tbytes::TBytesWriter<
            impl mavlink_hal::codec::tbytes::TBytesWriterBackend,
        >,
    ) -> Result<usize, TBytesError> {
        let mut written = 0;
        written += writer.write(self.errors)?;
        Ok(written)
    }
    #[inline]
    fn encode_2_aligned(
        &self,
        _writer: &mut mavlink_hal::codec::tbytes::TBytesWriter<
            impl mavlink_hal::codec::tbytes::TBytesWriterBackend,
        >,
    ) -> Result<usize, TBytesError> {
        Ok(0)
    }
    #[inline]
    fn encode_1_aligned(
        &self,
        _writer: &mut mavlink_hal::codec::tbytes::TBytesWriter<
            impl mavlink_hal::codec::tbytes::TBytesWriterBackend,
        >,
    ) -> Result<usize, TBytesError> {
        Ok(0)
    }
}

struct MessageWrapper<T> {
    id: u32,
    data: T,
}
impl mavio::protocol::IntoPayload for MessageWrapper<StatusCommonMessage> {
    fn encode(
        &self,
        version: mavio::prelude::MavLinkVersion,
    ) -> Result<mavio::protocol::Payload, mavio::prelude::SpecError> {
        if version != mavio::prelude::MavLinkVersion::V2 {
            return Err(mavio::prelude::SpecError::UnsupportedMavLinkVersion {
                actual: version,
                minimal: mavio::prelude::MavLinkVersion::V2,
            });
        }
        let mut buffer = [0_u8; mavlink_hal::macro_util::total_size(StatusCommonMessage::FIELDS)];
        let mut writer = mavlink_hal::codec::tbytes::TBytesWriter::from(buffer.as_mut_slice());
        let written = mavlink_hal::codec::encode_all(&self.data, &mut writer)
            .map_err(|_e| todo!("idk which error yet"))?;
        debug_assert_eq!(
            written,
            buffer.len(),
            "the implementation should always write exactly as many bytes as are defined by the type"
        );

        Ok(mavio::protocol::Payload::new(
            self.id,
            &buffer,
            mavio::prelude::MavLinkVersion::V2,
        ))
    }
}
impl mavio::protocol::MessageSpec for MessageWrapper<StatusCommonMessage> {
    fn id(&self) -> mavio::protocol::MessageId {
        self.id
    }
    fn min_supported_mavlink_version(&self) -> mavio::prelude::MavLinkVersion {
        mavio::prelude::MavLinkVersion::V2
    }
    fn crc_extra(&self) -> mavio::protocol::CrcExtra {
        let mut thingy = StatusCommonMessage::MAVLINK_FIELDS;
        calc_extra_crc("SOMETHING", &mut thingy)
    }
}
impl mavio::Message for MessageWrapper<StatusCommonMessage> {}

/// Shared firmware build information used by all boards.
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
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

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct CameraPower {
    /// Whether the recording camera(s) should be powered on (true=on, false=off).
    pub recording_pwr_enabled: bool,
    /// Whether the livestream camera(s) should be powered on (true=on, false=off).
    pub live_pwr_enabled: bool,
}

/// Shared rail status structure for both 5V and 24V power rails.
#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub struct RailStatus {
    /// Voltage in volts.
    pub voltage: f32,
    /// Current in amps.
    pub current: f32,
    /// Power in watts.
    pub power: f32,
}
