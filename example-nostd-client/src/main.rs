#![allow(dead_code)]

use data_core::pss::{self, lib::data::DatapointHolder};
use datatypes::{
    actuator::DPRValve,
    status::{SensorStatus, ValveState},
};

#[derive(pss::Data)]
enum AnyMessage {
    #[pss(kind = "dpr_actuator", name = "first_field")]
    PowerBoard(ValveState),
    #[unit("°C")]
    #[pss(kind = "analog_sensor", name = "second_field")]
    YetAnotherMessage(SensorStatus),
    #[pss(flatten)]
    OxidizerControlBoard(Asdf),
    NewMessage(Test),
}

#[derive(pss::Data)]
#[pss(kind = "digital_sensor", name = "third_field")]
#[unit("°F")]
struct Test(SensorStatus);

#[derive(pss::Data)]
struct Asdf {
    my_field: u16,
    #[unit("°C")]
    #[pss(kind = "dpr_actuator", name = "fourth_field")]
    my_other_field: DPRValve,
}

fn main() {
    let data_info = AnyMessage::DATAPOINTS;

    let msd = AnyMessage::OxidizerControlBoard(Asdf {
        my_field: 10,
        my_other_field: DPRValve::Disabled,
    });
    let values = msd.get_datapoint_values();
    println!("{data_info:?}");
    println!("{values:?}");
}
