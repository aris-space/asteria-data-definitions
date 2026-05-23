#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "derive_utoipa", allow(clippy::large_stack_arrays))] // allow slightly weird schema encoding

use data_core::Message;
#[cfg(feature = "can")]
use data_core::can;
#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "pss")]
use data_core::pss;
use datatypes::{
    actuator::{DigitalOut, InvertedDigitalOut, NormallyClosedValve, NormallyOpenValve},
    sensor::DigitalIn,
    status::{
        BuildInformationCommon, ConfigHash, DownlinkTelemetryStats, ManualActuatorChange,
        ManualStateTransition, StateMachineState, UplinkTelemetryHealth,
    },
    units::{BarG, Celsius},
};

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
#[schema(as = RfsComputerMessage)]
pub enum Message {
    // ======================== TX ========================
    /// Software build metadata periodically emitted by the RFS Computer.
    ///
    /// # CAN
    /// - **Kind**: Debug message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x611)]
    #[mavlink(id = 0x10_06_00)]
    BuildInfo(BuildInformationCommon),

    /// # `RfsComputerConfigHash`
    /// First 5 hex characters of the config file Git Object Hash currently
    /// loaded by the Rfs Computer.
    ///
    /// ## Type
    /// - `Periodic @ 0.2 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `RfsComputer`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Allows Rfs Computer config file tracking.
    ///
    /// ## Reaction
    /// Receivers may log or display the config hash.
    #[can(id = 0x630)]
    #[mavlink(id = 0x10_06_01)]
    ConfigHash(ConfigHash),

    /// # `RfsComputerStatus`
    /// Periodic status message emitted by the RFS Computer.
    ///
    /// Contains runtime health of the RFS Computer.
    /// It is also used to only ingite if the RFS is armed and ensure exclusive PSS control.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x311)]
    #[mavlink(id = 0x10_06_02)]
    Status(RfsComputerStatus),

    /// The current active state of the normally off spark plug.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x331)]
    #[pss(label = "Spark Plug", name = "ign_spk_plg")]
    IgniterSparkPlugState(DigitalOut),

    /// The current active state of the normally closed GOX igniter valve.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x332)]
    #[pss(label = "GOX Igniter", name = "gox_ign_vlv")]
    GoxIgniterValveState(NormallyClosedValve),

    /// The current active state of the normally closed H2 igniter valve.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x333)]
    #[pss(label = "H2 Igniter", name = "h2_ign_vlv")]
    H2IgniterValveState(NormallyClosedValve),

    /// The current active state of the normally closed GOX purge valve.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x334)]
    #[pss(label = "GOX Purge", name = "gox_prg_vlv")]
    GoxPurgeValveState(NormallyClosedValve),

    /// The current active state of the normally closed H2 purge valve.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x335)]
    #[pss(label = "H2 Purge", name = "h2_prg_vlv")]
    H2PurgeValveState(NormallyClosedValve),

    /// The current active state of the normally open N2 bleed valve.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x336)]
    #[pss(label = "N2 Bleed", name = "n2_bld_vlv")]
    N2BleedValveState(NormallyOpenValve),

    /// The current active state of the normally closed LOX purge valve.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x337)]
    #[pss(label = "LOX Purge", name = "lox_prg_vlv")]
    LoxPurgeValveState(NormallyClosedValve),

    /// The current active state of the normally open GOX bleed valve.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x338)]
    #[pss(label = "GOX Bleed", name = "gox_bld_vlv")]
    GoxBleedValveState(NormallyOpenValve),

    /// The current active state of the normally closed N2 purge valve.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x339)]
    #[pss(label = "N2 Purge", name = "n2_prg_vlv")]
    N2PurgeValveState(NormallyClosedValve),

    /// The current active state of the normally closed N2 fill valve.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x340)]
    #[pss(label = "N2 Fill", name = "n2_fll_vlv")]
    N2FillValveState(NormallyClosedValve),

    /// The current active state of the normally open H2 bleed valve.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x341)]
    #[pss(label = "H2 Bleed", name = "h2_bld_vlv")]
    H2BleedValveState(NormallyOpenValve),

    /// The current active state of the normally closed LOX bleed valve.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x342)]
    #[pss(label = "LOX Bleed", name = "lox_bld_vlv")]
    LoxBleedValveState(NormallyClosedValve),

    /// The current active state of the normally closed pneumatic booster valve.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x343)]
    #[pss(label = "Booster", name = "pnm_bstr_vlv")]
    BoosterValveState(NormallyClosedValve),

    /// The current active state of the normally closed LOX decoupler extend valve.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x344)]
    #[pss(label = "LOX Dec. Extend", name = "pnm_loxdec_ext")]
    LoxDecouplerExtendValveState(NormallyClosedValve),

    /// The current active state of the normally closed LOX decoupler retract valve.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x345)]
    #[pss(label = "LOX Dec. Retract", name = "pnm_loxdec_ret")]
    LoxDecouplerRetractValveState(NormallyClosedValve),

    /// The current active state of the normally closed N2 decoupler extend valve.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x346)]
    #[pss(label = "N2 Dec. Extend", name = "pnm_n2dec_ext")]
    N2DecouplerExtendValveState(NormallyClosedValve),

    /// The current active state of the normally closed N2 decoupler retract valve.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x347)]
    #[pss(label = "N2 Dec. Retract", name = "pnm_n2dec_ret")]
    N2DecouplerRetractValveState(NormallyClosedValve),

    /// The current active state of the normally closed N2 release valve.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x348)]
    #[pss(label = "N2 Release", name = "pnm_n2rls_vlv")]
    N2ReleaseValveState(NormallyClosedValve),

    /// The current active state of the rocket power output.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x349)]
    #[pss(label = "Rocket Power", name = "rocket_power")]
    RocketPowerState(InvertedDigitalOut),

    /// The current active state of the Maxon board power output.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x350)]
    #[pss(label = "Maxon Power", name = "maxon_board_power")]
    MaxonBoardPowerState(InvertedDigitalOut),

    /// The current active state of the FSS heating output.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x351)]
    #[pss(label = "FSS Heating", name = "fss_heating")]
    FssHeatingState(DigitalOut),

    /// The current active state of the pressure sensor heating output.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x352)]
    #[pss(label = "Sensor Heat", name = "pres_sensor_heat")]
    PressureSensorHeatingState(DigitalOut),

    /// The current active state of the ETH pump power output.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x353)]
    #[pss(label = "ETH Pump Power", name = "eth_pump_power")]
    EthPumpPowerState(DigitalOut),

    /// The current active state of the custom output 1.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x354)]
    #[pss(label = "Custom 1", name = "custom1")]
    Custom1State(DigitalOut),

    /// The current active state of the custom output 2.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x355)]
    #[pss(label = "Custom 2", name = "custom2")]
    Custom2State(DigitalOut),

    /// The current active state of the custom output 3.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x356)]
    #[pss(label = "Custom 3", name = "custom3")]
    Custom3State(DigitalOut),

    // ======================== Sensors ========================
    /// The temperature of the LOx line, from sensor `R-T-L-1`.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x357)]
    #[pss(label = "LOx Temperature", name = "r_t_l_1")]
    #[unit("°C")]
    LoxTemperature(Celsius),

    /// The temperature of the igniter, from sensor `I-T-I-1`.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x357)]
    #[pss(label = "Igniter Temperature", name = "i_t_i_1")]
    #[unit("°C")]
    IgniterTemperature(Celsius),

    /// The temperature inside the DACS cabinet.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x358)]
    #[pss(label = "Cabinet Temperature", name = "cabinet_t")]
    #[unit("°C")]
    CabinetTemperature(Celsius),

    /// The ambient temperature.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x359)]
    #[pss(label = "Ambient Temperature", name = "ambient_t")]
    #[unit("°C")]
    AmbientTemperature(Celsius),

    /// Whether the LOx decoupler is in its retracted state, from the proximity sensor.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x360)]
    #[pss(label = "LOx Decoupler Proximity", name = "lox_decoupler_prox")]
    LoxDecouplerProximity(DigitalIn),

    /// Whether the N2 decoupler is in its retracted state, from the proximity sensor.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x361)]
    #[pss(label = "N2 Decoupler Proximity", name = "n2_decoupler_prox")]
    N2DecouplerProximity(DigitalIn),

    /// Whether the valves are receiving power (arming relay state).
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x362)]
    #[pss(label = "Valves Armed", name = "valves_armed")]
    ValvesArmed(DigitalIn),

    /// Whether the spark plug is receiving power (arming relay state).
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x363)]
    #[pss(label = "Igniter Armed", name = "igniter_armed")]
    IgniterArmed(DigitalIn),

    /// The pressure from the igniter, from sensor ign_block_p
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x364)]
    #[pss(label = "Igniter Pressure", name = "pres_ign")]
    IgniterPressure(BarG),

    // ## Purpose
    // Communicates the actuator state of the actuators in the Remote Filling Station controllable via CAN.
    // This includes the ingiter valves and the spark plug.
    // ## Reaction
    // Receivers may log the data or use it for diagnostics and system health checks.
    // #[can(id = 0x325)]
    // #[mavlink(id = 0x10_06_03)]
    // ValveStates(ValveStates), not needed right? Single valves messages afaik
    /// The current state of the rfs computer state machine.
    #[mavlink(id = 0x10_06_04)]
    StateMachineState(StateMachineState),

    /// Statistics about the health of the telemetry uplink received by the Rfs Computer.
    ///
    /// Emitted periodically by the rfs computer.
    /// Can be used to calculate the throughput and loss rate.
    #[mavlink(id = 0x10_06_05)]
    UplinkTelemetryHealth(UplinkTelemetryHealth),

    /// Statistics about the telemetry downlink sent by the Rfs Computer.
    ///
    /// Emitted periodically by the Rfs computer.
    /// Can be used to calculate the throughput and loss rate.
    #[mavlink(id = 0x10_06_06)]
    DownlinkTelemetryStats(DownlinkTelemetryStats),

    /// Pressure of the N2 pre-booster line (`R-P-N-1`).
    #[can(id = 0x38a)]
    N2PreBoosterPressure(BarG),

    /// Pressure of the H2 purge line (`R-P-N-2`).
    #[can(id = 0x38b)]
    H2PurgePressure(BarG),

    /// Pressure of the N2 post-booster line (`R-P-N-3`).
    #[can(id = 0x38c)]
    N2PostBoosterPressure(BarG),

    /// Pressure of the LOX line (`R-P-L-1`).
    #[can(id = 0x38d)]
    LoxPressure(BarG),

    /// Pressure of the purge line (`R-P-P-1`).
    #[can(id = 0x38e)]
    PurgePressure(BarG),

    /// Pressure of the pneumatic line (`R-P-P-2`).
    #[can(id = 0x38f)]
    PneumaticPressure(BarG),

    /// Pressure of the GOX igniter line (`I-P-G-1`).
    #[can(id = 0x390)]
    GoxPressure(BarG),

    /// Pressure of the H2 igniter line (`I-P-H-1`).
    #[can(id = 0x391)]
    H2Pressure(BarG),

    // ======================== RX ========================
    /// Controls the GOX igniter valve on the RFS from the FC.
    ///
    /// This is needed during the ignition sequence where the flight computer controls the igniter
    /// and thus needs to open and close valves.
    ///
    /// # CAN
    /// - **Kind**: Event message
    /// - **Sent by**: Flight Computer
    #[can(id = 0x20a)]
    GoxIgniterValveControl(NormallyClosedValve),

    /// Controls the H2 igniter valve on the RFS from the FC.
    ///
    /// This is needed during the ignition sequence where the flight computer controls the igniter
    /// and thus needs to open and close valves.
    ///
    /// # CAN
    /// - **Kind**: Event message
    /// - **Sent by**: Flight Computer
    #[can(id = 0x20b)]
    #[mavlink(id = 0x10_06_07)]
    H2IgniterValveControl(NormallyClosedValve),

    /// Controls the igniter spark plug on the RFS from the FC.
    ///
    /// This is needed during the ignition sequence where the flight computer controls the igniter
    /// and thus needs to activate the spark plug.
    ///
    /// # CAN
    /// - **Kind**: Event message
    /// - **Sent by**: Flight Computer
    #[can(id = 0x20c)]
    #[mavlink(id = 0x10_06_08)]
    IgniterSparkPlugControl(DigitalOut),

    /// Controls the H2 purge valve on the RFS from the FC.
    ///
    /// This is needed during the ignition sequence where the flight computer controls the ignition
    /// and thus needs to purge.
    ///
    /// # CAN
    /// - **Kind**: Event message
    /// - **Sent by**: Flight Computer
    #[can(id = 0x20d)]
    #[mavlink(id = 0x10_06_09)]
    H2IgniterPurgeControl(NormallyClosedValve),

    /// Controls the GOX purge valve on the RFS from the FC.
    ///
    /// This is needed during the ignition sequence where the flight computer controls the ignition
    /// and thus needs to purge.
    ///
    /// # CAN
    /// - **Kind**: Event message
    /// - **Sent by**: Flight Computer
    #[can(id = 0x20e)]
    #[mavlink(id = 0x10_06_10)]
    GoxIgniterPurgeControl(NormallyClosedValve),

    /// Command a manual state transition on the RFSC.
    /// The state machine will perform the state transition and emit a state change event.
    #[mavlink(id = 0x20_06_00)]
    ManualStateTransition(ManualStateTransition),

    /// Command a specific actuator to move to a specific target position.
    /// The actuator name and the semantics of the value are defined using
    /// an `#[pss(name = "...", kind  = "...")]` attribute from the [`data_core::pss::Data`]
    /// macro.
    ///
    /// Upon change, an "actuator states" message will be emitted.
    #[mavlink(id = 0x20_06_01)]
    ManualActuatorChange(ManualActuatorChange),
}
