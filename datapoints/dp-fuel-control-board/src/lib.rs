#![cfg_attr(not(feature = "std"), no_std)]

use data_core::Message;
#[cfg(feature = "can")]
use data_core::can;
#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "pss")]
use data_core::pss;
use datatypes::actuator::{DPRValve, NormallyOpenValve};
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
#[schema(as = FueldControlBoardMessage)]
pub enum Message {
    // ======================== TX ========================
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
    #[mavlink(id = 0x10_02_00)]
    BuildInfo(BuildInformationCommon),

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
    #[mavlink(id = 0x10_02_01)]
    BoardStatus(FuelControlBoardStatus),

    /// ## Type
    /// - `Periodic @ 20 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `FuelControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Reports the pressure of the Pressurization Line (COPV).
    ///
    #[can(id = 0x450)]
    #[mavlink(id = 0x10_02_02)]
    #[pss(name = "prz_copv_p", label = "Pressurization Pressure Sensor")]
    PressurizationLinePressure(BarG),

    /// ## Type
    /// - `Periodic @ 20 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `FuelControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Reports Fuel Tank Pressures.
    /// These are mainly used as an input to the Control Loop running on the Fuel Control Board.
    ///
    #[can(id = 0x452)]
    #[mavlink(id = 0x10_02_03)]
    #[pss(flatten)]
    FuelTankPressure(FuelTankPressure),

    /// ## Type
    /// - `Periodic @ 5 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `FuelControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the DPR Valve State including setpoint.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x322)]
    #[mavlink(id = 0x10_02_04)]
    #[pss(label = "Fuel DPR Valve", name = "prz_fss_dpr")]
    FuelDprValveState(DPRValve),

    /// ## Type
    /// - `Periodic @ 5 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `FuelControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the Pressurization Valve State
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x326)]
    #[mavlink(id = 0x10_02_05)]
    #[pss(label = "Pressurization Vent Valve", name = "prz_copv_vnt")]
    PressurizationVentValveState(NormallyOpenValve),

    /// ## Type
    /// - `Periodic @ 5 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `FuelControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the Fuel Vent Valve State
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x327)]
    #[mavlink(id = 0x10_02_06)]
    #[pss(label = "Fuel Vent Valve", name = "fss_tnk_vnt")]
    FuelVentValveState(NormallyOpenValve),

    // ======================== RX ========================
    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `FuelControlBoard`
    ///
    /// ## Purpose
    /// Set ETH Pressure Regulator configuration
    ///
    /// ## Reaction
    /// The Fuel Control Board shall adjust the FUE DPR setpoint and enable/disable the controller accordingly
    #[can(id = 0x211)]
    FuelDprValveControlFC(DPRValve),

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
    PressurizationVentValveControlFC(NormallyOpenValve),

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
    FuelVentValveControlFC(NormallyOpenValve),

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
    FuelDprValveControlRFS(DPRValve),

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
    PressurizationVentValveControlRFS(NormallyOpenValve),

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
    FuelVentValveControlRFS(NormallyOpenValve),
}
