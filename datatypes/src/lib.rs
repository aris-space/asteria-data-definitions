use bincode::{Decode, Encode};
use gated_derive::gated_derive;

// FIXME: replace all these imports with fully qualified types
use mavlink_hal::description::{DescribeEnum, Enum, EnumVariant};
use mavlink_hal::typing::MavScalarType;

use serde::{Deserialize, Serialize};

/// A temperature in degrees Celsius.
///
/// Is `NAN` by default.
// #[derive(MavlinkFieldSet) // A newtype can be a fieldset with a single field
// #[unit("°C")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[gated_derive(bincode::Encode, bincode::Decode)]
pub struct Celsius(pub f32);

// generated: then `DescribeFieldSet` needs to be derived
impl mavlink_hal::description::DescribeFieldSet for Celsius {
    const FIELDS: &[mavlink_hal::description::MessageField] =
        &[mavlink_hal::description::MessageField {
            type_: <f32 as mavlink_hal::typing::Typeable>::TYPE, // from inner type
            name: "celsius",  // by default from the type name. Should be changeable.
            unit: Some("°C"), // from the optional `unit` attribute (note that this will also be needed by the CSV macros probably).
            description: "A temperature in degrees Celsius ...", // from doc comment
            enum_: <f32 as mavlink_hal::typing::Typeable>::ENUM, // inner type must be Typeable
        }];
}

/// This example shows how I imagine recursive fieldsets to work.
// #[derive(MavlinkFieldSet) // a struct with named fields can also be a fieldset
pub struct OSSTemperatures {
    /// This description and field name would be overwritten by the ones of `Celsius`.
    /// In this case not useful, but with other nested structures it should be.
    // #[mavlink(flatten)] // Either flatten
    pub tank: Celsius,
    /// This description and name would overide the generic one in `Celsius`,
    /// which would require exactly one field in the fieldset.
    // #[mavlink(refine)] // Or refine a newtypes documentation
    pub runline: Celsius,
}

// generated
impl mavlink_hal::description::DescribeFieldSet for OSSTemperatures {
    const FIELDS: &[mavlink_hal::description::MessageField] =
        &mavlink_hal::macro_util::concat_slices::<
            _,
            { <Celsius as mavlink_hal::description::DescribeFieldSet>::FIELDS.len() + 1 },
        >(&[
            Celsius::FIELDS,
            &[mavlink_hal::macro_util::refine_field(
                Celsius::FIELDS,
                "runline",
                "This description ...",
            )],
        ]);
}
// TODO: derive also the recursive en/decode.

/// A pressure in bar relative to atmospheric pressure (`barg`).
///
/// Is `NAN` by default.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Encode, Decode)]
pub struct BarG(pub f32);

/// The state of a valve that is normally open.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
// #[derive(MavlinkEnum) // A numeric enum (requires `= _` syntax for clarity) can be turned into a Enum.
// #[derive(MavlinkFieldSet) // and it should also be able to be a field set directly, so you can refine it
#[repr(u8)]
pub enum NormallyOpenValve {
    #[default]
    Open = 0,
    Closed = 1,
}

// generated: describes the possible variants
impl DescribeEnum for NormallyOpenValve {
    const ENUM: &'static Enum = &Enum {
        name: "NORMALLY_OPEN_VALVE",
        description: "The state of a valve that is normally open. ...",
        variants: &[
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
        ],
    };
}
// also implement it as a type, that way the other macros can use this.
impl mavlink_hal::typing::ScalarTypeable for NormallyOpenValve {
    const SCALAR_TYPE: MavScalarType = <u8 as mavlink_hal::typing::ScalarTypeable>::SCALAR_TYPE;
    const SCALAR_ENUM: Option<&'static Enum> = Some(<NormallyOpenValve as DescribeEnum>::ENUM);
}
// generated with MavlinkFieldSet
impl mavlink_hal::description::DescribeFieldSet for NormallyOpenValve {
    const FIELDS: &[mavlink_hal::description::MessageField] =
        &[mavlink_hal::description::MessageField {
            type_: <NormallyOpenValve as mavlink_hal::typing::Typeable>::TYPE,
            name: "normally_open_valve", // by default from the type name. Should be changeable.
            unit: None,                  // probably not needed for enum
            description: "The state of a valve that is normally open. ...", // from doc comment
            enum_: <NormallyOpenValve as mavlink_hal::typing::Typeable>::ENUM,
        }];
}
// generated: Encode/Decode so you can use it as a fully functional message body or refine/flatten it.
impl mavlink_hal::codec::Encode for NormallyOpenValve {
    #[inline]
    fn encode_1_aligned(
        &self,
        writer: &mut mavlink_hal::codec::tbytes::TBytesWriter<
            impl mavlink_hal::codec::tbytes::TBytesWriterBackend,
        >,
    ) -> Result<usize, mavlink_hal::codec::tbytes::errors::TBytesError> {
        use mavlink_hal::codec::tbytes::TBytesWriterFor as _;
        writer.write(*self as u8)
    }
}
impl mavlink_hal::codec::Decode for NormallyOpenValve {
    #[inline]
    fn decode(
        _reader_8_aligned: &mut mavlink_hal::codec::tbytes::TBytesReader<
            impl mavlink_hal::codec::tbytes::TBytesReaderBackend,
        >,
        _reader_4_aligned: &mut mavlink_hal::codec::tbytes::TBytesReader<
            impl mavlink_hal::codec::tbytes::TBytesReaderBackend,
        >,
        _reader_2_aligned: &mut mavlink_hal::codec::tbytes::TBytesReader<
            impl mavlink_hal::codec::tbytes::TBytesReaderBackend,
        >,
        reader_1_aligned: &mut mavlink_hal::codec::tbytes::TBytesReader<
            impl mavlink_hal::codec::tbytes::TBytesReaderBackend,
        >,
    ) -> Result<Self, mavlink_hal::codec::tbytes::errors::TBytesError> {
        use mavlink_hal::codec::tbytes::TBytesReaderFor as _;
        reader_1_aligned.read()
    }
}
// generated: "primitive" reading of enums without flatten/refine
impl<B: mavlink_hal::codec::tbytes::TBytesReaderBackend>
    mavlink_hal::codec::tbytes::TBytesReaderFor<NormallyOpenValve>
    for mavlink_hal::codec::tbytes::TBytesReader<B>
{
    #[inline]
    fn read(&self) -> Result<NormallyOpenValve, mavlink_hal::codec::tbytes::errors::TBytesError> {
        let variant_id: u8 = self.read()?;
        match variant_id {
            0 => Ok(NormallyOpenValve::Open),
            1 => Ok(NormallyOpenValve::Closed),
            _ => Err(mavlink_hal::codec::tbytes::errors::TBytesError::OutOfBounds),
        }
    }
    #[inline]
    fn read_array<const N: usize>(
        &self,
    ) -> Result<[NormallyOpenValve; N], mavlink_hal::codec::tbytes::errors::TBytesError> {
        let mut res = [NormallyOpenValve::Closed; N]; // any value, will be overridden.
        for item in &mut res {
            *item = self.read()?;
        }
        Ok(res)
    }
}
// generated: "primitive" writing of enums without flatten/refine
impl<B: mavlink_hal::codec::tbytes::TBytesWriterBackend>
    mavlink_hal::codec::tbytes::TBytesWriterFor<NormallyOpenValve>
    for mavlink_hal::codec::tbytes::TBytesWriter<B>
where
    NormallyOpenValve: Copy, // required for functionality
{
    #[inline]
    fn write(
        &mut self,
        value: NormallyOpenValve,
    ) -> Result<usize, mavlink_hal::codec::tbytes::errors::TBytesError> {
        self.write(value as u8)
    }
    #[inline]
    fn write_slice(
        &mut self,
        values: &[NormallyOpenValve],
    ) -> Result<usize, mavlink_hal::codec::tbytes::errors::TBytesError> {
        let mut written = 0;
        for value in values {
            written += self.write(*value)?;
        }
        Ok(written)
    }
}

/// The state of a valve that is normally closed.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub enum NormallyClosedValve {
    Open,
    #[default]
    Closed,
}

#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
// #[derive(MavlinkFieldSet)
pub struct StatusCommonMessage {
    /// Number of errors encountered since startup.
    pub errors: u32,
    /// Number of microseconds since the last board restart.
    pub micros_since_restart: u64,
}

// generated: track the fields, used for `DescribeFieldSet` trait and ultimately XMl generation.
impl mavlink_hal::description::DescribeFieldSet for StatusCommonMessage {
    const FIELDS: &[mavlink_hal::description::MessageField] = &[
        mavlink_hal::description::MessageField {
            type_: <u32 as mavlink_hal::typing::Typeable>::TYPE,
            name: "errors",
            unit: None,
            description: "Number of errors encountered since startup.",
            enum_: <u32 as mavlink_hal::typing::Typeable>::ENUM,
        },
        mavlink_hal::description::MessageField {
            type_: <u64 as mavlink_hal::typing::Typeable>::TYPE,
            name: "micros_since_restart",
            unit: None,
            description: "Number of microseconds since the last board restart.",
            enum_: <u64 as mavlink_hal::typing::Typeable>::ENUM,
        },
    ];
}
// generated: provide a alignment reordering compliant encode implementation
impl mavlink_hal::codec::Encode for StatusCommonMessage {
    #[inline]
    fn encode_8_aligned(
        &self,
        writer: &mut mavlink_hal::codec::tbytes::TBytesWriter<
            impl mavlink_hal::codec::tbytes::TBytesWriterBackend,
        >,
    ) -> Result<usize, mavlink_hal::codec::tbytes::errors::TBytesError> {
        use mavlink_hal::codec::tbytes::TBytesWriterFor as _;
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
    ) -> Result<usize, mavlink_hal::codec::tbytes::errors::TBytesError> {
        use mavlink_hal::codec::tbytes::TBytesWriterFor as _;
        let mut written = 0;
        written += writer.write(self.errors)?;
        Ok(written)
    }
}
// generated: provide alignment-reordering compliant recursive trait implementation
impl mavlink_hal::codec::Decode for StatusCommonMessage {
    #[inline]
    fn decode(
        reader_8_aligned: &mut mavlink_hal::codec::tbytes::TBytesReader<
            impl mavlink_hal::codec::tbytes::TBytesReaderBackend,
        >,
        reader_4_aligned: &mut mavlink_hal::codec::tbytes::TBytesReader<
            impl mavlink_hal::codec::tbytes::TBytesReaderBackend,
        >,
        _reader_2_aligned: &mut mavlink_hal::codec::tbytes::TBytesReader<
            impl mavlink_hal::codec::tbytes::TBytesReaderBackend,
        >,
        _reader_1_aligned: &mut mavlink_hal::codec::tbytes::TBytesReader<
            impl mavlink_hal::codec::tbytes::TBytesReaderBackend,
        >,
    ) -> Result<Self, mavlink_hal::codec::tbytes::errors::TBytesError> {
        use mavlink_hal::codec::tbytes::TBytesReaderFor as _;
        Ok(Self {
            errors: reader_4_aligned.read()?,
            micros_since_restart: reader_8_aligned.read()?,
        })
    }
}

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
