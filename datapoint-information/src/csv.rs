//! InfluxDB annotated CSV logger for messages implementing [`CsvLoggable`] + [`DatapointHolder`].
//!
//! One CSV file is created per measurement (= sensor variant), lazily on first write.
//! Timestamps are Unix nanoseconds.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::Serialize;
use tokio::fs::File;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

use crate::datapoint::{
    ActuatorKind, DatapointHolder, DatapointInformation, DatapointKind, NamedDatapointValue,
    SensorKind,
};

/// Routes `(message, timestamp_ns)` to per-measurement InfluxDB annotated CSV files.
pub struct CsvLogger {
    dir: PathBuf,
    /// Keyed by `snake_case(sensor)` (= measurement name = file stem).
    writers: HashMap<String, MeasurementWriter>,
}

struct MeasurementWriter {
    writer: csv_async::AsyncSerializer<Compat<File>>,
}

impl CsvLogger {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self {
            dir: dir.into(),
            writers: HashMap::new(),
        }
    }

    /// Write one row for `msg` at `timestamp_ns` (Unix nanoseconds).
    ///
    /// Creates the CSV file and writes the annotated header on the first call
    /// for a given sensor variant.
    pub async fn log<M>(&mut self, msg: &M, timestamp_ns: u64) -> Result<(), Error>
    where
        M: Serialize + DatapointHolder,
    {
        let values = msg.get_datapoint_values();
        if !values.is_empty() {
            for (value, kind) in values.into_iter().zip(M::DATAPOINTS) {
                self.log_datapoint(value, kind, timestamp_ns).await?;
            }
            return Ok(());
        }

        // TODO: serialize. Figure out variant. If first, figure out headers. Then serialize.

        todo!()
    }

    async fn log_datapoint(
        &mut self,
        datapoint: NamedDatapointValue,
        kind: &'static DatapointInformation,
        timestamp_ns: u64,
    ) -> Result<(), Error> {
        let measurement = to_measurement(kind.kind);

        if !self.writers.contains_key(&measurement) {
            let mw =
                MeasurementWriter::create(&self.dir, &measurement, board, sensor, &values).await?;
            self.writers.insert(measurement.clone(), mw);
        }

        let mw = self.writers.get_mut(&measurement).expect("just inserted");

        let mut row: Vec<String> = Vec::with_capacity(3 + values.len() + 1);
        row.push(measurement);
        row.push(board.to_owned());
        row.push(sensor.to_owned());
        for ndv in &values {
            let f: f32 = ndv.value.into();
            row.push(f.to_string());
        }
        row.push(timestamp_ns.to_string());

        mw.writer.write_record(&row).await.map_err(Error::Csv)?;
        mw.writer.flush().await.map_err(Error::Csv)?;

        Ok(())
    }
}

impl MeasurementWriter {
    async fn create(
        dir: &std::path::Path,
        measurement: &str,
        // board/sensor only used in #default row as a hint – actual tag values come per-row
        _board: &str,
        _sensor: &str,
        values: &[NamedDatapointValue],
    ) -> Result<Self, Error> {
        let path = dir.join(format!("{measurement}.csv"));
        let file = File::create(&path).await.map_err(Error::Io)?;
        let compat = file.compat();
        let mut writer = csv_async::AsyncWriterBuilder::new()
            .has_headers(false)
            .create_writer(compat);

        // #datatype annotation row
        {
            let mut row: Vec<&str> = vec!["#datatype", "measurement", "tag", "tag"];
            for _ in values {
                row.push("double"); // FIXME
            }
            row.push("unsignedLong"); // timestamp as unix ns
            writer.write_record(&row).await.map_err(Error::Csv)?;
        }

        // Column name row
        {
            let mut row: Vec<&str> = vec!["", "datapoint", "system", "sensor_id"]; // fixme: only sensor id for datapoint
            for ndv in values {
                row.push(ndv.name);
            }
            row.push("time");
            writer.write_record(&row).await.map_err(Error::Csv)?;
        }

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
        DatapointKind::Sensor(SensorKind::Analog { .. }) => "BROKEN",
        DatapointKind::Sensor(SensorKind::Digital { .. }) => "BROKEN",
    }
}

// todo: thiserror
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Csv(csv_async::Error),
}
