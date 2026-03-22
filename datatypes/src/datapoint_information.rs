use core::f32;

use datapoint_information::datapoint::{ActuatorValue, DatapointValue, SensorValue};

use crate::{
    actuator::{
        DPRValve, DigitalOut, InvertedDigitalOut, NormallyClosedValve, NormallyOpenValve,
        ThrottleableValve,
    },
    sensor::DigitalIn,
    status::SensorStatus,
    units::Celsius,
};

impl From<NormallyClosedValve> for DatapointValue {
    fn from(value: NormallyClosedValve) -> Self {
        Self::Actuator(ActuatorValue::Digital {
            active: matches!(value, NormallyClosedValve::Open),
        })
    }
}

impl From<NormallyOpenValve> for DatapointValue {
    fn from(value: NormallyOpenValve) -> Self {
        Self::Actuator(ActuatorValue::Digital {
            active: matches!(value, NormallyOpenValve::Open),
        })
    }
}

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
        })
    }
}

impl From<DPRValve> for DatapointValue {
    fn from(value: DPRValve) -> Self {
        let actuator = match value {
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

impl From<ThrottleableValve> for DatapointValue {
    fn from(value: ThrottleableValve) -> Self {
        Self::Actuator(ActuatorValue::Fractional {
            precent: f32::from(value.openness_percent),
        })
    }
}

impl From<SensorStatus> for DatapointValue {
    fn from(value: SensorStatus) -> Self {
        Self::Sensor(SensorValue::Digital {
            enabled: matches!(value, SensorStatus::Online),
        })
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
