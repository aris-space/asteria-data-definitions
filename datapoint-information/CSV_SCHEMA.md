# InfluxDB CSV Schema — Problem Specification

## Executive Summary

The rocket system produces a stream of typed messages from multiple boards
(FuelControlBoard, FlightComputer, etc.). These messages must be persisted to an
InfluxDB v2 time-series database for post-flight analysis and real-time monitoring.
InfluxDB organises data into measurements (tables), each with tags (indexed string
metadata), typed fields (the actual sensor readings), and a nanosecond timestamp.
Messages split cleanly into two categories: those carrying PSS-annotated sensor or
actuator readings, and those carrying structured status or metadata. Both categories
need to land in InfluxDB, but with different measurement structures. The mapping
described here defines what measurement each message writes to, which tags identify
the source, and which fields carry the payload.

---

## 1. InfluxDB v2 Data Model

InfluxDB v2 stores data as time series inside a _bucket_. Every data point belongs
to exactly one _measurement_, has zero or more _tags_, one or more _fields_, and a
_timestamp_.

| Concept     | Role                                             | Type                  |
| ----------- | ------------------------------------------------ | --------------------- |
| Measurement | The logical table name (e.g. `pressure_sensor`)  | string                |
| Tag         | Indexed metadata used to filter and group series | string                |
| Field       | The actual payload value being recorded          | float/int/bool/string |
| Timestamp   | When the reading was taken, in Unix nanoseconds  | uint64                |

Tags form the _series key_: all rows sharing the same measurement + tag set belong
to one time series. Fields are not indexed and carry the data. Choosing what is a
tag versus a field therefore has query-performance implications: source identity
belongs in tags; sensor readings belong in fields.

---

## 2. Our Message Structure

Every message in the system is an instance of a two-level enum:

```
Board variant         Inner variant        Leaf payload
──────────────────────────────────────────────────────
FuelControlBoard  →  FuelTankPressure  →  barg
FuelControlBoard  →  BuildInfo         →  { version, hash, … }
FlightComputer    →  ImuData           →  { accel_x, accel_y, … }
…
```

The **outer level** identifies which physical board emitted the message.
The **inner level** identifies the logical message type within that board.
The **leaf** is a type whose fields are the actual readings.

A subset of inner variants is annotated with PSS metadata. These annotations attach:

- a short unique identifier (PSS name, e.g. `fss_tnk_p1`)
- a semantic kind: analog sensor with unit, digital valve

The remaining inner variants carry no PSS annotations. They hold equally important information, but don't have a well-defined indentifier (firmware version, board health, position, power measurements).

---

## 3. Category A — PSS-Annotated Messages

These messages carry physical sensor or actuator readings. Each annotated field maps
independently to a measurement determined solely by its _kind_, not by which board
or variant it came from. Sensors of the same physical type (e.g. all pressure
sensors regardless of board) share a single measurement.

### 3.1 Measurement Name

| PSS Kind                   | Measurement name         |
| -------------------------- | ------------------------ |
| Analog sensor, unit `barg` | `pressure_sensor`        |
| Analog sensor, unit `°C`   | `temperature_sensor`     |
| Digital actuator (NC/NO)   | `valve_state`            |
| DPR actuator               | `dpr_valve_state`        |
| Fractional actuator        | `throttable_valve_state` |

Each measurement is a separate file / InfluxDB write stream.

### 3.2 Tags

| Tag key     | Value                                             |
| ----------- | ------------------------------------------------- |
| `sensor_id` | The PSS name of the datapoint (e.g. `fss_tnk_p1`) |

The PSS name already encodes subsystem and sensor identity, making it sufficient as
the sole series-identifying tag. It is unique across the entire system by construction.

### 3.3 Fields

The field set depends on the datapoint kind:

| Datapoint kind        | Fields                                         |
| --------------------- | ---------------------------------------------- |
| Analog sensor         | `value` (double)                               |
| Digital sensor        | `open` (boolean)                               |
| Digital actuator      | `open` (boolean)                               |
| DPR actuator — open   | `open` (boolean, true), `setpoint` (double)    |
| DPR actuator — closed | `open` (boolean, false) — `setpoint` is absent |
| Fractional actuator   | `openness` (double, 0.0–1.0)                   |

---

## 4. Category B — Non-PSS Messages

These messages carry board status, firmware metadata, or other structured data.
There is no physical-quantity kind to group them by, so each inner type gets
its own measurement.

### 4.1 Measurement Name

The measurement name is the type name converted to snake_case.

| Inner variant       | Measurement name |
| ------------------- | ---------------- |
| `BuildInfoCommon`   | `build_info`     |
| `BoardStatusCommon` | `board_status`   |

### 4.2 Tags

| Tag key   | Value                                        |
| --------- | -------------------------------------------- |
| `board`   | Outer variant name (e.g. `FuelControlBoard`) |
| `message` | Inner variant name (e.g. `BuildInfo`)        |

Together these two tags uniquely identify the series source.

### 4.3 Fields

All fields of the leaf struct become InfluxDB fields. The column name is the
serialised field name (respecting any rename conventions). The InfluxDB type is
inferred from the Rust type:

| Rust primitive       | InfluxDB type  |
| -------------------- | -------------- |
| `bool`               | `boolean`      |
| `i8`–`i64`           | `long`         |
| `u8`–`u64`           | `unsignedLong` |
| `f32`, `f64`         | `double`       |
| `&str`, `String`     | `string`       |
| enum (unit variants) | `string`       |
| nested struct        | flattened      |

### 4.4 Struct Flattening

Nested structs field names use dot separated field names, e.g. `common.errors` for a field named `common`
that holds a struct with a field named `errors`.

---

## 5. Timestamp

All rows, both category A and B, carry a single `time` column of type
`unsignedLong` holding the Unix nanosecond timestamp supplied by the caller at
log time. The message itself carries no embedded timestamp.

---

## 6. File Layout

One CSV file is produced per measurement. Files are created on demand when the first
message of that measurement is logged. The filename is `measurements/<measurement>.csv`.

```
logs/measurements/
  pressure_sensor.csv
  temperature_sensor.csv
  valve_state.csv
  dpr_valve_state.csv
  build_info.csv
  board_status.csv
  …
```

Each file uses InfluxDB annotated CSV format, which begins with a `#datatype`
annotation row specifying the column roles, followed by a header row with column
names, followed by data rows.
Read <https://docs.influxdata.com/influxdb/v2/reference/syntax/annotated-csv/>
