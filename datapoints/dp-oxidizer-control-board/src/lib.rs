#![cfg_attr(not(feature = "std"), no_std)]

use data_core::Message;
use datatypes::actuator::{DprState, PressurizationGain};
use datatypes::status::{BuildInformationCommon, ValveState};
use datatypes::units::BarG;
use gated_derive::gated_derive;

mod dptypes;
pub use dptypes::*;

#[derive(Message)]
#[gated_derive(data_core::CanMessage(feature = "can"), Debug)]
pub enum Message {
    // ======================== TX ========================
    /// # `OxidizerControlBoardBuildInfo`
    /// Firmware build metadata emitted by the Oxidizer Control Board.
    ///
    /// ## Type
    /// - `Periodic @ 0.2 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `OxidizerControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Allows version tracking and verification of the Oxidizer Control Board firmware.
    ///
    /// ## Reaction
    /// Receivers may log or display the firmware version and build metadata.
    #[can(id = 0x618)]
    BuildInfo(BuildInformationCommon),

    /// # `OxidizerControlBoardStatus`
    /// Periodic status message emitted by the Oxidizer Control Board.
    ///
    /// ## Type
    /// - `Periodic @ 1 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `OxidizerControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the runtime health of the Oxidizer Control Board, including sensor.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x321)]
    BoardStatus(OxidizerControlBoardStatus),

    /// # `OxidizerControlBoardValveStates`
    /// Periodic status message emitted by the Oxidizer Control Board.
    ///
    /// ## Type
    /// - `Periodic @ 5 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `OxidizerControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the Valve State of all Valves controlled by the Oxidizer Control Board, including the DPR control loop state and setpoint.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x323)]
    ValveStates(OxidizerControlBoardValveStates),

    /// # `OxidizerPressurizationCompleted`
    /// Event message sent by the OxidizerControlBoard
    ///
    /// ## Type
    /// - `Event`: sent by the OCB to signal completed pressurization on the OxidizerControlBoard
    ///
    /// ## Direction
    /// - **Emitted by:** `OxidizerControlBoard`
    /// - **Received by:** `FlightComputer`
    ///
    /// ## Purpose
    /// Signal completion of Oxidizer Tank pressurization
    ///
    /// ## Reaction
    ///
    #[can(id = 0x12a)]
    OxidizerPressurizationCompleted,

    // ======================== RX ========================
    /// # `OxidizerPressurization`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC to initiate pressurization on the OxidizerControlBoard
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `OxidizerControlBoard`
    ///
    /// ## Purpose
    /// Pressurize the Oxidizer Tank to a desired setpoint
    ///
    /// ## Reaction
    ///
    #[can(id = 0x124)]
    OxidizerPressurization(BarG),

    /// # `OxidizerPressurizationGain`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC to configure the pressurization gain on the OxidizerControlBoard
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `OxidizerControlBoard`
    ///
    /// ## Purpose
    /// Configure the Oxidizer Pressurization Gain
    ///
    /// ## Reaction
    ///
    #[can(id = 0x126)]
    OxidizerPressurizationGain(PressurizationGain),

    /// # `OxidizerPressurizationAbort`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC to abort pressurization on the OxidizerControlBoard
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `OxidizerControlBoard`
    ///
    /// ## Purpose
    /// Abort Oxidizer Tank pressurization
    ///
    /// ## Reaction
    ///
    #[can(id = 0x128)]
    OxidizerPressurizationAbort,

    // ======================== RX (cont.) ========================
    // The following 3 sets of messages have the same meaning, but are separate messages
    // sent from different systems such they can never collide.
    /// # `OxidizerDprConfigFC`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `OxidizerControlBoard`
    ///
    /// ## Purpose
    /// Set OXD Pressure Regulator configuration
    /// Configures whether the control loop should be active, and what reference point (setpoint) it should assume.
    ///
    /// ## Reaction
    /// The Oxidizer Control Board shall adjust the OXD DPR setpoint and enable/disable the controller accordingly
    #[can(id = 0x210)]
    OxidizerDprConfigFC(DprState),

    /// # `OxidizerVentControlFC`
    /// Event message sent by the FC.
    ///
    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `OxidizerControlBoard`
    ///
    /// ## Purpose
    /// Sets the Oxidizer Vent Valve to a certain state.
    ///
    /// ## Reaction
    /// The Oxidizer Control Board shall adjust the Oxidizer Vent Valve state accordingly
    #[can(id = 0x214)]
    OxidizerVentControlFC(ValveState),

    /// # `OxidizerDprConfigECU`
    /// Event message sent by the ECU.
    ///
    /// ## Type
    /// - `Event`: sent by the ECU
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `OxidizerControlBoard`
    ///
    /// ## Purpose
    /// Set OXD Pressure Regulator configuration
    /// Configures whether the control loop should be active, and what reference point (setpoint) it should assume.
    ///
    /// ## Reaction
    /// The Oxidizer Control Board shall adjust the OXD DPR setpoint and enable/disable the controller accordingly
    #[can(id = 0x215)]
    OxidizerDprConfigECU(DprState),

    /// # `OxidizerVentControlECU`
    /// Event message sent by the ECU.
    ///
    /// ## Type
    /// - `Event`: sent by the ECU
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `OxidizerControlBoard`
    ///
    /// ## Purpose
    /// Sets the Oxidizer Vent Valve to a certain state.
    ///
    /// ## Reaction
    /// The Oxidizer Control Board shall adjust the Oxidizer Vent Valve state accordingly
    #[can(id = 0x219)]
    OxidizerVentControlECU(ValveState),

    /// # `OxidizerDprConfigRFS`
    /// Event message sent by the RFS.
    ///
    /// ## Type
    /// - `Event`: sent by the RFS
    ///
    /// ## Direction
    /// - **Emitted by:** `RemoteFillingStation`
    /// - **Received by:** `OxidizerControlBoard`
    ///
    /// ## Purpose
    /// Set OXD Pressure Regulator configuration
    ///
    /// ## Reaction
    /// The Oxidizer Control Board shall adjust the OXD DPR setpoint and enable/disable the controller accordingly
    #[can(id = 0x21a)]
    OxidizerDprConfigRFS(DprState),

    /// # `OxidizerVentControlRFS`
    /// Event message sent by the RFS.
    ///
    /// ## Type
    /// - `Event`: sent by the RFS
    ///
    /// ## Direction
    /// - **Emitted by:** `RemoteFillingStation`
    /// - **Received by:** `OxidizerControlBoard`
    ///
    /// ## Purpose
    /// Sets the Oxidizer Vent Valve to a certain state.
    ///
    /// ## Reaction
    /// The Oxidizer Control Board shall adjust the Oxidizer Vent Valve state accordingly
    #[can(id = 0x21e)]
    OxidizerVentControlRFS(ValveState),
}
