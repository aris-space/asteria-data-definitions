use gated_derive::gated_derive;

#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case", tag = "kind")]
#[schema(as = ActuatorKind)]
pub enum ActuatorKind {
    Digital {
        default_enabled: bool,
    },
    #[serde(rename = "dpr")]
    DPR,
    Fractional,
}

#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case", tag = "kind")]
#[schema(as = SensorKind)]
pub enum SensorKind {
    Analog { unit: &'static str },
    Digital { default_enabled: bool },
}

#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[serde(tag = "component", rename_all = "snake_case")]
#[schema(as = DatapointKind)]
pub enum DatapointKind {
    Actuator(ActuatorKind),
    Sensor(SensorKind),
}

/// Compile time information about a PSS datapoint
#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[schema(as = DatapointInformation)]
pub struct DatapointInformation {
    /// Unique name of the datapoint
    pub name: &'static str,
    /// User-facing label of the datapoint
    pub label: &'static str,
    /// Kind of the datapoint (e.g. DPR actuator, digital sensor, ...)
    pub kind: DatapointKind,
}

#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case", tag = "kind", content = "value")]
#[schema(as = ActuatorValue)]
pub enum ActuatorValue {
    Digital {
        enabled: bool,
    },
    #[serde(rename = "dpr")]
    DPR {
        enabled: bool,
        setpoint: f32,
    },
    Fractional {
        /// Openness in range [0, 1].
        fraction: f32,
    },
}

impl From<ActuatorValue> for f32 {
    fn from(value: ActuatorValue) -> Self {
        match value {
            ActuatorValue::Digital { enabled } => enabled.into(),
            ActuatorValue::DPR { enabled: false, .. } => f32::NAN,
            ActuatorValue::DPR { setpoint, .. } => setpoint,
            ActuatorValue::Fractional { fraction } => fraction,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case", tag = "kind", content = "value")]
#[schema(as = SensorValue)]
pub enum SensorValue {
    Analog { value: f32 },
    Digital { enabled: bool },
}

impl From<SensorValue> for f32 {
    fn from(value: SensorValue) -> Self {
        match value {
            SensorValue::Analog { value } => value,
            SensorValue::Digital { enabled } => enabled.into(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[serde(tag = "component", rename_all = "snake_case")]
#[schema(as = DatapointValue)]
pub enum DatapointValue {
    Actuator(ActuatorValue),
    Sensor(SensorValue),
}

impl From<DatapointValue> for f32 {
    fn from(value: DatapointValue) -> Self {
        match value {
            DatapointValue::Actuator(actuator_value) => actuator_value.into(),
            DatapointValue::Sensor(sensor_value) => sensor_value.into(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(as = NamedDatapointValue)]
pub struct NamedDatapointValue {
    /// Unique name of the datapoint
    pub name: &'static str,
    /// Value of the datapoint
    pub value: DatapointValue,
}

/// Trait which can be implemented on enums and structs to recursively
/// collect fields which contain PSS datapoints
pub trait DatapointHolder {
    const DATAPOINTS: &[DatapointInformation] = &[];

    /// Recursively get all datapoint values for this instance
    fn get_datapoint_values(&self) -> Vec<NamedDatapointValue> {
        Vec::new()
    }
}

/// Trait to turn instances of datapoint types into [`DatapointValue`]s.
/// At the same time, it's also used to get the [`DatapointKind`] to which the
/// datapoint type maps.
pub trait IntoDatapointValue {
    /// The [`DatapointKind`] to which the current type maps.
    const KIND: DatapointKind;

    /// Convert an instance of the current type into a [`DatapointValue`].
    fn into_value(self) -> DatapointValue;
}
