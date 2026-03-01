#![allow(dead_code)]

use data_core::pss::{self, lib::data::DatapointHolder};
use datatypes::{actuator::DPRValve, status::SensorStatus};

#[derive(pss::Data)]
enum AnyMessage {
    #[pss(type = "binary", id = "first_field")]
    PowerBoard(u8),
    #[pss(type = "binary", id = "second_field")]
    YetAnotherMessage(u16),
    OxidizerControlBoard(Asdf),
    NewMessage(Test),
}

#[derive(pss::Data)]
#[pss(type = "binary", id = "third_field")]
#[unit("°F")]
struct Test(SensorStatus);

#[derive(pss::Data)]
struct Asdf {
    my_field: u16,
    #[unit("°C")]
    #[pss(type = "binary", id = "fourth_field")]
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
