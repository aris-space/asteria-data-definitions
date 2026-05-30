#![cfg_attr(not(feature = "std"), no_std)]

use data_core::Message;
#[cfg(feature = "can")]
use data_core::can;
#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "pss")]
use data_core::pss;
use datatypes::actuator::{DPRValve, NormallyClosedValve, NormallyOpenValve};
use datatypes::status::BuildInformationCommon;
use datatypes::units::BarG;
use gated_derive::gated_derive;

mod dptypes;
pub use dptypes::*;

mod version;
pub use version::{VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH};

#[derive(Message, Clone)]
#[gated_derive(can::Message, Debug, defmt::Format)]
#[gated_derive(mavlink::Message)]
#[gated_derive(utoipa::ToSchema, serde::Deserialize, serde::Serialize)]
#[gated_derive(pss::Data)]
#[serde(tag = "type", content = "content")]
#[schema(as = OxidizerControlBoardMessage)]
pub enum Message {
    // ======================== TX ========================
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
    #[mavlink(id = 0x10_03_00)]
    BuildInfo(BuildInformationCommon),

    /// ## Type
    /// - `Periodic @ 1 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `OxidizerControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the runtime health of the Oxidizer Control Board, including sensor status.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x321)]
    #[mavlink(id = 0x10_03_01)]
    BoardStatus(OxidizerControlBoardStatus),

    /// ## Type
    /// - `Periodic @ 20 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `OxidizerControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Reports Oxidizer Tank Pressure Sensor 1.
    /// This is mainly used as an input to the Control Loop running on the Oxidizer Control Board.
    ///
    #[can(id = 0x455)]
    #[mavlink(id = 0x10_03_02)]
    #[pss(name = "oss_tnk_p1", label = "Oxidizer Tank Pressure Sensor 1")]
    OxidizerTankPressureSensor1(BarG),

    /// ## Type
    /// - `Periodic @ 20 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `OxidizerControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Reports Oxidizer Tank Pressure Sensor 2.
    /// This is mainly used as an input to the Control Loop running on the Oxidizer Control Board.
    ///
    #[can(id = 0x456)]
    #[mavlink(id = 0x10_03_05)]
    #[pss(name = "oss_tnk_p2", label = "Oxidizer Tank Pressure Sensor 2")]
    OxidizerTankPressureSensor2(BarG),

    /// ## Type
    /// - `Periodic @ 20 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `OxidizerControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Reports the absolute valued Oxidizer Tank differential pressure.
    ///
    #[can(id = 0x457)]
    #[mavlink(id = 0x10_03_06)]
    #[pss(name = "oss_tnk_dps", label = "Oxidizer Differential Pressure Sensor")]
    OxidizerTankDifferentialPressure(BarG),

    /// ## Type
    /// - `Periodic @ 5 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `OxidizerControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the DPR Valve State including setpoint.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x323)]
    #[mavlink(id = 0x10_03_03)]
    #[pss(label = "Oxidizer DPR Valve", name = "prz_oss_dpr")]
    OxidizerDprValveState(DPRValve),

    /// ## Type
    /// - `Periodic @ 5 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `OxidizerControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the Oxidizer Vent Valve State.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x328)]
    #[mavlink(id = 0x10_03_04)]
    #[pss(label = "Oxidizer Vent Valve", name = "oss_tnk_vnt")]
    OxidizerVentValveState(NormallyOpenValve),

    // ======================== RX ========================
    // The following 2 sets of messages have the same meaning, but are separate messages
    // sent from different systems such they can never collide.
    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `OxidizerControlBoard`
    ///
    /// ## Purpose
    /// Set OXD Pressure Regulator configuration
    ///
    /// ## Reaction
    /// The Oxidizer Control Board shall adjust the OXD DPR setpoint and enable/disable the controller accordingly
    #[can(id = 0x210)]
    OxidizerDprValveControlFC(DPRValve),

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
    OxidizerVentValveControlFC(NormallyOpenValve),

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
    OxidizerDprValveControlRFS(DPRValve),

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
    OxidizerVentValveControlRFS(NormallyOpenValve),

    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `OxidizerControlBoard`
    ///
    /// ## Purpose
    /// Sets the proportional Oxidizer DPR Gain
    ///
    /// ## Reaction
    /// The Oxidizer Control Board shall adjust the proportional DPR gain accordingly
    #[can(id = 0x22a)]
    OxidizerDprGainP(f32),

    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `OxidizerControlBoard`
    ///
    /// ## Purpose
    /// Sets the integral Oxidizer DPR Gain
    ///
    /// ## Reaction
    /// The Oxidizer Control Board shall adjust the integral DPR gain accordingly
    #[can(id = 0x22b)]
    OxidizerDprGainI(f32),

    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `OxidizerControlBoard`
    ///
    /// ## Purpose
    /// Sets the derivative Oxidizer DPR Gain
    ///
    /// ## Reaction
    /// The Oxidizer Control Board shall adjust the derivative DPR gain accordingly
    #[can(id = 0x22c)]
    OxidizerDprGainD(f32),

    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `OxidizerControlBoard`
    ///
    /// ## Purpose
    /// Sets the minimum opening time ms of the DPR
    ///
    /// ## Reaction
    /// The Oxidizer Control Board shall adjust the derivative DPR minimum opening time accordingly
    #[can(id = 0x22d)]
    OxidizerDprMinOpeningTime(f32),

    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `OxidizerControlBoard`
    ///
    /// ## Purpose
    /// Sets the maximum opening time ms of the DPR
    ///
    /// ## Reaction
    /// The Oxidizer Control Board shall adjust the derivative DPR maximum opening time accordingly
    #[can(id = 0x22e)]
    OxidizerDprMaxOpeningTime(f32),
}
