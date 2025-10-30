pub mod board_status;
pub mod debug_info;
pub mod event_messages;
mod reserved;
pub mod sensor_data;
pub mod system_management;

mod board_id;

pub use board_id::*;

use crate::define_messages;

// note: messages should be defined in the correct message group and
// in ascending ID order.
define_messages!(Message {
    // System Management Messages
    ResetAll(system_management::ResetAll),
    ResetSpecific(system_management::ResetSpecific),
    UTCTimeUpdate(system_management::UTCTimeUpdate),

    // Event Messages
    #[cfg(feature = "message-engine-firing-abort-initiation")]
    FiringAbortInitiation(event_messages::FiringAbortInitiation),
    #[cfg(feature = "message-engine-firing-aborted")]
    FiringAborted(event_messages::FiringAborted),
    #[cfg(feature = "message-engine-firing-initiation")]
    FiringInitiation(event_messages::FiringInitiation),
    #[cfg(feature = "message-engine-firing-completed")]
    FiringCompleted(event_messages::FiringCompleted),

    #[cfg(feature = "message-engine-firing-initiated")]
    FiringInitiated(event_messages::FiringInitiated),
    #[cfg(feature = "message-engine-ignition-detected")]
    IgnitionDetected(event_messages::IgnitionDetected),
    #[cfg(feature = "message-engine-combustion-detected")]
    CombustionDetected(event_messages::CombustionDetected),

    #[cfg(feature = "message-fuel-pressurization")]
    FuelPressurization(event_messages::FuelPressurization),
    #[cfg(feature = "message-oxidizer-pressurization")]
    OxidizerPressurization(event_messages::OxidizerPressurization),
    #[cfg(feature = "message-fuel-pressurization-gain")]
    FuelPressurizationGain(event_messages::FuelPressurizationGain),
    #[cfg(feature = "message-oxidizer-pressurization-gain")]
    OxidizerPressurizationGain(event_messages::OxidizerPressurizationGain),
    #[cfg(feature = "message-fuel-pressurization-abort")]
    FuelPressurizationAbort(event_messages::FuelPressurizationAbort),
    #[cfg(feature = "message-oxidizer-pressurization-abort")]
    OxidizerPressurizationAbort(event_messages::OxidizerPressurizationAbort),
    #[cfg(feature = "message-fuel-pressurization-completed")]
    FuelPressurizationComplete(event_messages::FuelPressurizationCompleted),
    #[cfg(feature = "message-oxidizer-pressurization-completed")]
    OxidizerPressurizationComplete(event_messages::OxidizerPressurizationCompleted),
    #[cfg(feature = "message-separation-trigger")]
    SeparationTrigger(event_messages::SeparationTrigger),
    #[cfg(feature = "message-deployment-trigger")]
    DeploymentTrigger(event_messages::DeploymentTrigger),
    #[cfg(feature = "message-separation-occurred")]
    SeparationOccurred(event_messages::SeparationOccurred),
    #[cfg(feature = "message-deployment-occurred")]
    DeploymentOccurred(event_messages::DeploymentOccurred),
    #[cfg(feature = "message-steering-target-positions")]
    SteeringTargetPositions(event_messages::SteeringTargetPositions),
    #[cfg(feature = "message-steering-actual-positions")]
    SteeringActualPositions(event_messages::SteeringActualPositions),
    #[cfg(feature = "message-recovery-power-config")]
    RecoveryPowerConfig(event_messages::RecoveryPowerConfig),
    #[cfg(feature = "message-camera-power-control")]
    CameraPowerControl(event_messages::CameraPowerControl),
    #[cfg(feature = "message-gox-igniter-valve-control")]
    GoxIgniterValveControl(event_messages::GoxIgniterValveControl),
    #[cfg(feature = "message-h2-igniter-valve-control")]
    H2IgniterValveControl(event_messages::H2IgniterValveControl),
    #[cfg(feature = "message-igniter-spark-plug-control")]
    IgniterSparkPlugControl(event_messages::IgniterSparkPlugControl),
    #[cfg(feature = "message-igniter-purge-control")]
    GoxPurgeValveControl(event_messages::IgniterPurgeControl),
    #[cfg(feature= "message-oxidizer-dpr-config-fc")]
    OxidizerDprConfigFC(event_messages::OxidizerDprConfigFC),
    #[cfg(feature= "message-fuel-dpr-config-fc")]
    FuelDprConfigFC(event_messages::FuelDprConfigFC),
    #[cfg(feature= "message-pressure-vent-control-fc")]
    PressurizationVentControlFC(event_messages::PressurizationVentControlFC),
    #[cfg(feature= "message-fuel-vent-control-fc")]
    FuelVentControlFC(event_messages::FuelVentControlFC),
    #[cfg(feature= "message-oxidizer-vent-control-fc")]
    OxidizerVentControlFC(event_messages::OxidizerVentControlFC),
    #[cfg(feature= "message-oxidizer-dpr-config-ecu")]
    OxidizerDprConfigECU(event_messages::OxidizerDprConfigECU),
    #[cfg(feature= "message-fuel-dpr-config-ecu")]
    FuelDprConfigECU(event_messages::FuelDprConfigECU),
    #[cfg(feature= "message-pressure-vent-control-ecu")]
    PressurizationVentControlECU(event_messages::PressurizationVentControlECU),
    #[cfg(feature= "message-fuel-vent-control-ecu")]
    FuelVentControlECU(event_messages::FuelVentControlECU),
    #[cfg(feature= "message-oxidizer-vent-control-ecu")]
    OxidizerVentControlECU(event_messages::OxidizerVentControlECU),
    #[cfg(feature= "message-oxidizer-dpr-config-rfs")]
    OxidizerDprConfigRFS(event_messages::OxidizerDprConfigRFS),
    #[cfg(feature= "message-fuel-dpr-config-rfs")]
    FuelDprConfigRFS(event_messages::FuelDprConfigRFS),
    #[cfg(feature= "message-pressure-vent-control-rfs")]
    PressurizationVentControlRFS(event_messages::PressurizationVentControlRFS),
    #[cfg(feature= "message-fuel-vent-control-rfs")]
    FuelVentControlRFS(event_messages::FuelVentControlRFS),
    #[cfg(feature= "message-oxidizer-vent-control-rfs")]
    OxidizerVentControlRFS(event_messages::OxidizerVentControlRFS),
    #[cfg(feature = "message-oxidizer-main-control-fc")]
    OxidizerMainControlFC(event_messages::OxidizerMainControlFC),
    #[cfg(feature = "message-fuel-main-control-fc")]
    FuelMainControlFC(event_messages::FuelMainControlFC),
    #[cfg(feature = "message-oxidizer-main-control-rfs")]
    OxidizerMainControlRFS(event_messages::OxidizerMainControlRFS),
    #[cfg(feature = "message-fuel-main-control-rfs")]
    FuelMainControlRFS(event_messages::FuelMainControlRFS),
    #[cfg(feature = "message-engine-thrust-config")]
    ThrustCurveConfig(event_messages::ThrustCurveConfig),

    // Status Messages
    #[cfg(feature = "message-flight-computer-status")]
    FlightComputerStatus(board_status::FlightComputerStatus),
    #[cfg(feature = "message-rfs-computer-status")]
    RfsComputerStatus(board_status::RfsComputerStatus),
    #[cfg(feature = "message-sensor-carrier-status")]
    SensorCarrierStatus(board_status::SensorCarrierStatus),
    #[cfg(feature = "message-communication-board-status")]
    CommunicationBoardStatus(board_status::CommunicationBoardStatus),
    #[cfg(feature = "message-engine-control-board-status")]
    EngineControlBoardStatus(board_status::EngineControlBoardStatus),
    #[cfg(feature = "message-power-board-status")]
    PowerBoardStatus(board_status::PowerBoardStatus),
    #[cfg(feature = "message-recovery-board-status")]
    RecoveryBoardStatus(board_status::RecoveryBoardStatus),
    #[cfg(feature = "message-fuel-control-board-status")]
    FuelControlBoardStatus(board_status::FuelControlBoardStatus),
    #[cfg(feature = "message-oxidizer-control-board-status")]
    OxidizerControlBoardStatus(board_status::OxidizerControlBoardStatus),

    #[cfg(feature = "message-fuel-control-board-valve-states")]
    FuelControlBoardValveStates(board_status::FuelControlBoardValveStates),
    #[cfg(feature = "message-oxidizer-control-board-valve-states")]
    OxidizerControlBoardValveStates(board_status::OxidizerControlBoardValveStates),
    #[cfg(feature = "message-engine-control-board-valve-states")]
    EngineControlBoardValveStates(board_status::EngineControlBoardValveStates),
    #[cfg(feature = "message-rfs-computer-valve-states")]
    RfsComputerValveStates(board_status::RfsComputerValveStates),
    #[cfg(feature = "message-camera-power-status")]
    CameraPowerStatus(board_status::CameraPowerStatus),

    // Sensor Data
    #[cfg(feature = "message-position-data")]
    PositionData(sensor_data::PositionData),
    #[cfg(feature = "message-imu-data")]
    ImuData(sensor_data::ImuData),
    #[cfg(feature = "message-velocity-data")]
    VelocityData(sensor_data::VelocityData),
    #[cfg(feature = "message-orientation-data")]
    OrientationData(sensor_data::OrientationData),
    #[cfg(feature = "message-rail-status-5v")]
    RailStatus5V(sensor_data::RailStatus5V),
    #[cfg(feature = "message-rail-status-24v")]
    RailStatus24V(sensor_data::RailStatus24V),
    #[cfg(feature = "message-environmental-data")]
    EnvironmentalData(sensor_data::EnvironmentalData),
    #[cfg(feature = "message-pressure-data")]
    PressureData(sensor_data::PressureData),
    #[cfg(feature = "message-magnetometer-data")]
    MagnetometerData(sensor_data::MagnetometerData),
    #[cfg(feature = "message-engine-pressure-data")]
    EnginePressure(sensor_data::EnginePressure),
    #[cfg(feature = "message-engine-temperature-data")]
    EngineBayTemperature(sensor_data::EngineBayTemperature),

    #[cfg(feature = "message-igniter-pressure")]
    IgniterPressure(sensor_data::IgniterPressure),
    #[cfg(feature = "message-igniter-temperature")]
    IgniterTemperature(sensor_data::IgniterTemperature),

    #[cfg(feature = "message-pressurization-line-pressure")]
    PressurizationLinePressure(sensor_data::PressurizationLinePressure),
    #[cfg(feature = "message-fuel-tank-pressure")]
    FuelTankPressure(sensor_data::FuelTankPressure),
    #[cfg(feature = "message-oxidizer-tank-pressure")]
    OxidizerTankPressure(sensor_data::OxidizerTankPressure),
    #[cfg(feature = "message-fuel-tank-temperature")]
    FuelTankTemperature(sensor_data::FuelTankTemperature),
    #[cfg(feature = "message-oxidizer-tank-level")]
    OxidizerTankLevel(sensor_data::OxidizerTankLevel),

    // Debug Info Messages
    #[cfg(feature = "message-flight-computer-build-info")]
    FlightComputerBuildInfo(debug_info::FlightComputerBuildInfo),
    #[cfg(feature = "message-rfs-computer-build-info")]
    RfsComputerBuildInfo(debug_info::RfsComputerBuildInfo),
    #[cfg(feature = "message-sensor-carrier-build-info")]
    SensorCarrierBuildInfo(debug_info::SensorCarrierBuildInfo),
    #[cfg(feature = "message-communication-board-build-info")]
    CommunicationBoardBuildInfo(debug_info::CommunicationBoardBuildInfo),
    #[cfg(feature = "message-engine-control-board-build-info")]
    EngineControlBoardBuildInfo(debug_info::EngineControlBoardBuildInfo),
    #[cfg(feature = "message-power-board-build-info")]
    PowerBoardBuildInfo(debug_info::PowerBoardBuildInfo),
    #[cfg(feature = "message-recovery-board-build-info")]
    RecoveryBoardBuildInfo(debug_info::RecoveryBoardBuildInfo),
    #[cfg(feature = "message-fuel-control-board-build-info")]
    FuelControlBoardBuildInfo(debug_info::FuelControlBoardBuildInfo),
    #[cfg(feature = "message-oxidizer-control-board-build-info")]
    OxidizerControlBoardBuildInfo(debug_info::OxidizerControlBoardBuildInfo),
});
