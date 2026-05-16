use core::f32;

use datapoint_information::datapoint::{
    ActuatorKind, ActuatorValue, DatapointKind, DatapointValue, IntoDatapointValue, SensorKind,
    SensorValue,
};

use crate::{
<<<<<<< HEAD
    actuator::{
        DPRValve, DigitalOut, InvertedDigitalOut, NormallyClosedValve, NormallyOpenValve,
        ThrottleableValve,
    },
    sensor::DigitalIn,
    status::SensorStatus,
    units::Celsius,
=======
    actuator::{DPRValve, NormallyClosedValve, NormallyOpenValve, ThrottleableValve},
    status::ValveState,
    units::{BarG, Celsius},
>>>>>>> main
};

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

<<<<<<< HEAD
impl From<DigitalOut> for DatapointValue {
    fn from(value: DigitalOut) -> Self {
        Self::Actuator(ActuatorValue::Digital {
            active: matches!(value, DigitalOut::On),
        })
    }
}

impl From<InvertedDigitalOut> for DatapointValue {
    fn from(value: InvertedDigitalOut) -> Self {
        Self::Actuator(ActuatorValue::Digital {
            active: matches!(value, InvertedDigitalOut::On),
=======
impl IntoDatapointValue for ValveState {
    const KIND: DatapointKind = DatapointKind::Actuator(ActuatorKind::Digital {
        default_enabled: false,
    });

    fn into_value(self) -> DatapointValue {
        DatapointValue::Actuator(ActuatorValue::Digital {
            enabled: matches!(self, ValveState::Active),
>>>>>>> main
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
            percent: f32::from(self.openness_percent),
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

impl From<Celsius> for DatapointValue {
    fn from(Celsius(value): Celsius) -> Self {
        Self::Sensor(SensorValue::Analog { value })
    }
}

impl From<DigitalIn> for DatapointValue {
    fn from(value: DigitalIn) -> Self {
        Self::Sensor(SensorValue::Digital {
            enabled: matches!(value, DigitalIn::On),
        })
    }
}
