use data_core::Message;
use datatypes::actuator::{DprState, PressurizationGain};
use datatypes::status::{BuildInformationCommon, ValveState};
use datatypes::units::BarG;
use gated_derive::gated_derive;

mod dptypes;
pub use dptypes::*;

#[derive(Message)]
#[gated_derive(data_core::CanMessage(feature = "can"))]
pub enum Message {
    // ======================== TX ========================
    /// # `FuelControlBoardBuildInfo`
    /// Firmware build metadata emitted by the Fuel Control Board.
    ///
    /// ## Type
    /// - `Periodic @ 0.2 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `FuelControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Allows version tracking and verification of the Fuel Control Board firmware.
    ///
    /// ## Reaction
    /// Receivers may log or display the firmware version and build metadata.
    #[can(id = 0x617)]
    FuelControlBoardBuildInfo(BuildInformationCommon),

    /// # `FuelControlBoardStatus`
    /// Periodic status message emitted by the Fuel Control Board.
    ///
    /// ## Type
    /// - `Periodic @ 1 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `FuelControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the runtime health of the Fuel Control Board, including sensor.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x320)]
    FuelControlBoardStatusMsg(FuelControlBoardStatus),

    /// # `FuelControlBoardValveStates`
    /// Periodic status message emitted by the Fuel Control Board.
    ///
    /// ## Type
    /// - `Periodic @ 5 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `FuelControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the Valve State of all Valves controlled by the Fuel Control Board, including the DPR control loop state and setpoint.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x322)]
    FuelControlBoardValveStatesMsg(FuelControlBoardValveStates),

    /// # `FuelPressurizationCompleted`
    /// Event message sent by the FuelControlBoard.
    ///
    /// ## Type
    /// - `Event`: sent by the FCB to signal completed pressurization on the FuelControlBoard
    ///
    /// ## Direction
    /// - **Emitted by:** `FuelControlBoard`
    /// - **Received by:** `FlightComputer`
    ///
    /// ## Purpose
    /// Signal completion of Fuel Tank pressurization
    ///
    /// ## Reaction
    ///
    #[can(id = 0x12b)]
    FuelPressurizationCompleted,

    // ======================== TX ========================
    /// # `FuelPressurization`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC to initiate pressurization on the FuelControlBoard
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `FuelControlBoard`
    ///
    /// ## Purpose
    /// Pressurize the Fuel Tank to a desired setpoint
    ///
    /// ## Reaction
    ///
    #[can(id = 0x125)]
    FuelPressurization(BarG),

    /// # `FuelPressurizationGain`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC to configure the pressurization gain on the FuelControlBoard
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `FuelControlBoard`
    ///
    /// ## Purpose
    /// Configure the Fuel Pressurization Gain
    ///
    /// ## Reaction
    ///
    #[can(id = 0x127)]
    FuelPressurizationGain(PressurizationGain),

    /// # `FuelPressurizationAbort`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC to abort pressurization on the FuelControlBoard
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `FuelControlBoard`
    ///
    /// ## Purpose
    /// Abort Fuel Tank pressurization
    ///
    /// ## Reaction
    ///
    #[can(id = 0x129)]
    FuelPressurizationAbort,

    /// # `FuelDprConfigFC`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `FuelControlBoard`
    ///
    /// ## Purpose
    /// Set ETH Pressure Regulator configuration
    /// Configures whether the control loop should be active, and what reference point (setpoint) it should assume.
    ///
    /// ## Reaction
    /// The Fuel Control Board shall adjust the FUE DPR setpoint and enable/disable the controller accordingly
    #[can(id = 0x211)]
    FuelDprConfigFC(DprState),

    /// # `PressurizationVentControlFC`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `FuelControlBoard`
    ///
    /// ## Purpose
    /// Sets the N2 Vent Valve to a certain state.
    ///
    /// ## Reaction
    /// The Fuel Control Board shall adjust N2 Vent Valve state accordingly
    #[can(id = 0x212)]
    PressurizationVentControlFC(ValveState),

    /// # `FuelVentControlFC`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `FuelControlBoard`
    ///
    /// ## Purpose
    /// Sets the Fuel Vent Valve to a certain state.
    ///
    /// ## Reaction
    /// The Fuel Control Board shall adjust the Fuel Vent Valve state accordingly
    #[can(id = 0x213)]
    FuelVentControlFC(ValveState),

    /// # `FuelDprConfigECU`
    /// Event message sent by the ECU.
    ///
    /// ## Type
    /// - `Event`: sent by the ECU
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `FuelControlBoard`
    ///
    /// ## Purpose
    /// Set ETH Pressure Regulator configuration
    /// Configures whether the control loop should be active, and what reference point (setpoint) it should assume.
    ///
    /// ## Reaction
    /// The Fuel Control Board shall adjust the FUE DPR setpoint and enable/disable the controller accordingly
    #[can(id = 0x216)]
    FuelDprConfigECU(DprState),

    /// # `PressurizationVentControlECU`
    /// Event message sent by the ECU.
    ///
    /// ## Type
    /// - `Event`: sent by the ECU
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `FuelControlBoard`
    ///
    /// ## Purpose
    /// Sets the N2 Vent Valve to a certain state.
    ///
    /// ## Reaction
    /// The Fuel Control Board shall adjust N2 Vent Valve state accordingly
    #[can(id = 0x217)]
    PressurizationVentControlECU(ValveState),

    /// # `FuelVentControlECU`
    /// Event message sent by the ECU.
    ///
    /// ## Type
    /// - `Event`: sent by the ECU
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `FuelControlBoard`
    ///
    /// ## Purpose
    /// Sets the Fuel Vent Valve to a certain state.
    ///
    /// ## Reaction
    /// The Fuel Control Board shall adjust the Fuel Vent Valve state accordingly
    #[can(id = 0x218)]
    FuelVentControlECU(ValveState),

    /// # `FuelDprConfigRFS`
    /// Event message sent by the RFS.
    ///
    /// ## Type
    /// - `Event`: sent by the RFS
    ///
    /// ## Direction
    /// - **Emitted by:** `RemoteFillingStation`
    /// - **Received by:** `FuelControlBoard`
    ///
    /// ## Purpose
    /// Set ETH Pressure Regulator configuration
    ///
    /// ## Reaction
    /// The Fuel Control Board shall adjust the FUE DPR setpoint and enable/disable the controller accordingly
    #[can(id = 0x21b)]
    FuelDprConfigRFS(DprState),

    /// # `PressurizationVentControlRFS`
    /// Event message sent by the RFS.
    ///
    /// ## Type
    /// - `Event`: sent by the RFS
    ///
    /// ## Direction
    /// - **Emitted by:** `RemoteFillingStation`
    /// - **Received by:** `FuelControlBoard`
    ///
    /// ## Purpose
    /// Sets the N2 Vent Valve to a certain state.
    ///
    /// ## Reaction
    /// The Fuel Control Board shall adjust N2 Vent Valve state accordingly
    #[can(id = 0x21c)]
    PressurizationVentControlRFS(ValveState),

    /// # `FuelVentControlRFS`
    /// Event message sent by the RFS.
    ///
    /// ## Type
    /// - `Event`: sent by the RFS
    ///
    /// ## Direction
    /// - **Emitted by:** `RemoteFillingStation`
    /// - **Received by:** `FuelControlBoard`
    ///
    /// ## Purpose
    /// Sets the Fuel Vent Valve to a certain state.
    ///
    /// ## Reaction
    /// The Fuel Control Board shall adjust the Fuel Vent Valve state accordingly
    #[can(id = 0x21d)]
    FuelVentControlRFS(ValveState),

    // ========= NOT SURE  =============
    /// # `FuelMainControlFC`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC to control the Fuel Main Valve
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `EngineControlBoard`
    ///
    /// ## Purpose
    /// Sets the Fuel Main valve to be active or inactive.
    ///
    /// ## Reaction
    /// The Engine Control Board shall change the Fuel Main Valve position accordingly.
    /// ## Note
    /// The Engine Control Board might also change the valve position without input from this message, during the firing sequence
    /// /// The Valve is Normally Closed
    #[can(id = 0x226)]
    FuelMainControlFC(ValveState),

    /// # `FuelMainControlRFS`
    /// Event message sent by the RFS.
    ///
    /// ## Type
    /// - `Event`: sent by the RFS to control the Fuel Main Valve
    ///
    /// ## Direction
    /// - **Emitted by:** `RemoteFillingStation`
    /// - **Received by:** `EngineControlBoard`
    ///
    /// ## Purpose
    /// Sets the Fuel Main valve to be active or inactive.
    ///
    /// ## Reaction
    /// The Engine Control Board shall change the Fuel Main Valve position accordingly.
    /// ## Note
    /// The Engine Control Board might also change the valve position without input from this message, during the firing sequence
    /// /// The Valve is Normally Closed
    #[can(id = 0x228)]
    FuelMainControlRFS(ValveState),
}
