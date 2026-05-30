//! InfluxDB [`DataPoint`] construction from typed datapoint messages.
//!
//! Used to both log as annotated CSV or directly serialize to line protocol and upload.
//!
//! # Structure
//!
//! Messages fall into two categories based on whether they carry PSS annotations.
//!
//! ## PSS-annotated
//!
//! Measurement is determined by kind and unit: `pressure_sensor`,
//! `temperature_sensor`, `valve_state`, `dpr_valve_state`, `throttable_valve_state`.
//! The sole tag is `sensor_id` (the PSS name). Fields: `value` (analog),
//! `open` (digital/DPR), `setpoint` (DPR when open), `openness` (fractional).
//!
//! ## Non-PSS structured messages
//!
//! The outer enum variant name becomes the `board` tag, the inner variant name
//! the `message` tag, and the inner type name (snake_case) the measurement.
//! All struct fields are collected as InfluxDB fields; nested structs are
//! flattened with dot-joined names (e.g. `common.errors`). Unit variants use
//! measurement `event` with a synthetic `happened = true` field.
//!
//! Both categories leave `timestamp_ns` as `None`; the caller fills it in.
//!

use std::borrow::Cow;
use std::collections::BTreeMap;

use facet::{Facet, StructKind, Variant};
use facet_format::{EnumVariantEncoding, FormatSerializer, ScalarValue, SerializeError};
use facet_reflect::Peek;

use crate::datapoint::{
    ActuatorKind, ActuatorValue, DatapointHolder, DatapointInformation, DatapointKind,
    DatapointValue, NamedDatapointValue, SensorKind, SensorValue,
};

/// A fully-resolved InfluxDB data point ready for serialisation.
// (copied from `influxdb2`)
#[derive(Debug, Clone)]
pub struct DataPoint {
    /// The measurement name (snake case)
    pub measurement: Cow<'static, str>,
    /// All the tags (snake case), all strings
    pub tags: BTreeMap<&'static str, Cow<'static, str>>,
    /// All the fields, by their name
    pub fields: BTreeMap<Cow<'static, str>, FieldValue>,
    /// Timestamp for this timeseries point
    pub timestamp_ns: Option<u64>,
}

/// Typed InfluxDB field value.
// (copied from `influxdb2`)
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    /// A boolean, which is sometimes also used for a binary enum
    Bool(bool),
    /// A integer (including integers of less width)
    I64(i64),
    /// A float (including floats of less width)
    F64(f64),
    /// Or a enum variant
    String(Cow<'static, str>),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Analog sensor unit has no measurement mapping.
    #[error("unknown sensor unit: {0}")]
    UnknownSensorUnit(String),
    /// A field type has no InfluxDB equivalent (sequences, maps, etc.).
    #[error("unsupported field type: {0}")]
    UnsupportedType(&'static str),
    /// A `[u8; N]` field is not valid UTF-8.
    #[error("field is not valid UTF-8")]
    InvalidUtf8,
    /// facet-format reported an error during serialisation.
    #[error("serialization error: {0}")]
    Serialize(String),
    /// The message did not conform to the expected two-level enum structure.
    #[error("malformed message: {0}")]
    MalformedMessage(&'static str),
    /// This extractor (intentionally) cannot handle legacy multi-datapoint messages.
    #[error("message contains too many datapoints")]
    TooManyDatapoints,
}

impl From<SerializeError<Error>> for Error {
    fn from(e: SerializeError<Error>) -> Self {
        match e {
            SerializeError::Backend(inner) => inner,
            other => Error::Serialize(other.to_string()),
        }
    }
}

/// Convert a message into a [`DataPoint`].
///
/// If `msg` contains datapoints via [`DatapointHolder`], this one is used.
/// (Although only a single one is supported).
///
/// Else, `M` must be a second level message enum (i.e. the one in `messages`).
/// Board and message names are extracted automatically from the first two nested enum variant names.
///
/// This does not yet contain a timestamp or system tag!
pub fn to_datapoint<M>(msg: &M) -> Result<DataPoint, Error>
where
    M: DatapointHolder + for<'a> Facet<'a>,
{
    let values = msg.get_datapoint_values();
    if values.is_empty() {
        // Not a named datapoint, use `facet` to gain information

        let mut ser = InfluxSerializer::default();
        facet_format::serialize_root(&mut ser, Peek::new(msg))?;
        let datap = ser.finish()?;
        Ok(datap)
    } else if values.len() == 1 {
        // Lets use the PSS datapoint and corresponding kind as the best information

        let named = values[0];
        let info = M::DATAPOINTS
            .iter()
            .find(|info| info.name == named.name)
            .expect("get_datapoint_values returned a name absent from DATAPOINTS");
        DataPoint::from_pss(named, info)
    } else {
        Err(Error::TooManyDatapoints)
    }
}

/// [`facet_format`] serializer that walks a two level enum, and then collects all fields.
///
/// Implemented as a state machine.
#[derive(Debug, Default)]
struct InfluxSerializer {
    state: SerState,
}

impl InfluxSerializer {
    /// Retuns the serialized datapoint, without a timestamp, or mission_id
    fn finish(self) -> Result<DataPoint, Error> {
        let SerState::Collecting {
            board,
            message,
            inner_type,
            mut fields,
            path,
            pending_key: None,
            ..
        } = self.state
        else {
            return Err(Error::MalformedMessage("didn't get two level enum"));
        };
        if !path.is_empty() {
            return Err(Error::MalformedMessage("still has outstanding path"));
        }
        let mut tags = BTreeMap::new();
        tags.insert("board", Cow::Borrowed(board));
        tags.insert("message", Cow::Borrowed(message));

        if fields.is_empty() {
            // fabricate a field if none existed
            let _ = fields.insert(Cow::Borrowed("happened"), FieldValue::Bool(true));
        }

        Ok(DataPoint {
            measurement: Cow::Borrowed(inner_type),
            tags,
            fields,
            timestamp_ns: None,
        })
    }
}

#[derive(Debug, Default)]
enum SerState {
    /// Initial state; awaiting the first (board-level) enum variant.
    #[default]
    Init,
    /// Got the board variant name; awaiting the inner (message-level) variant.
    GotBoard { board: &'static str },
    /// Get all message information, next enum variant will be the actual message.
    GotMessage {
        board: &'static str,
        message: &'static str,
        inner_type: &'static str,
    },
    /// Collecting payload struct fields.
    Collecting {
        board: &'static str,
        message: &'static str,
        inner_type: &'static str,
        /// Nesting path: empty at the root, one entry for each nested struct.
        path: Vec<String>,
        /// Field name from the most recent `field_key`; consumed by the next
        /// `scalar`, `begin_struct`, or `begin_enum_variant`.
        pending_key: Option<String>,
        next_seq_is_unit: bool,
        fields: BTreeMap<Cow<'static, str>, FieldValue>,
    },
}

impl FormatSerializer for InfluxSerializer {
    type Error = Error;

    /// Return `Index` so `begin_enum_variant` fires for every enum variant
    /// instead of the default `begin_struct + field_key` wrapping.
    fn enum_variant_encoding(&self) -> EnumVariantEncoding {
        EnumVariantEncoding::Index
    }

    /// Core of the state machine. The first two calls establish `board` and
    /// `message`; subsequent calls (from payload enum fields) emit the variant
    /// name as the string value of the current pending field.
    #[allow(clippy::match_wildcard_for_single_variants)]
    fn begin_enum_variant(&mut self, _index: usize, name: &'static str) -> Result<(), Error> {
        self.state = match std::mem::replace(&mut self.state, SerState::Init) {
            SerState::Init => SerState::GotBoard { board: name },
            SerState::GotMessage {
                board,
                message,
                inner_type,
            } => SerState::Collecting {
                board,
                message,
                inner_type,
                path: vec![], // root level
                pending_key: None,
                next_seq_is_unit: false,
                fields: BTreeMap::new(),
            },
            // Payload enum field: emit variant name as the string value.
            SerState::Collecting {
                board,
                message,
                inner_type,
                path,
                mut pending_key,
                next_seq_is_unit,
                mut fields,
            } => {
                let field_name = collecting_field_name(&path, &mut pending_key);
                fields.insert(field_name, FieldValue::String(Cow::Borrowed(name)));
                SerState::Collecting {
                    board,
                    message,
                    inner_type,
                    path,
                    pending_key,
                    next_seq_is_unit,
                    fields,
                }
            }
            other => other, // absorb in other states
        };
        Ok(())
    }

    /// Provides variant metadata before serializing an enum variant.
    /// Used to get the type name of the inner messsage type.
    fn variant_metadata(&mut self, variant: &'static Variant) -> Result<(), Self::Error> {
        self.state = match std::mem::replace(&mut self.state, SerState::Init) {
            SerState::GotBoard { board } => {
                if variant.data.kind == StructKind::Unit {
                    SerState::GotMessage {
                        board,
                        message: variant.effective_name(),
                        inner_type: "event",
                    }
                } else {
                    if variant.data.kind != StructKind::TupleStruct
                        || variant.data.fields.len() != 1
                    {
                        return Err(Error::MalformedMessage("not newtype enum after two levels"));
                    }
                    SerState::GotMessage {
                        board,
                        message: variant.effective_name(),
                        inner_type: variant.data.fields[0].shape().effective_name(),
                    }
                }
            }
            other => other,
        };
        Ok(())
    }

    /// Pushes the path
    #[allow(clippy::collapsible_if)] // requires newer rust version
    fn begin_struct(&mut self) -> Result<(), Error> {
        // Nested struct within the payload: push current key as path segment.
        if let SerState::Collecting {
            path, pending_key, ..
        } = &mut self.state
        {
            if let Some(key) = pending_key.take() {
                path.push(key);
            }
        }
        Ok(())
    }

    /// Pops the path
    fn end_struct(&mut self) -> Result<(), Error> {
        if let SerState::Collecting { path, .. } = &mut self.state {
            path.pop();
        }
        Ok(()) // absorb in all other states (enum wrappers, outer wrapper)
    }

    /// Stores the field name
    fn field_key(&mut self, key: &str) -> Result<(), Error> {
        if let SerState::Collecting { pending_key, .. } = &mut self.state {
            *pending_key = Some(key.to_owned());
        }
        Ok(())
    }

    /// Records a scalar field
    fn scalar(&mut self, scalar: ScalarValue<'_>) -> Result<(), Error> {
        if let SerState::Collecting {
            path,
            pending_key,
            fields,
            ..
        } = &mut self.state
        {
            let name = collecting_field_name(path, pending_key);
            if let Some(value) = scalar_to_field(scalar)? {
                fields.insert(name, value);
            }
        }
        Ok(())
    }

    /// Records a byte array field as a string
    fn serialize_byte_array(&mut self, bytes: &[u8]) -> Result<bool, Error> {
        if let SerState::Collecting {
            path,
            pending_key,
            fields,
            ..
        } = &mut self.state
        {
            let s = std::str::from_utf8(bytes).map_err(|_| Error::InvalidUtf8)?;
            let name = collecting_field_name(path, pending_key);
            fields.insert(name, FieldValue::String(Cow::Owned(s.to_owned())));
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Catch newtypes that are not transparent.
    fn struct_metadata(&mut self, shape: &facet::Shape) -> Result<(), Self::Error> {
        let facet::Type::User(facet::UserType::Struct(struct_)) = shape.ty else {
            return Ok(());
        };
        let kind = struct_.kind;

        if kind == StructKind::TupleStruct && struct_.fields.len() == 1 {
            // is indeed a newtype, so track it.
            if let SerState::Collecting {
                next_seq_is_unit, ..
            } = &mut self.state
            {
                *next_seq_is_unit = true;
            }
        }
        Ok(())
    }

    fn begin_seq(&mut self) -> Result<(), Error> {
        let SerState::Collecting {
            next_seq_is_unit, ..
        } = self.state
        else {
            return Err(Error::UnsupportedType("sequence in top level"));
        };
        if next_seq_is_unit {
            // Unit tuples get serialized as sequences, so we ignore this
            return Ok(());
        }
        Err(Error::UnsupportedType("sequence"))
    }

    fn end_seq(&mut self) -> Result<(), Error> {
        if let SerState::Collecting {
            next_seq_is_unit, ..
        } = &mut self.state
        {
            *next_seq_is_unit = false;
        }
        Ok(())
    }
}

/// Helper to convert a facet scalar to a InfluxDB scalar
fn scalar_to_field(s: ScalarValue<'_>) -> Result<Option<FieldValue>, Error> {
    Ok(Some(match s {
        ScalarValue::Unit | ScalarValue::Null => return Ok(None),
        ScalarValue::Bool(v) => FieldValue::Bool(v),
        ScalarValue::I64(v) => FieldValue::I64(v),
        ScalarValue::U64(v) => FieldValue::I64(
            v.try_into()
                .map_err(|_| Error::UnsupportedType("too large u64"))?,
        ),
        ScalarValue::F64(v) => FieldValue::F64(v),
        ScalarValue::Str(v) => FieldValue::String(Cow::Owned(v.into_owned())),
        ScalarValue::Char(c) => FieldValue::String(Cow::Owned(c.to_string())),
        ScalarValue::I128(_) | ScalarValue::U128(_) => {
            return Err(Error::UnsupportedType("i128/u128"));
        }
        ScalarValue::Bytes(_) => return Err(Error::UnsupportedType("byte sequence")),
    }))
}

impl DataPoint {
    /// Build a [`DataPoint`] from a PSS-annotated datapoint value.
    fn from_pss(
        named: NamedDatapointValue,
        info: &'static DatapointInformation,
    ) -> Result<Self, Error> {
        let mut tags = BTreeMap::new();
        tags.insert("sensor_id", Cow::Borrowed(named.name));

        let mut fields: BTreeMap<Cow<'static, str>, FieldValue> = BTreeMap::new();
        match named.value {
            DatapointValue::Sensor(SensorValue::Analog { value }) => {
                fields.insert(Cow::Borrowed("value"), FieldValue::F64(value.into()));
            }
            DatapointValue::Actuator(ActuatorValue::Digital { enabled }) => {
                fields.insert(Cow::Borrowed("open"), FieldValue::Bool(enabled));
            }
            DatapointValue::Sensor(SensorValue::Digital { enabled }) => {
                fields.insert(Cow::Borrowed("enabled"), FieldValue::Bool(enabled));
            }
            DatapointValue::Actuator(ActuatorValue::DPR { enabled, setpoint }) => {
                fields.insert(Cow::Borrowed("open"), FieldValue::Bool(enabled));
                if enabled {
                    fields.insert(Cow::Borrowed("setpoint"), FieldValue::F64(setpoint.into()));
                }
            }
            DatapointValue::Actuator(ActuatorValue::Fractional { percent }) => {
                fields.insert(Cow::Borrowed("openness"), FieldValue::F64(percent.into()));
            }
        }

        Ok(DataPoint {
            measurement: Cow::Borrowed(infer_pss_measurement(info.kind)?),
            tags,
            fields,
            timestamp_ns: None,
        })
    }
}

/// Guesses the pss measurement based on the kind (mostly works)
fn infer_pss_measurement(kind: DatapointKind) -> Result<&'static str, Error> {
    Ok(match kind {
        DatapointKind::Actuator(ActuatorKind::DPR) => "dpr_valve_state",
        DatapointKind::Actuator(ActuatorKind::Digital { .. }) => "valve_state", // FIXME: this is not very specific
        DatapointKind::Actuator(ActuatorKind::Fractional) => "throttable_valve_state",
        DatapointKind::Sensor(SensorKind::Analog { unit: "barg" }) => "pressure_sensor",
        DatapointKind::Sensor(SensorKind::Analog { unit: "°C" }) => "temperature_sensor",
        DatapointKind::Sensor(SensorKind::Analog { unit }) => {
            return Err(Error::UnknownSensorUnit(unit.to_owned()));
        }
        DatapointKind::Sensor(SensorKind::Digital { .. }) => "proximity_sensor", // FIXME: this is not very specific
    })
}

/// Build the dot-joined field name from the current path prefix and pending key.
fn collecting_field_name(path: &[String], pending_key: &mut Option<String>) -> Cow<'static, str> {
    let key = pending_key.take().unwrap_or_else(|| "value".into());
    if path.is_empty() {
        Cow::Owned(key)
    } else {
        Cow::Owned(format!("{}.{key}", path.join(".")))
    }
}
