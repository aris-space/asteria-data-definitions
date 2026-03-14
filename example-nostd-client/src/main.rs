#![allow(dead_code)]

use data_core::pss::{self, lib::data::DatapointHolder};
use datatypes::{
    actuator::DPRValve,
    status::{SensorStatus, ValveState},
};
use serde::Serialize;

#[derive(pss::Data)]
#[derive(Serialize)]
enum AnyMessage {
    #[pss(kind = "dpr_actuator", name = "first_field", label = "First Field")]
    PowerBoard(ValveState),
    #[unit("°C")]
    #[pss(kind = "analog_sensor", name = "second_field", label = "Second Field")]
    YetAnotherMessage(SensorStatus),
    #[pss(flatten)]
    OxidizerControlBoard(Asdf),
    #[pss(flatten)]
    NewMessage(Test),
}

#[derive(pss::Data)]
#[pss(kind = "digital_sensor", name = "third_field", label = "Third Field")]
#[unit("°F")]
#[derive(Serialize)]
struct Test(SensorStatus);

#[derive(pss::Data)]
#[derive(Serialize)]
struct Asdf {
    my_field: u16,
    #[unit("°C")]
    #[pss(kind = "dpr_actuator", name = "fourth_field", label = "Fourth Field")]
    my_other_field: DPRValve,
}

fn main() {
    let data_info = AnyMessage::DATAPOINTS;

    let msd = AnyMessage::OxidizerControlBoard(Asdf {
        my_field: 10,
        my_other_field: DPRValve::Disabled,
    });
    let values = msd.get_datapoint_values();
    let json = serde_json::to_string_pretty(&data_info).expect("asdf");
    // println!("{data_info:?}");
    // println!("{values:?}");
    println!("{json}");
}
