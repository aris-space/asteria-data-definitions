//! InfluxDB annotated CSV logger for [`DatapointHolder`] messages.
//!
//! Each datapoint is routed to a per-measurement CSV file derived from its
//! [`DatapointKind`]. Files are created lazily on first write.
//! Timestamps are Unix nanoseconds.

use std::collections::HashMap;
use std::path::PathBuf;

use tokio::fs::File;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

use crate::datapoint::{
    ActuatorKind, DatapointHolder, DatapointInformation, DatapointKind, NamedDatapointValue,
    SensorKind,
};

/// Routes `(message, timestamp_ns)` to per-measurement InfluxDB annotated CSV files.
///
/// One file is created per measurement type (e.g. `pressure_sensor.csv`).
/// Rows are keyed by the datapoint's PSS name as `sensor_id`.
pub struct CsvLogger {
    dir: PathBuf,
    /// Keyed by measurement name (a `&'static str` from [`to_measurement`]).
    writers: HashMap<&'static str, MeasurementWriter>,
}

struct MeasurementWriter {
    writer: csv_async::AsyncWriter<Compat<File>>,
}

impl CsvLogger {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self {
            dir: dir.into(),
            writers: HashMap::new(),
        }
    }

    /// Write one row per datapoint in `msg` at `timestamp_ns` (Unix nanoseconds).
    ///
    /// Messages with no PSS datapoints (e.g. `BuildInfo`) are silently skipped.
    /// Creates measurement files and writes annotated headers on first use.
    pub async fn log<M>(&mut self, msg: &M, timestamp_ns: u64) -> Result<(), Error>
    where
        M: serde::Serialize + DatapointHolder,
    {
        let values = msg.get_datapoint_values();
        if !values.is_empty() {
            for (value, info) in values.into_iter().zip(M::DATAPOINTS) {
                self.log_datapoint(value, info, timestamp_ns).await?;
            }
            return Ok(());
        }

        // Message has no PSS datapoints (e.g. BuildInfo) — log via serde.
        // TODO: implement serde-based row writing (AsyncSerializer path, separate writers map).
        let _schema = inspect::inspect(msg);
        todo!("serde-based logging for non-datapoint messages")
    }

    async fn log_datapoint(
        &mut self,
        datapoint: NamedDatapointValue,
        info: &'static DatapointInformation,
        timestamp_ns: u64,
    ) -> Result<(), Error> {
        let measurement = to_measurement(info.kind);

        if !self.writers.contains_key(measurement) {
            let mw = MeasurementWriter::create(&self.dir, measurement).await?;
            self.writers.insert(measurement, mw);
        }

        let mw = self.writers.get_mut(measurement).expect("just inserted");

        let f: f32 = datapoint.value.into();
        mw.writer
            .write_record(&[
                measurement,
                info.name,
                &f.to_string(),
                &timestamp_ns.to_string(),
            ])
            .await
            .map_err(Error::Csv)?;
        mw.writer.flush().await.map_err(Error::Csv)?;

        Ok(())
    }
}

impl MeasurementWriter {
    async fn create(dir: &std::path::Path, measurement: &str) -> Result<Self, Error> {
        let path = dir.join(format!("{measurement}.csv"));
        let file = File::create(&path).await.map_err(Error::Io)?;
        let compat = file.compat();
        let mut writer = csv_async::AsyncWriterBuilder::new()
            .has_headers(false)
            .create_writer(compat);

        // InfluxDB annotated CSV: #datatype row
        writer
            .write_record(&["#datatype", "measurement", "tag", "double", "unsignedLong"])
            .await
            .map_err(Error::Csv)?;

        // Column name row
        writer
            .write_record(&["m", "sensor_id", "value", "time"])
            .await
            .map_err(Error::Csv)?;

        writer.flush().await.map_err(Error::Csv)?;

        Ok(Self { writer })
    }
}

fn to_measurement(k: DatapointKind) -> &'static str {
    match k {
        DatapointKind::Actuator(ActuatorKind::DPR) => "dpr_valve_state",
        DatapointKind::Actuator(ActuatorKind::Digital { .. }) => "valve_state",
        DatapointKind::Actuator(ActuatorKind::Fractional) => "throttable_valve_state",
        DatapointKind::Sensor(SensorKind::Analog { unit: "barg" }) => "pressure_sensor",
        DatapointKind::Sensor(SensorKind::Analog { unit: "°C" }) => "temperature_sensor",
        DatapointKind::Sensor(SensorKind::Analog { unit }) => {
            panic!(
                "no measurement mapping for analog sensor with unit {unit:?} — add it to to_measurement()"
            )
        }
        DatapointKind::Sensor(SensorKind::Digital { .. }) => {
            // No digital sensor types exist in datatypes yet.
            // When added, decide on a measurement name and add a match arm above.
            todo!("digital sensor measurement name")
        }
    }
}

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

/// Inspect a two-level `#[serde(tag, content)]` enum, collecting variant names and field schemas.
///
/// Designed for the top-level `Message` enum:
/// ```text
/// Message::FuelControlBoard(dp_fuel_control_board::Message::BuildInfo(BuildInfo { .. }))
///   → Inspection { outer: "FuelControlBoard", inner: "BuildInfo", fields: [...] }
/// ```
pub mod inspect {
    use serde::ser::{self, Impossible, Serialize};

    #[derive(Debug)]
    struct InspectError;
    impl std::fmt::Display for InspectError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("inspect error")
        }
    }
    impl ser::Error for InspectError {
        fn custom<T: std::fmt::Display>(_: T) -> Self {
            InspectError
        }
    }

    #[derive(Debug)]
    pub struct Inspection {
        /// Outer variant name, e.g. `"FuelControlBoard"`.
        pub outer_variant: String,
        /// Inner variant name, e.g. `"BuildInfo"`.
        pub inner_variant: String,
        /// Leaf struct fields with inferred InfluxDB types.
        pub fields: Vec<Field>,
    }

    #[derive(Debug)]
    pub struct Field {
        /// Serde field name (respects `#[serde(rename)]` / `rename_all`).
        pub name: String,
        /// InfluxDB `#datatype` value: `"boolean"`, `"long"`, `"unsignedLong"`, `"double"`, or `"string"`.
        pub influx_type: &'static str,
    }

    /// Inspect `msg` and return its two-level enum schema, or `None` if the type
    /// does not serialize as a two-level adjacently-tagged enum.
    pub fn inspect<M: Serialize>(msg: &M) -> Option<Inspection> {
        msg.serialize(OuterSer).ok().flatten()
    }

    // ── Outer level ────────────────────────────────────────────────────────

    struct OuterSer;

    impl ser::Serializer for OuterSer {
        type Ok = Option<Inspection>;
        type Error = InspectError;
        type SerializeSeq = Impossible<Option<Inspection>, InspectError>;
        type SerializeTuple = Impossible<Option<Inspection>, InspectError>;
        type SerializeTupleStruct = Impossible<Option<Inspection>, InspectError>;
        type SerializeTupleVariant = Impossible<Option<Inspection>, InspectError>;
        type SerializeMap = Impossible<Option<Inspection>, InspectError>;
        type SerializeStruct = OuterStructSer;
        type SerializeStructVariant = Impossible<Option<Inspection>, InspectError>;

        fn serialize_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            Ok(OuterStructSer {
                outer_variant: None,
                result: None,
            })
        }
        fn serialize_newtype_struct<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            v: &T,
        ) -> Result<Self::Ok, Self::Error> {
            v.serialize(self)
        }
        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_some<T: ?Sized + Serialize>(self, _: &T) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_unit_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_newtype_variant<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: &T,
        ) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            Err(InspectError)
        }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
            Err(InspectError)
        }
        fn serialize_tuple_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            Err(InspectError)
        }
        fn serialize_tuple_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            Err(InspectError)
        }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Err(InspectError)
        }
        fn serialize_struct_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            Err(InspectError)
        }
    }

    struct OuterStructSer {
        outer_variant: Option<String>,
        result: Option<Inspection>,
    }

    impl ser::SerializeStruct for OuterStructSer {
        type Ok = Option<Inspection>;
        type Error = InspectError;

        fn serialize_field<T: ?Sized + Serialize>(
            &mut self,
            _key: &'static str,
            value: &T,
        ) -> Result<(), Self::Error> {
            if self.outer_variant.is_none() {
                // First field: the tag value, e.g. "FuelControlBoard"
                self.outer_variant = value.serialize(StrSer).ok().flatten();
            } else if let Some(ref outer) = self.outer_variant.clone() {
                // Second field: the inner enum
                self.result = value
                    .serialize(InnerSer {
                        outer: outer.clone(),
                    })
                    .ok()
                    .flatten();
            }
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(self.result)
        }
    }

    // ── Inner level ────────────────────────────────────────────────────────

    struct InnerSer {
        outer: String,
    }

    impl ser::Serializer for InnerSer {
        type Ok = Option<Inspection>;
        type Error = InspectError;
        type SerializeSeq = Impossible<Option<Inspection>, InspectError>;
        type SerializeTuple = Impossible<Option<Inspection>, InspectError>;
        type SerializeTupleStruct = Impossible<Option<Inspection>, InspectError>;
        type SerializeTupleVariant = Impossible<Option<Inspection>, InspectError>;
        type SerializeMap = Impossible<Option<Inspection>, InspectError>;
        type SerializeStruct = InnerStructSer;
        type SerializeStructVariant = Impossible<Option<Inspection>, InspectError>;

        fn serialize_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            Ok(InnerStructSer {
                outer: self.outer,
                inner_variant: None,
                result: None,
            })
        }
        fn serialize_newtype_struct<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            v: &T,
        ) -> Result<Self::Ok, Self::Error> {
            v.serialize(self)
        }
        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_some<T: ?Sized + Serialize>(self, _: &T) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_unit_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_newtype_variant<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: &T,
        ) -> Result<Self::Ok, Self::Error> {
            Ok(None)
        }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            Err(InspectError)
        }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
            Err(InspectError)
        }
        fn serialize_tuple_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            Err(InspectError)
        }
        fn serialize_tuple_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            Err(InspectError)
        }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Err(InspectError)
        }
        fn serialize_struct_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            Err(InspectError)
        }
    }

    struct InnerStructSer {
        outer: String,
        inner_variant: Option<String>,
        result: Option<Inspection>,
    }

    impl ser::SerializeStruct for InnerStructSer {
        type Ok = Option<Inspection>;
        type Error = InspectError;

        fn serialize_field<T: ?Sized + Serialize>(
            &mut self,
            _key: &'static str,
            value: &T,
        ) -> Result<(), Self::Error> {
            if self.inner_variant.is_none() {
                // First field: inner tag value, e.g. "BuildInfo"
                self.inner_variant = value.serialize(StrSer).ok().flatten();
            } else if let Some(ref inner) = self.inner_variant.clone() {
                // Second field: the leaf struct
                let fields = value.serialize(FieldsSer).ok().unwrap_or_default();
                self.result = Some(Inspection {
                    outer_variant: self.outer.clone(),
                    inner_variant: inner.clone(),
                    fields,
                });
            }
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(self.result)
        }
    }

    // ── Field collection ───────────────────────────────────────────────────

    struct FieldsSer;

    impl ser::Serializer for FieldsSer {
        type Ok = Vec<Field>;
        type Error = InspectError;
        type SerializeSeq = Impossible<Vec<Field>, InspectError>;
        type SerializeTuple = Impossible<Vec<Field>, InspectError>;
        type SerializeTupleStruct = Impossible<Vec<Field>, InspectError>;
        type SerializeTupleVariant = Impossible<Vec<Field>, InspectError>;
        type SerializeMap = Impossible<Vec<Field>, InspectError>;
        type SerializeStruct = FieldsStructSer;
        type SerializeStructVariant = Impossible<Vec<Field>, InspectError>;

        fn serialize_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            Ok(FieldsStructSer { fields: Vec::new() })
        }
        fn serialize_newtype_struct<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            v: &T,
        ) -> Result<Self::Ok, Self::Error> {
            v.serialize(self)
        }
        fn serialize_newtype_variant<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            v: &T,
        ) -> Result<Self::Ok, Self::Error> {
            v.serialize(self)
        }
        fn serialize_some<T: ?Sized + Serialize>(self, v: &T) -> Result<Self::Ok, Self::Error> {
            v.serialize(self)
        }
        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_unit_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Ok(vec![])
        }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            Err(InspectError)
        }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
            Err(InspectError)
        }
        fn serialize_tuple_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            Err(InspectError)
        }
        fn serialize_tuple_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            Err(InspectError)
        }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Err(InspectError)
        }
        fn serialize_struct_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            Err(InspectError)
        }
    }

    struct FieldsStructSer {
        fields: Vec<Field>,
    }

    impl ser::SerializeStruct for FieldsStructSer {
        type Ok = Vec<Field>;
        type Error = InspectError;

        fn serialize_field<T: ?Sized + Serialize>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<(), Self::Error> {
            let influx_type = value.serialize(TypeSer).unwrap_or("string");
            self.fields.push(Field {
                name: key.to_owned(),
                influx_type,
            });
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(self.fields)
        }
    }

    // ── Type inference ─────────────────────────────────────────────────────

    struct TypeSer;

    /// Returned by [`TypeSer::serialize_struct`] — skips nested fields, yields `"string"`.
    struct NestedAsString;

    impl ser::SerializeStruct for NestedAsString {
        type Ok = &'static str;
        type Error = InspectError;
        fn serialize_field<T: ?Sized + Serialize>(
            &mut self,
            _: &'static str,
            _: &T,
        ) -> Result<(), InspectError> {
            Ok(())
        }
        fn end(self) -> Result<&'static str, InspectError> {
            Ok("string")
        }
    }

    impl ser::Serializer for TypeSer {
        type Ok = &'static str;
        type Error = InspectError;
        type SerializeSeq = Impossible<&'static str, InspectError>;
        type SerializeTuple = Impossible<&'static str, InspectError>;
        type SerializeTupleStruct = Impossible<&'static str, InspectError>;
        type SerializeTupleVariant = Impossible<&'static str, InspectError>;
        type SerializeMap = Impossible<&'static str, InspectError>;
        type SerializeStruct = NestedAsString;
        type SerializeStructVariant = Impossible<&'static str, InspectError>;

        fn serialize_bool(self, _: bool) -> Result<&'static str, InspectError> {
            Ok("boolean")
        }
        fn serialize_i8(self, _: i8) -> Result<&'static str, InspectError> {
            Ok("long")
        }
        fn serialize_i16(self, _: i16) -> Result<&'static str, InspectError> {
            Ok("long")
        }
        fn serialize_i32(self, _: i32) -> Result<&'static str, InspectError> {
            Ok("long")
        }
        fn serialize_i64(self, _: i64) -> Result<&'static str, InspectError> {
            Ok("long")
        }
        fn serialize_u8(self, _: u8) -> Result<&'static str, InspectError> {
            Ok("unsignedLong")
        }
        fn serialize_u16(self, _: u16) -> Result<&'static str, InspectError> {
            Ok("unsignedLong")
        }
        fn serialize_u32(self, _: u32) -> Result<&'static str, InspectError> {
            Ok("unsignedLong")
        }
        fn serialize_u64(self, _: u64) -> Result<&'static str, InspectError> {
            Ok("unsignedLong")
        }
        fn serialize_f32(self, _: f32) -> Result<&'static str, InspectError> {
            Ok("double")
        }
        fn serialize_f64(self, _: f64) -> Result<&'static str, InspectError> {
            Ok("double")
        }
        fn serialize_char(self, _: char) -> Result<&'static str, InspectError> {
            Ok("string")
        }
        fn serialize_str(self, _: &str) -> Result<&'static str, InspectError> {
            Ok("string")
        }
        fn serialize_bytes(self, _: &[u8]) -> Result<&'static str, InspectError> {
            Ok("string")
        }
        fn serialize_none(self) -> Result<&'static str, InspectError> {
            Ok("string")
        }
        fn serialize_some<T: ?Sized + Serialize>(
            self,
            v: &T,
        ) -> Result<&'static str, InspectError> {
            v.serialize(self)
        }
        fn serialize_unit(self) -> Result<&'static str, InspectError> {
            Ok("string")
        }
        fn serialize_unit_struct(self, _: &'static str) -> Result<&'static str, InspectError> {
            Ok("string")
        }
        fn serialize_unit_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
        ) -> Result<&'static str, InspectError> {
            Ok("string")
        }
        fn serialize_newtype_struct<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            v: &T,
        ) -> Result<&'static str, InspectError> {
            v.serialize(self)
        }
        fn serialize_newtype_variant<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            v: &T,
        ) -> Result<&'static str, InspectError> {
            v.serialize(self)
        }
        fn serialize_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStruct, InspectError> {
            Ok(NestedAsString)
        }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, InspectError> {
            Err(InspectError)
        }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, InspectError> {
            Err(InspectError)
        }
        fn serialize_tuple_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleStruct, InspectError> {
            Err(InspectError)
        }
        fn serialize_tuple_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleVariant, InspectError> {
            Err(InspectError)
        }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, InspectError> {
            Err(InspectError)
        }
        fn serialize_struct_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStructVariant, InspectError> {
            Err(InspectError)
        }
    }

    // ── String extraction ──────────────────────────────────────────────────

    /// Extracts the first string value serde produces — works for tag fields and unit variants.
    struct StrSer;

    impl ser::Serializer for StrSer {
        type Ok = Option<String>;
        type Error = InspectError;
        type SerializeSeq = Impossible<Option<String>, InspectError>;
        type SerializeTuple = Impossible<Option<String>, InspectError>;
        type SerializeTupleStruct = Impossible<Option<String>, InspectError>;
        type SerializeTupleVariant = Impossible<Option<String>, InspectError>;
        type SerializeMap = Impossible<Option<String>, InspectError>;
        type SerializeStruct = Impossible<Option<String>, InspectError>;
        type SerializeStructVariant = Impossible<Option<String>, InspectError>;

        fn serialize_str(self, v: &str) -> Result<Option<String>, InspectError> {
            Ok(Some(v.to_owned()))
        }
        fn serialize_unit_variant(
            self,
            _: &'static str,
            _: u32,
            v: &'static str,
        ) -> Result<Option<String>, InspectError> {
            Ok(Some(v.to_owned()))
        }
        fn serialize_newtype_struct<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            v: &T,
        ) -> Result<Option<String>, InspectError> {
            v.serialize(self)
        }
        fn serialize_bool(self, _: bool) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_i8(self, _: i8) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_i16(self, _: i16) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_i32(self, _: i32) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_i64(self, _: i64) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_u8(self, _: u8) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_u16(self, _: u16) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_u32(self, _: u32) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_u64(self, _: u64) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_f32(self, _: f32) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_f64(self, _: f64) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_char(self, _: char) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_bytes(self, _: &[u8]) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_none(self) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_some<T: ?Sized + Serialize>(
            self,
            _: &T,
        ) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_unit(self) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_newtype_variant<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: &T,
        ) -> Result<Option<String>, InspectError> {
            Ok(None)
        }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, InspectError> {
            Err(InspectError)
        }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, InspectError> {
            Err(InspectError)
        }
        fn serialize_tuple_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleStruct, InspectError> {
            Err(InspectError)
        }
        fn serialize_tuple_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleVariant, InspectError> {
            Err(InspectError)
        }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, InspectError> {
            Err(InspectError)
        }
        fn serialize_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStruct, InspectError> {
            Err(InspectError)
        }
        fn serialize_struct_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStructVariant, InspectError> {
            Err(InspectError)
        }
    }
}

// todo: thiserror instead of manual
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Csv(csv_async::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "io: {e}"),
            Error::Csv(e) => write!(f, "csv: {e}"),
        }
    }
}

impl std::error::Error for Error {}
