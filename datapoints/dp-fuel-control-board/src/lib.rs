#![cfg_attr(not(feature = "std"), no_std)]

use data_core::Message;
#[cfg(feature = "can")]
use data_core::can;
#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "derive_pss")]
use data_core::pss;
use datatypes::actuator::{NormallyClosedValve, NormallyOpenValve};
use datatypes::status::BuildInformationCommon;
use gated_derive::gated_derive;

mod dptypes;
pub use dptypes::*;

mod version;
pub use version::{VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH};

#[derive(Message)]
#[gated_derive(can::Message, Debug)]
#[gated_derive(mavlink::Message, pss::Data)]
#[gated_derive(utoipa::ToSchema, serde::Deserialize, serde::Serialize)]
#[gated_derive(pss::Data)]
#[serde(tag = "type", content = "content")]
#[schema(as = FueldControlBoardMessage)]
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
    #[mavlink(id = 0x10_02_00)]
    BuildInfo(BuildInformationCommon),

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
    #[mavlink(id = 0x10_02_01)]
    BoardStatus(FuelControlBoardStatus),

    /// # `FuelDprValveState`
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
    /// Communicates the Fuel DPR control loop state and setpoint.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x322)]
    #[mavlink(id = 0x10_02_02)]
    #[pss(label = "Fuel DPR Valve", name = "prz_fss_dpr")]
    FuelDprValveState(NormallyClosedValve),

    /// # `PressurizationVentValveState`
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
    /// Communicates the state of the Pressurization Vent Valve.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x323)]
    #[mavlink(id = 0x10_02_05)]
    #[pss(label = "Pressurization Vent Valve", name = "prz_copv_vnt")]
    PressurizationVentValveState(NormallyOpenValve),

    /// # `FuelVentValveState`
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
    /// Communicates the state of the Fuel Vent Valve.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x324)]
    #[mavlink(id = 0x10_02_06)]
    #[pss(label = "Fuel Vent Valve", name = "fss_tnk_vnt")]
    FuelVentValveState(NormallyOpenValve),

    /// # `PressurizationLinePressure`
    /// Periodic telemetry message emitted by the Fuel Control Board.
    ///
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
    /// ## Reaction
    /// No action required.
    #[can(id = 0x450)]
    #[mavlink(id = 0x10_02_03)]
    PressurizationLinePressure(PressurizationLinePressure),

    /// # `FuelTankPressure`
    /// Periodic telemetry message emitted by the Fuel Control Board.
    ///
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
    /// ## Reaction
    /// No action required.
    #[can(id = 0x452)]
    #[mavlink(id = 0x10_02_04)]
    FuelTankPressure(FuelTankPressure),
}
