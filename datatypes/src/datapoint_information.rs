use core::f32;

use datapoint_information::datapoint::{ActuatorValue, DatapointValue, SensorValue};

use crate::{
    actuator::{DPRValve, NormallyClosedValve, NormallyOpenValve, ThrottleableValve},
    status::{SensorStatus, ValveState},
    units::{BarG, Celsius},
};

impl From<NormallyClosedValve> for DatapointValue {
    fn from(value: NormallyClosedValve) -> Self {
        Self::Actuator(ActuatorValue::Digital {
            enabled: matches!(value, NormallyClosedValve::Open),
        })
    }
}

impl From<NormallyOpenValve> for DatapointValue {
    fn from(value: NormallyOpenValve) -> Self {
        Self::Actuator(ActuatorValue::Digital {
            enabled: matches!(value, NormallyOpenValve::Open),
        })
    }
}

impl From<ValveState> for DatapointValue {
    fn from(value: ValveState) -> Self {
        Self::Actuator(ActuatorValue::Digital {
            enabled: matches!(value, ValveState::Active),
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
            percent: f32::from(value.openness_percent),
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

impl From<BarG> for DatapointValue {
    fn from(value: BarG) -> Self {
        Self::Sensor(SensorValue::Analog { value: value.0 })
    }
}

impl From<Celsius> for DatapointValue {
    fn from(value: Celsius) -> Self {
        Self::Sensor(SensorValue::Analog { value: value.0 })
    }
}
