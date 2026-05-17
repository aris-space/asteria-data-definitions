//! InfluxDB [`DataPoint`] construction from typed datapoint messages.
//!
//! # Semantics
//!
//! - If the message contains non-empty `get_datapoint_values()`,
//!   then one [`DataPoint`] per PSS field via the name and kind there.
//! - Else, the two enum variants become tags, the inner type the measurement,
//!   and the fields become fields
//!   (FIXME: how to handle enums/unit messages)
//!

use std::borrow::Cow;
use std::collections::BTreeMap;

use facet::Facet;
use facet_format::{EnumVariantEncoding, FormatSerializer, ScalarValue, SerializeError};
use facet_reflect::Peek;

use crate::datapoint::{
    ActuatorKind, ActuatorValue, DatapointHolder, DatapointInformation, DatapointKind,
    DatapointValue, NamedDatapointValue, SensorKind, SensorValue,
};

/// A fully-resolved InfluxDB data point ready for serialisation.
// (copied from `influxdb2`)
// TODO: maybe we can delay allocating for snake case except for the first occurance (for header).
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
    /// A integer (including integers of less width)
    U64(u64),
    /// A float (including floats of less width)
    F64(f64),
    /// Or a enum variant
    String(Cow<'static, str>),
}

#[derive(Debug)]
pub enum Error {
    /// Analog sensor unit has no measurement mapping.
    UnknownSensorUnit(String),
    /// A field type has no InfluxDB equivalent (sequences, maps, etc.).
    UnsupportedType(&'static str),
    /// A `[u8; N]` field is not valid UTF-8.
    InvalidUtf8,
    /// facet-format reported an error during serialisation.
    Serialize(String),
    /// The struct produced no fields (all-unit or empty).
    NoFields,
    /// The message did not conform to the expected two-level enum structure.
    MalformedMessage,
    /// This extractor (intentionally) cannot handle legacy multi-datapoint messages.
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
pub fn to_datapoint<M>(msg: &M, timestamp_ns: u64) -> Result<DataPoint, Error>
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
    // TODO set timestamp and mission id
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
        let SerState::Done {
            board,
            message,
            fields,
        } = self.state
        else {
            return Err(Error::MalformedMessage);
        };
        if fields.is_empty() {
            return Err(Error::NoFields);
        }
        let mut tags = BTreeMap::new();
        tags.insert("board", Cow::Owned(board));
        tags.insert("message", Cow::Owned(message.clone()));
        Ok(DataPoint {
            measurement: Cow::Owned(to_snake_case(&message)), // FIXME: this should be inner's type
            tags,
            fields,
            timestamp_ns: None,
        })
    }
}

// FIXME: waaaay to many allocated strings, most of them are always (or at least sometimes) &'static

#[derive(Debug, Default)]
enum SerState {
    /// Initial state; awaiting the first (board-level) enum variant.
    #[default]
    Init,
    /// Got the board variant name; awaiting the inner (message-level) variant.
    GotBoard { board: String },
    // FIXME: remove probably -> innermost type can also be a primitive (or more common, a enum)
    /// Got both names; the next `begin_struct` starts field collection.
    GotMessage { board: String, message: String },
    /// Collecting payload struct fields.
    Collecting {
        board: String,
        message: String,
        /// Nesting path: `None` at the root, `Some(name)` for each nested struct.
        // FIXME: should this just be Vec<&'static str>, with empty being the root?
        path: Vec<Option<String>>,
        /// Field name from the most recent `field_key`; consumed by the next
        /// `scalar` or `begin_struct`.
        pending_key: Option<String>,
        fields: BTreeMap<Cow<'static, str>, FieldValue>,
    },
    /// Collection complete; trailing `end_struct` calls are absorbed.
    Done {
        board: String,
        message: String,
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
    fn begin_enum_variant(&mut self, _index: usize, name: &'static str) -> Result<(), Error> {
        self.state = match std::mem::replace(&mut self.state, SerState::Init) {
            SerState::Init => SerState::GotBoard {
                board: name.to_owned(),
            },
            SerState::GotBoard { board } => SerState::GotMessage {
                board,
                message: name.to_owned(),
            },
            // Payload enum field: emit variant name as the string value.
            SerState::Collecting {
                board,
                message,
                path,
                mut pending_key,
                mut fields,
            } => {
                let field_name = collecting_field_name(&path, &mut pending_key);
                fields.insert(field_name, FieldValue::String(Cow::Borrowed(name)));
                SerState::Collecting {
                    board,
                    message,
                    path,
                    pending_key,
                    fields,
                }
            }
            other => other, // absorb in Done or unexpected states
        };
        Ok(())
    }

    fn begin_struct(&mut self) -> Result<(), Error> {
        self.state = match std::mem::replace(&mut self.state, SerState::Init) {
            // Payload struct begins: initialise collecting state.
            SerState::GotMessage { board, message } => SerState::Collecting {
                board,
                message,
                path: vec![None], // root level
                pending_key: None,
                fields: BTreeMap::new(),
            },
            // Nested struct within the payload: push current key as path segment.
            SerState::Collecting {
                board,
                message,
                mut path,
                mut pending_key,
                fields,
            } => {
                path.push(pending_key.take());
                SerState::Collecting {
                    board,
                    message,
                    path,
                    pending_key,
                    fields,
                }
            }
            other => other, // absorb
        };
        Ok(())
    }

    fn end_struct(&mut self) -> Result<(), Error> {
        // Phase 1: pop path (active borrow ends here).
        let complete = if let SerState::Collecting { path, .. } = &mut self.state {
            path.pop();
            path.is_empty()
        } else {
            false
        };
        // Phase 2: if the root struct closed, consume state (no active borrow).
        if complete {
            match std::mem::replace(&mut self.state, SerState::Init) {
                SerState::Collecting {
                    board,
                    message,
                    fields,
                    ..
                } => {
                    self.state = SerState::Done {
                        board,
                        message,
                        fields,
                    };
                }
                _ => unreachable!(),
            }
        }
        Ok(()) // absorb in all other states (enum wrappers, outer wrapper)
    }

    fn field_key(&mut self, key: &str) -> Result<(), Error> {
        if let SerState::Collecting { pending_key, .. } = &mut self.state {
            *pending_key = Some(key.to_owned());
        }
        Ok(())
    }

    fn scalar(&mut self, scalar: ScalarValue<'_>) -> Result<(), Error> {
        match &mut self.state {
            SerState::Collecting {
                path,
                pending_key,
                fields,
                ..
            } => {
                let name = collecting_field_name(path, pending_key);
                if let Some(value) = scalar_to_field(scalar)? {
                    fields.insert(name, value);
                }
            }
            // Simple scalar payload at the message level (e.g. a Cat A PSS
            // value that serialises as a bare f32). We don't collect it — Cat A
            // uses get_datapoint_values() — but we need to reach Done.
            SerState::GotMessage { .. } => {
                match std::mem::replace(&mut self.state, SerState::Init) {
                    SerState::GotMessage { board, message } => {
                        self.state = SerState::Done {
                            board,
                            message,
                            fields: BTreeMap::new(),
                        };
                    }
                    _ => unreachable!(),
                }
            }
            _ => {} // absorb
        }
        Ok(())
    }

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

    fn begin_seq(&mut self) -> Result<(), Error> {
        Err(Error::UnsupportedType("sequence"))
    }

    fn end_seq(&mut self) -> Result<(), Error> {
        unreachable!("begin_seq always errors")
    }
}

/// Helper to convert a facet scalar to a InfluxDB scalar
fn scalar_to_field(s: ScalarValue<'_>) -> Result<Option<FieldValue>, Error> {
    Ok(Some(match s {
        ScalarValue::Unit | ScalarValue::Null => return Ok(None),
        ScalarValue::Bool(v) => FieldValue::Bool(v),
        ScalarValue::I64(v) => FieldValue::I64(v),
        ScalarValue::U64(v) => FieldValue::U64(v),
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
            DatapointValue::Sensor(SensorValue::Digital { enabled })
            | DatapointValue::Actuator(ActuatorValue::Digital { enabled }) => {
                fields.insert(Cow::Borrowed("open"), FieldValue::Bool(enabled));
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
        DatapointKind::Actuator(ActuatorKind::Digital { .. }) => "valve_state",
        DatapointKind::Actuator(ActuatorKind::Fractional) => "throttable_valve_state",
        DatapointKind::Sensor(SensorKind::Analog { unit: "barg" }) => "pressure_sensor",
        DatapointKind::Sensor(SensorKind::Analog { unit: "°C" }) => "temperature_sensor",
        DatapointKind::Sensor(SensorKind::Analog { unit }) => {
            return Err(Error::UnknownSensorUnit(unit.to_owned()));
        }
        DatapointKind::Sensor(SensorKind::Digital { .. }) => {
            todo!("digital sensor measurement name not yet decided")
        }
    })
}

/// Build the dot-joined field name from the current path prefix and pending key.
fn collecting_field_name(
    path: &[Option<String>],
    pending_key: &mut Option<String>,
) -> Cow<'static, str> {
    let key = pending_key.take().unwrap_or_default();
    let prefix_parts: Vec<&str> = path.iter().filter_map(|p| p.as_deref()).collect();
    if prefix_parts.is_empty() {
        Cow::Owned(key)
    } else {
        Cow::Owned(format!("{}.{key}", prefix_parts.join(".")))
    }
}

/// Helper to convert a string to snake_case
fn to_snake_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i != 0 {
            out.push('_');
        }
        out.extend(c.to_lowercase());
    }
    out
}
