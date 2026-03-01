use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Copy, Serialize, ToSchema, Debug)]
#[serde(rename_all = "snake_case")]
#[schema(as = PssDatapointType)]
pub enum DatapointType {
    #[serde(rename = "dpr")]
    DPR,
    InteractableBinary,
    Binary,
    Value,
}

/// Compile time information about a PSS datapoint
#[derive(Clone, Copy, Serialize, ToSchema, Debug)]
#[schema(as = PssDatapointInformation)]
pub struct DatapointInformation {
    /// Identifier of the datapoint
    pub id: &'static str,
    /// Optional unit of the datapoint
    pub unit: Option<&'static str>,
    /// Type of the datapoint
    ///
    /// This can be used for handling the received [`f32`] data
    /// in the correct format
    #[serde(rename = "type")]
    pub data_type: DatapointType,
}

/// Specific value of a PSS datapoint
///
/// Using [`DatapointInformation::data_type`] the data can be
/// interpreted in the correct format
#[derive(Serialize, Debug)]
pub struct DatapointValue {
    pub id: &'static str,
    /// Type of the datapoint (e.g. valve, dpr valve, ...)
    pub data: f32,
}

/// Trait which can be implemented on enums and structs to recursively
/// collect fields which contain PSS datapoints
pub trait DatapointHolder {
    const DATAPOINTS: &[DatapointInformation] = &[];

    /// Recursively get all datapoint values for this instance
    fn get_datapoint_values(&self) -> Vec<DatapointValue> {
        Vec::new()
    }
}

macro_rules! default_dp_holder {
    (
        $($ty:ty),* $(,)?
    ) => {
        $(
            impl DatapointHolder for $ty {}
            impl<const N: usize> DatapointHolder for [$ty; N] {}
        )*
    };
}

// Implement [`DatapointHolder`] for all primitives and primitive arrays we also support
// through MavLink. All other field types need to implement [`DatapointHandler`] manually.
default_dp_holder! {
    bool,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    f32,
    u64,
    i64,
    f64
}
