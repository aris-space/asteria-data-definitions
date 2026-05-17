#![cfg_attr(not(feature = "std"), no_std)]

use data_core::Message;
#[cfg(feature = "can")]
use data_core::can;
#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "pss")]
use data_core::pss;
use datatypes::actuator::NormallyClosedValve;
use datatypes::status::BuildInformationCommon;
use datatypes::units::{BarG, Celsius};
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
#[schema(as = EngineControlBoardMessage)]
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
    #[mavlink(id = 0x10_08_00)]
    BuildInfo(BuildInformationCommon),

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
    #[mavlink(id = 0x10_08_01)]
    BoardStatus(EngineControlBoardStatus),

    /// ## Type
    /// - `Periodic @ 5 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the Fuel Main Valve State.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x324)]
    #[mavlink(id = 0x10_08_02)]
    #[pss(label = "Fuel Main Valve", name = "fss_ml_vlv")]
    FuelMainValveState(NormallyClosedValve),

    /// ## Type
    /// - `Periodic @ 5 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `EngineControlBoard`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the Oxidizer Main Valve State.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x329)]
    #[mavlink(id = 0x10_08_05)]
    #[pss(label = "Oxidizer Main Valve", name = "oss_ml_vlv")]
    OxidizerMainValveState(NormallyClosedValve),

    /// # `EngineChamberPressure`
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
    /// Reports the Combustion Chamber Pressure.
    ///
    /// ## Reaction
    /// No action required.
    ///
    /// ## Data Origin
    /// Data originates from analog Trafag pressure sensors installed on the engine. They range from 0 to 60 barg
    /// The value will be `NaN` if reading the sensor fails.
    #[can(id = 0x462)]
    #[mavlink(id = 0x10_08_03)]
    #[pss(name = "eng_cc_p", label = "Chamber Pressure Sensor")]
    ChamberPressure(BarG),

    /// # `InjectorFuelPressure`
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
    /// Reports the Injector Fuel Pressure.
    ///
    /// ## Reaction
    /// No action required.
    ///
    /// ## Data Origin
    /// Data originates from analog Trafag pressure sensors installed on the engine. They range from 0 to 60 barg
    /// The value will be `NaN` if reading the sensor fails.
    #[can(id = 0x463)]
    #[mavlink(id = 0x10_08_06)]
    #[pss(name = "eng_inj_p", label = "Injector Fuel Pressure Sensor")]
    InjectorFuelPressure(BarG),

    /// # `InjectorOxidizerPressure`
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
    /// Reports the Injector Oxidizer Pressure.
    ///
    /// ## Reaction
    /// No action required.
    ///
    /// ## Data Origin
    /// Data originates from analog Trafag pressure sensors installed on the engine. They range from 0 to 60 barg
    /// The value will be `NaN` if reading the sensor fails.
    #[can(id = 0x464)]
    #[mavlink(id = 0x10_08_07)]
    #[pss(name = "oss_inj_p", label = "Injector Oxidizer Pressure Sensor")]
    InjectorOxidizerPressure(BarG),

    /// # `InjectorFuelTemperature`
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
    /// Reports the Injector Fuel Temperature.
    ///
    /// ## Reaction
    /// No action required.
    ///
    /// ## Data Origin
    /// Data originates from high precision ADCs which read the differential thermocouple voltage together with a thermistor from which the temperature is calculated.
    /// Thermocouples installed can be either Type K or Type T.
    /// If the value read in firmware is outside the specific temperature range of [-200.0, 200.0] °C,
    /// the value will be set to `NaN`.
    #[can(id = 0x465)]
    #[mavlink(id = 0x10_08_04)]
    #[pss(name = "eng_inj_t", label = "Injector Fuel Temperature Sensor")]
    InjectorFuelTemperature(Celsius),

    /// # `OxidizerTankTemperature`
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
    /// Reports the Oxidizer Tank Bottom Temperature.
    ///
    /// ## Reaction
    /// No action required.
    ///
    /// ## Data Origin
    /// Data originates from high precision ADCs which read the differential thermocouple voltage together with a thermistor from which the temperature is calculated.
    /// Thermocouples installed can be either Type K or Type T.
    /// If the value read in firmware is outside the specific temperature range of [-200.0, 200.0] °C,
    /// the value will be set to `NaN`.
    #[can(id = 0x466)]
    #[mavlink(id = 0x10_08_08)]
    #[pss(name = "oss_tnk_t", label = "Oxidizer Tank Temperature Sensor")]
    OxidizerTankTemperature(Celsius),

    // ======================== RX (cont.) ========================
    // The following 2 sets of messages have the same meaning, but are separate messages
    // sent from different systems such they can never collide.
    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `EngineControlBoard`
    ///
    /// ## Purpose
    /// Sets the Oxidizer Main valve to a certain state.
    ///
    /// ## Reaction
    /// The Engine Control Board shall change the Oxidizer Main Valve position accordingly
    #[can(id = 0x225)]
    OxidizerMainValveControlFC(NormallyClosedValve),

    /// ## Type
    /// - `Event`: sent by the RFS
    ///
    /// ## Direction
    /// - **Emitted by:** `RemoteFillingStation`
    /// - **Received by:** `EngineControlBoard`
    ///
    /// ## Purpose
    /// Sets the Oxidizer Main valve to a certain state.
    ///
    /// ## Reaction
    /// The Engine Control Board shall change the Oxidizer Main Valve position accordingly
    #[can(id = 0x227)]
    OxidizerMainValveControlRFS(NormallyClosedValve),

    /// ## Type
    /// - `Event`: sent by the FC
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** `EngineControlBoard`
    ///
    /// ## Purpose
    /// Sets the Fuel Main valve to a certain state.
    ///
    /// ## Reaction
    /// The Engine Control Board shall change the Fuel Main Valve position accordingly.
    #[can(id = 0x226)]
    FuelMainValveControlFC(NormallyClosedValve),

    /// ## Type
    /// - `Event`: sent by the RFS
    ///
    /// ## Direction
    /// - **Emitted by:** `RemoteFillingStation`
    /// - **Received by:** `EngineControlBoard`
    ///
    /// ## Purpose
    /// Sets the Fuel Main valve to a certain state.
    ///
    /// ## Reaction
    /// The Engine Control Board shall change the Fuel Main Valve position accordingly.
    #[can(id = 0x228)]
    FuelMainValveControlRFS(NormallyClosedValve),
}
