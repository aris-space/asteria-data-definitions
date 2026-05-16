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
