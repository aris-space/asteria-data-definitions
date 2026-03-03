use gated_derive::gated_derive;

#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case", tag = "kind")]
#[schema(as = ActuatorKind)]
pub enum ActuatorKind {
    Digital,
    #[serde(rename = "dpr")]
    DPR,
    Fractional
}

#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case", tag = "kind", content = "value")]
#[schema(as = SensorKind)]
pub enum SensorKind {
    Analog { unit: &'static str },
    Digital
}

#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[serde(tag = "component", rename_all = "snake_case")]
#[schema(as = DatapointKind)]
pub enum DatapointKind {
    Actuator(ActuatorKind),
    Sensor(SensorKind)
}

/// Compile time information about a PSS datapoint
#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[schema(as = DatapointInformation)]
pub struct DatapointInformation {
    /// Unique name of the datapoint
    pub name: &'static str,
    /// Kind of the datapoint (e.g. DPR actuator, digital sensor, ...)
    pub kind: DatapointKind,
}

#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(as = ActuatorValue)]
pub enum ActuatorValue {
    Digital {
        active: bool,
    },
    #[serde(rename = "dpr")]
    DPR {
        enabled: bool,
        setpoint: f32,
    },
    Fractional {
        precent: f32,
    },
}

#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(as = SensorValue)]
pub enum SensorValue {
    Analog { value: f32 },
    Digital { enabled: bool },
}

#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
#[schema(as = DatapointValue)]
pub enum DatapointValue {
    Actuator(ActuatorValue),
    Sensor(SensorValue),
}

#[derive(Clone, Copy, Debug)]
#[gated_derive(serde::Serialize, utoipa::ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
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
