//! This crate exposes traits to gain additional metadata about messages,
//! mostly for sensors and valves to support dynamic UI generation.

pub mod datapoint;
pub mod macro_util;

#[cfg(feature = "csv")]
pub mod csv_f;

facet::define_attr_grammar! {
    ns "datapoint";
    crate_path ::datapoint_information;

    pub enum Attr {
        SensorId(&'static str),
        Container,
    }
}
