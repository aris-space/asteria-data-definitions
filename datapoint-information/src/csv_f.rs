use std::{borrow::Cow, collections::BTreeMap};

use facet::{EnumType, Facet, StructType, Type, UserType, Variant};
use facet_format::FormatSerializer;
use facet_reflect::{HasFields, Peek, PeekEnum};

#[derive(Debug, Clone)]
pub enum FieldValue {
    // Includes binary enums ??
    Bool(bool),
    // Includes f32
    F64(f64),
    // Includes timestamps, and all other ints
    I64(i64),
    // Includes enums
    String(Cow<'static, str>),
}

#[derive(Debug, Clone)]
pub struct DataPoint {
    measurement: Cow<'static, str>,
    tags: BTreeMap<Cow<'static, str>, Cow<'static, str>>,
    fields: BTreeMap<Cow<'static, str>, FieldValue>,
    timestamp: Option<i64>,
}

struct Config {
    mission_id: &'static str,
}

#[derive(Debug)]
enum Error {
    // e.g. unsupported type/struct
}

enum SerState {
    Init,
    DatapointEnum { board: &'static str },
    Message { datapoint: DataPoint },
    Finished(DataPoint),
}

struct InfluxSerializer {
    config: Config,
    state: SerState,
}

impl FormatSerializer for InfluxSerializer {
    type Error = Error;

    fn begin_struct(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn field_key(&mut self, key: &str) -> Result<(), Self::Error> {
        todo!()
    }

    fn end_struct(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn begin_seq(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn end_seq(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn scalar(&mut self, scalar: facet_format::ScalarValue<'_>) -> Result<(), Self::Error> {
        todo!()
    }
}
