use data_core::{CanMessage, Message};
use datatypes::{BuildInformationCommon, ValveState};
mod dptypes;
pub use dptypes::*;

#[derive(Message, CanMessage)]
pub enum Message {
    // ======================== TX ========================
    /// # `EngineControlBoardBuildInfo`
    /// Firmware build metadata emitted by the Engine Control Board.
    ///
    /// ## Type
    /// - `Periodic @ 0.2 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Allows version tracking and verification of the Engine Control Board firmware.
    ///
    /// ## Reaction
    /// Receivers may log or display the firmware version and build metadata.
    #[can(id = 0x614)]
    EngineControlBoardBuildInfo(BuildInformationCommon),

    /// # `EngineControlBoardStatus`
    /// Periodic status message emitted by the Engine Control Board.
    ///
    /// ## Type
    /// - `Periodic @ 1 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the runtime health of the Engine Control Board.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x314)]
    EngineControlBoardStatusMsg(EngineControlBoardStatus),

    /// # `EngineControlBoardValveStates`
    /// Periodic status message emitted by the Engine Control Board.
    ///
    /// ## Type
    /// - `Periodic @ 5 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the Valve State of all Valves controlled by the Engine Control Board.
    /// This includes the Oxidizer Main and Fuel Main Valve.
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x324)]
    EngineControlBoardValveStatesMsg(EngineControlBoardValveStates),

    /// # `EnginePressure`
    /// Periodic telemetry message emitted by the Engine Control Board.
    ///
    /// ## Type
    /// - `Periodic @ 20 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Reports various Pressures from the Engine.
    /// This includes the Combustion Chamber Pressure, Injector Fuel Pressure and Injector Oxidizer Pressure.
    ///
    /// ## Reaction
    /// No action required.
    ///
    /// ## Data Origin
    /// Data originates from analog Trafag pressure sensors installed on the engine. They range from 0 to 60 barg
    /// The value will be `NaN` if reading the sensor fails.
    #[can(id = 0x460)]
    EnginePressureMsg(EnginePressure),

    /// # `EngineBayTemperature`
    /// Periodic telemetry message emitted by the Engine Control Board.
    ///
    /// ## Type
    /// - `Periodic @ 10 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Reports various Data from Thermocouples in the Engine Bay.
    /// This includes the OSS Runline Temperature and the OSS Tank Bottom Temperature.
    ///
    /// ## Reaction
    /// No action required.
    ///
    /// ## Data Origin
    /// Data originates from high precision ADCs which read the differential thermocouple voltage together with a thermistor from which the temperature is calculated.
    /// Thermocouples installed can be either Type K or Type T.
    /// If the value read in firmware is outside the specific temperature range of [-200.0, 200.0] °C,
    /// the value will be set to `NaN`.
    #[can(id = 0x461)]
    EngineBayTemperatureMsg(EngineBayTemperature),

    /// # `FiringAborted`
    /// Event message sent by the ECU.
    ///
    /// ## Type
    /// - `Event`: sent by the ECU to signalize an abort has occurred during the firing sequence
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `FlightComputer`
    ///
    /// ## Purpose
    /// Signaling Firing aborts
    ///
    /// ## Reaction
    ///
    #[can(id = 0x121)]
    FiringAborted,

    /// # `FiringCompleted`
    /// Event message sent by the ECU.
    ///
    /// ## Type
    /// - `Event`: sent by the ECU to signalize the firing sequence has successfully been completed
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Signaling completion of firing sequence
    ///
    /// ## Reaction
    ///
    #[can(id = 0x123)]
    FiringCompleted,

    /// # `FiringInitiated`
    /// Event message sent by the EngineControlBoard.
    ///
    /// ## Type
    /// - `Event`: sent by the ECU to signal firing initiation
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Signal initiation of firing sequence
    ///
    /// ## Reaction
    ///
    #[can(id = 0x130)]
    FiringInitiated,

    /// # `IgnitionDetected`
    /// Event message sent by the EngineControlBoard.
    ///
    /// ## Type
    /// - `Event`: sent by the ECU to signal successful igniter ignition
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Signal successful igniter ignition
    ///
    /// ## Reaction
    ///
    #[can(id = 0x131)]
    IgnitionDetected,

    /// # `CombustionDetected`
    /// Event message sent by the EngineControlBoard.
    ///
    /// ## Type
    /// - `Event`: sent by the ECU to signal successful engine combustion
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Signal successful engine combustion
    ///
    /// ## Reaction
    ///
    #[can(id = 0x132)]
    CombustionDetected,

    // ======================== RX ========================
    /// # `ThrustCurveConfig`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC configure the Launch Thrust Curve
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `EngineControlBoard`
    ///
    /// ## Purpose
    /// Transmit the flight thrust curve to the Engine Control Board.
    ///
    /// ## Reaction
    ///
    #[can(id = 0x240)]
    ThrustCurveConfigMsg(ThrustCurveConfig),

    /// # `FiringAbortInitiation`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC to initialize a firing abort
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `EngineControlBoard`
    ///datapoints/dp-flight-computer/src
    /// ## Purpose
    /// Initiate abortion of firing sequence
    ///
    /// ## Reaction
    /// The ECU should properly abort the firing
    #[can(id = 0x120)]
    FiringAbortInitiation,

    /// # `FiringInitiation`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC to initiate firing sequence
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `EngineControlBoard`
    ///
    /// ## Purpose
    /// Initiate firing sequence
    ///
    /// ## Reaction
    /// The Engine Control Board shall initiate the firing sequence upon receiving this command
    #[can(id = 0x122)]
    FiringInitiation,

    /// # `OxidizerMainControlFC`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC to control the Oxidizer Main Valve
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `EngineControlBoard`
    ///
    /// ## Purpose
    /// Sets the Oxidizer Main valve to be active or inactive.
    ///
    /// ## Reaction
    /// The Engine Control Board shall change the Oxidizer Main Valve position accordingly
    /// ## Note
    /// The Engine Control Board might also change the valve position without input from this message, during the firing sequence
    /// The Valve is Normally Closed
    #[can(id = 0x225)]
    OxidizerMainControlFC(ValveState),

    /// # `OxidizerMainControlRFS`
    /// Event message sent by the RFS.
    ///
    /// ## Type
    /// - `Event`: sent by the RFS to control the Oxidizer Main Valve
    ///
    /// ## Direction
    /// - **Emitted by:** `RemoteFillingStation`
    /// - **Received by:** `EngineControlBoard`
    ///
    /// ## Purpose
    /// Sets the Oxidizer Main valve to be active or inactive.
    ///
    /// ## Reaction
    /// The Engine Control Board shall change the Oxidizer Main Valve position accordingly
    /// ## Note
    /// The Engine Control Board might also change the valve position without input from this message, during the firing sequence
    /// /// The Valve is Normally Closed
    #[can(id = 0x227)]
    OxidizerMainControlRFS(ValveState),
}
