use datapoint_information::datapoint::{
    ActuatorKind, ActuatorValue, DatapointKind, DatapointValue, IntoDatapointValue, SensorKind,
    SensorValue,
};

use crate::actuator::{
    DPRValve, DigitalOut, InvertedDigitalOut, NormallyClosedValve, NormallyOpenValve,
    ThrottleableValve,
};
use crate::sensor::DigitalIn;
use crate::status::ArmingState;
use crate::units::{BarG, Celsius};

impl IntoDatapointValue for NormallyClosedValve {
    const KIND: DatapointKind = DatapointKind::Actuator(ActuatorKind::Digital {
        default_enabled: false,
    });

    fn into_value(self) -> DatapointValue {
        DatapointValue::Actuator(ActuatorValue::Digital {
            enabled: matches!(self, NormallyClosedValve::Open),
        })
    }
}

impl IntoDatapointValue for NormallyOpenValve {
    const KIND: DatapointKind = DatapointKind::Actuator(ActuatorKind::Digital {
        default_enabled: true,
    });

    fn into_value(self) -> DatapointValue {
        DatapointValue::Actuator(ActuatorValue::Digital {
            enabled: matches!(self, NormallyOpenValve::Open),
        })
    }
}

impl IntoDatapointValue for DigitalIn {
    // `DigitalIn::default()` is `Off`, which maps to `enabled: false`.
    const KIND: DatapointKind = DatapointKind::Sensor(SensorKind::Digital {
        default_enabled: false,
    });

    fn into_value(self) -> DatapointValue {
        DatapointValue::Sensor(SensorValue::Digital {
            enabled: matches!(self, DigitalIn::On),
        })
    }
}

/// Using a Arming state as a digital sensor
impl IntoDatapointValue for ArmingState {
    const KIND: DatapointKind = DatapointKind::Sensor(SensorKind::Digital {
        default_enabled: false,
    });

    fn into_value(self) -> DatapointValue {
        DatapointValue::Sensor(SensorValue::Digital {
            enabled: matches!(self, ArmingState::Armed),
        })
    }
}

impl IntoDatapointValue for DigitalOut {
    const KIND: DatapointKind = DatapointKind::Actuator(ActuatorKind::Digital {
        default_enabled: false,
    });

    fn into_value(self) -> DatapointValue {
        DatapointValue::Actuator(ActuatorValue::Digital {
            enabled: matches!(self, DigitalOut::On),
        })
    }
}

impl IntoDatapointValue for InvertedDigitalOut {
    // `InvertedDigitalOut::default()` is `On`, which maps to `enabled: true`.
    // The inversion semantic (Off = energized) lives in `Energizable`, not in this reporting layer.
    const KIND: DatapointKind = DatapointKind::Actuator(ActuatorKind::Digital {
        default_enabled: true,
    });

    fn into_value(self) -> DatapointValue {
        DatapointValue::Actuator(ActuatorValue::Digital {
            enabled: matches!(self, InvertedDigitalOut::On),
        })
    }
}

impl IntoDatapointValue for DPRValve {
    const KIND: DatapointKind = DatapointKind::Actuator(ActuatorKind::DPR);

    fn into_value(self) -> DatapointValue {
        let actuator = match self {
            DPRValve::Enabled { setpoint } => ActuatorValue::DPR {
                enabled: true,
                setpoint,
            },
            DPRValve::Disabled => ActuatorValue::DPR {
                enabled: false,
                setpoint: f32::NAN,
            },
        };
        DatapointValue::Actuator(actuator)
    }
}

impl IntoDatapointValue for ThrottleableValve {
    const KIND: DatapointKind = DatapointKind::Actuator(ActuatorKind::Fractional);

    fn into_value(self) -> DatapointValue {
        DatapointValue::Actuator(ActuatorValue::Fractional {
            fraction: f32::from(self),
        })
    }
}

impl IntoDatapointValue for BarG {
    // FIXME: Once we have an `Unit` trait, we should be able to set this to `BarG::UNIT`
    // to prevent duplication of unit names.
    const KIND: DatapointKind = DatapointKind::Sensor(SensorKind::Analog { unit: "barg" });

    fn into_value(self) -> DatapointValue {
        DatapointValue::Sensor(SensorValue::Analog { value: self.0 })
    }
}

impl IntoDatapointValue for Celsius {
    // FIXME: Once we have an `Unit` trait, we should be able to set this to `Celsius::UNIT`
    // to prevent duplication of unit names.
    const KIND: DatapointKind = DatapointKind::Sensor(SensorKind::Analog { unit: "°C" });

    fn into_value(self) -> DatapointValue {
        DatapointValue::Sensor(SensorValue::Analog { value: self.0 })
    }
}

#[derive(facet::Facet)]
#[repr(u8)]
#[facet(tag = "message")]
enum Message {
    #[facet(datapoint_information::container)]
    Valves(ValvesContainer),
}

#[derive(facet::Facet)]
#[facet(pod, rename = "my-type")] // this would be the 
struct ValvesContainer {
    #[facet(datapoint_information::sensor_id = "lox-vent")]
    lox_vent: bool,
}

#[test]
fn bar() {
    use facet::{EnumType, Facet, StructType, Type, UserType};

    eprintln!("{}", ValvesContainer::SHAPE.effective_name());

    let Type::User(UserType::Enum(EnumType { variants, .. })) = Message::SHAPE.ty else {
        panic!("expected enum");
    };

    let Type::User(UserType::Struct(StructType { fields, .. })) =
        variants[0].data.fields[0].shape().ty
    else {
        panic!("expected struct");
    };

    let field = &fields[0];

    let &sens_id = field
        .get_attr(Some("datapoint"), "sensor_id")
        .expect("attribute should exist")
        .get_as::<&'static str>()
        .expect("attribute should decode");
    assert_eq!(sens_id, "lox-vent");
}
