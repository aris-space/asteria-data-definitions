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

    /// First 5 hex characters of the config file Git Object Hash currently
    /// loaded by the Rfs Computer.
    ///
    /// This is used to auto-select the state machine in the mission control
    #[mavlink(id = 0x10_06_01)]
    ConfigHash(ConfigHash),

    /// Periodic status message emitted by the RFS Computer.
    ///
    /// Contains runtime health of the RFS Computer.
    /// It is also used to only ingite if the RFS is armed.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x311)]
    #[mavlink(id = 0x10_06_02)]
    Status(RfsComputerStatus),

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

    /// The current active state of the normally closed N2 rocket purge valve.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x348)]
    #[pss(label = "N2 Rocket Purge", name = "pnm_rocprg")]
    N2RocketPurgeValveState(NormallyClosedValve),

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
    /// The temperature of the LOx line, from sensor `lox_bottle_t`.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x357)]
    #[pss(label = "LOx Temperature", name = "lox_bottle_t")]
    #[unit("°C")]
    LoxTemperature(Celsius),

    /// The temperature of the igniter, from sensor `ign_block_t`.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x357)]
    #[pss(label = "Igniter Temperature", name = "ign_block_t")]
    #[unit("°C")]
    IgniterTemperature(Celsius),

    /// The temperature inside the DACS cabinet.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x358)]
    #[pss(label = "Cabinet Temperature", name = "rfs_cabinent_t")]
    #[unit("°C")]
    CabinetTemperature(Celsius),

    /// The ambient temperature.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x359)]
    #[pss(label = "Ambient Temperature", name = "rfs_ambient_t")]
    #[unit("°C")]
    AmbientTemperature(Celsius),

    /// Whether the LOx decoupler is in its retracted state, from the proximity sensor.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x360)]
    #[pss(label = "LOx Decoupler Proximity", name = "pmn_loxdec_prox")]
    LoxDecouplerProximity(DigitalIn),

    /// Whether the N2 decoupler is in its retracted state, from the proximity sensor.
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x361)]
    #[pss(label = "N2 Decoupler Proximity", name = "pmn_n2dec_prox")]
    N2DecouplerProximity(DigitalIn),

    /// Whether the valves are receiving power (arming relay state).
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x362)]
    #[pss(label = "Valves Armed", name = "rfs_main_armed")]
    ValvesArmed(DigitalIn),

    /// Whether the spark plug is receiving power (arming relay state).
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x363)]
    #[pss(label = "Igniter Armed", name = "rfs_ign_armed")]
    IgniterArmed(DigitalIn),

    /// The pressure from the igniter block
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x364)]
    #[pss(label = "Igniter Pressure", name = "ign_block_p")]
    IgniterPressure(BarG),

    /// Pressure of the N2 pre-booster line.
    #[can(id = 0x38a)]
    #[pss(label = "N2 pre booster pressure", name = "n2_bottle_p")]
    N2PreBoosterPressure(BarG),

    /// Pressure of the H2 purge line.
    #[can(id = 0x38b)]
    #[pss(label = "H2 purge pressure", name = "h2_prg_p")]
    H2PurgePressure(BarG),

    /// Pressure of the N2 post-booster line.
    #[can(id = 0x38c)]
    #[pss(label = "N2 post booster pressure", name = "n2_fill_p")]
    N2PostBoosterPressure(BarG),

    /// Pressure of the LOX line.
    #[can(id = 0x38d)]
    #[pss(label = "LOx pressure", name = "lox_bottle_p")]
    LoxPressure(BarG),

    /// Pressure of the purge line.
    #[can(id = 0x38e)]
    #[pss(label = "purge pressure", name = "prg_line_p")]
    PurgePressure(BarG),

    /// Pressure of the pneumatic line.
    #[can(id = 0x38f)]
    #[pss(label = "pneumatic pressure", name = "pnm_line_p")]
    PneumaticPressure(BarG),

    /// Pressure of the GOX igniter line.
    #[can(id = 0x390)]
    #[pss(label = "GOx pressure", name = "gox_line_p")]
    GoxPressure(BarG),

    /// Pressure of the H2 igniter line.
    #[can(id = 0x391)]
    #[pss(label = "H2 pressure", name = "h2_line_p")]
    H2Pressure(BarG),

    // ======================== RX ========================
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
    GoxIgniterPurgeControl(NormallyClosedValve),
}
