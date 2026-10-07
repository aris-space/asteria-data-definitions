//! The RFS datapoints, see also the instrumentation naming excel in sharepoint.
//!
//! Only some of them have CAN support, as those are the only ones used by the FC sequences.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "derive_utoipa", allow(clippy::large_stack_arrays))] // allow slightly weird schema encoding

use data_core::Message;
#[cfg(feature = "can")]
use data_core::can;
#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "pss")]
use data_core::pss;
use datatypes::actuator::{
    DigitalOut, InvertedDigitalOut, NormallyClosedValve, NormallyOpenValve, ThrottleableValve,
};
use datatypes::sensor::DigitalIn;
use datatypes::status::{
    ArmingState, BuildInformationCommon, ConfigHash, DownlinkTelemetryStats, ManualActuatorChange,
    ManualStateTransition, StateMachineState, UplinkTelemetryHealth,
};
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
#[gated_derive(pss::Data, facet::Facet)]
#[repr(u8)]
#[serde(tag = "type", content = "content")]
#[schema(as = RfsComputerMessage)]
pub enum Message {
    // ======================== TX ========================

    // -------- Status / health --------
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
    /// It is also used to only ignite if the RFS is armed.
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

    // -------- Valve / actuator states --------
    /// The current active state of the normally off spark plug.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x331)]
    #[mavlink(id = 0x10_06_10)]
    #[pss(label = "Spark Plug", name = "ign_spk_plg")]
    IgniterSparkPlugState(DigitalOut),

    /// The current active state of the normally closed GOX igniter valve.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x332)]
    #[mavlink(id = 0x10_06_11)]
    #[pss(label = "GOX Igniter", name = "gox_ign_vlv")]
    GoxIgniterValveState(NormallyClosedValve),

    /// The current active state of the normally closed H2 igniter valve.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x333)]
    #[mavlink(id = 0x10_06_12)]
    #[pss(label = "H2 Igniter", name = "h2_ign_vlv")]
    H2IgniterValveState(NormallyClosedValve),

    /// The current active state of the normally closed GOX purge valve.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x334)]
    #[mavlink(id = 0x10_06_13)]
    #[pss(label = "GOX Purge", name = "gox_prg_vlv")]
    GoxPurgeValveState(NormallyClosedValve),

    /// The current active state of the normally closed H2 purge valve.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x335)]
    #[mavlink(id = 0x10_06_14)]
    #[pss(label = "H2 Purge", name = "h2_prg_vlv")]
    H2PurgeValveState(NormallyClosedValve),

    /// The current active state of the normally open N2 bleed valve.
    #[mavlink(id = 0x10_06_15)]
    #[pss(label = "N2 Bleed", name = "n2_bld_vlv")]
    N2BleedValveState(NormallyOpenValve),

    /// The current active state of the normally closed LOX purge valve.
    #[mavlink(id = 0x10_06_16)]
    #[pss(label = "LOX Purge", name = "lox_prg_vlv")]
    LoxPurgeValveState(NormallyClosedValve),

    /// The current active state of the normally open GOX bleed valve.
    #[mavlink(id = 0x10_06_17)]
    #[pss(label = "GOX Bleed", name = "gox_bld_vlv")]
    GoxBleedValveState(NormallyOpenValve),

    /// The current active state of the normally closed N2 purge valve.
    #[mavlink(id = 0x10_06_18)]
    #[pss(label = "N2 Purge", name = "n2_prg_vlv")]
    N2PurgeValveState(NormallyClosedValve),

    /// The current active state of the normally closed N2 fill valve.
    #[mavlink(id = 0x10_06_19)]
    #[pss(label = "N2 Fill", name = "n2_fll_vlv")]
    N2FillValveState(NormallyClosedValve),

    /// The current active state of the normally open H2 bleed valve.
    #[mavlink(id = 0x10_06_1a)]
    #[pss(label = "H2 Bleed", name = "h2_bld_vlv")]
    H2BleedValveState(NormallyOpenValve),

    /// The current active state of the normally closed LOX bleed valve.
    #[mavlink(id = 0x10_06_1b)]
    #[pss(label = "LOX Bleed", name = "lox_bld_vlv")]
    LoxBleedValveState(NormallyClosedValve),

    /// The current active state of the normally closed pneumatic booster valve.
    #[mavlink(id = 0x10_06_1c)]
    #[pss(label = "Booster", name = "n2_bst_vlv")]
    BoosterValveState(NormallyClosedValve),

    /// The current active state of the normally closed LOX decoupler extend valve.
    #[mavlink(id = 0x10_06_1d)]
    #[pss(label = "LOX Dec. Extend", name = "lox_dec_ext")]
    LoxDecouplerExtendValveState(NormallyClosedValve),

    /// The current active state of the normally closed LOX decoupler retract valve.
    #[mavlink(id = 0x10_06_1e)]
    #[pss(label = "LOX Dec. Retract", name = "lox_dec_ret")]
    LoxDecouplerRetractValveState(NormallyClosedValve),

    /// The current active state of the normally closed N2 decoupler extend valve.
    #[mavlink(id = 0x10_06_1f)]
    #[pss(label = "N2 Dec. Extend", name = "n2_dec_ext")]
    N2DecouplerExtendValveState(NormallyClosedValve),

    /// The current active state of the normally closed N2 decoupler retract valve.
    #[mavlink(id = 0x10_06_20)]
    #[pss(label = "N2 Dec. Retract", name = "n2_dec_ret")]
    N2DecouplerRetractValveState(NormallyClosedValve),

    /// The current active state of the normally closed N2 shower valves.
    #[mavlink(id = 0x10_06_21)]
    #[pss(label = "N2 Showers", name = "n2_shw_vlv")]
    N2ShowerValvesState(NormallyClosedValve),

    /// The current active state of the normally closed engine purge valve valve.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x336)]
    #[mavlink(id = 0x10_06_22)]
    #[pss(label = "Engine Purge", name = "eng_prg_vlv")]
    N2RocketPurgeValveState(NormallyClosedValve),

    // -------- Digital power outputs --------
    /// The current active state of the rocket power output.
    #[mavlink(id = 0x10_06_30)]
    #[pss(label = "Rocket Power", name = "rocket_power")]
    RocketPowerState(InvertedDigitalOut),

    /// The current active state of the Maxon board power output.
    #[mavlink(id = 0x10_06_31)]
    #[pss(label = "Maxon Power", name = "maxon_board_power")]
    MaxonBoardPowerState(InvertedDigitalOut),

    /// The current active state of the rocket heating output.
    #[mavlink(id = 0x10_06_32)]
    #[pss(label = "Heating", name = "rocket_heating")]
    RocketHeatingState(DigitalOut),

    /// The current active state of the custom output 1.
    #[mavlink(id = 0x10_06_35)]
    #[pss(label = "Custom 1", name = "custom1")]
    Custom1State(DigitalOut),

    /// The current active state of the custom output 2.
    #[mavlink(id = 0x10_06_36)]
    #[pss(label = "Custom 2", name = "custom2")]
    Custom2State(DigitalOut),

    /// The current active state of the custom output 3.
    #[mavlink(id = 0x10_06_37)]
    #[pss(label = "Custom 3", name = "custom3")]
    Custom3State(DigitalOut),

    /// The current openness state of the LOX fill valve
    #[mavlink(id = 0x10_06_38)]
    #[pss(label = "LOx fill", name = "lox_fll_vlv")]
    LoxFillValveState(ThrottleableValve),

    // -------- Temperature sensors --------
    /// The temperature of the LOx line.
    #[mavlink(id = 0x10_06_40)]
    #[pss(label = "LOx T.", name = "lox_line_t")]
    #[unit("°C")]
    LoxTemperature(Celsius),

    /// The temperature of the igniter.
    ///
    /// # CAN
    /// - **Kind**: Sensor message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x365)]
    #[mavlink(id = 0x10_06_41)]
    #[pss(label = "Igniter T.", name = "ign_cc_t")]
    #[unit("°C")]
    IgniterTemperature(Celsius),

    /// The temperature inside the DACS cabinet.
    #[mavlink(id = 0x10_06_42)]
    #[pss(label = "Cabinet T.", name = "rfs_cabinent_t")]
    #[unit("°C")]
    CabinetTemperature(Celsius),

    /// The ambient temperature.
    #[mavlink(id = 0x10_06_43)]
    #[pss(label = "Ambient T.", name = "rfs_ambient_t")]
    #[unit("°C")]
    AmbientTemperature(Celsius),

    /// The temperature inside the SMART box.
    #[mavlink(id = 0x10_06_44)]
    #[pss(label = "SMART T.", name = "smart_cabinent_t")]
    #[unit("°C")]
    SmartBoxTemperature(Celsius),

    /// A unused thermocouple on the RFS.
    #[mavlink(id = 0x10_06_45)]
    #[pss(label = "Custom T.1", name = "custom_t1")]
    #[unit("°C")]
    CustomT1(Celsius),

    // -------- Digital sensors / arming --------
    /// Whether the LOx decoupler is in its retracted state, from the proximity sensor.
    #[mavlink(id = 0x10_06_50)]
    #[pss(label = "LOx Dec. Prox", name = "lox_dec_prox")]
    LoxDecouplerProximity(DigitalIn),

    /// Whether the N2 decoupler is in its retracted state, from the proximity sensor.
    #[mavlink(id = 0x10_06_51)]
    #[pss(label = "N2 Dec. Prox", name = "n2_dec_prox")]
    N2DecouplerProximity(DigitalIn),

    /// Whether the valves are receiving power (arming relay state).
    #[mavlink(id = 0x10_06_52)]
    #[pss(label = "Valves Armed", name = "rfs_main_armed")]
    ValvesArmed(DigitalIn),

    /// Whether the spark plug is receiving power (arming relay state).
    #[mavlink(id = 0x10_06_53)]
    #[pss(label = "Igniter Armed", name = "rfs_ign_armed")]
    IgniterArmed(DigitalIn),

    /// Whether the valves of the smart box are receiving power (arming relay state).
    #[mavlink(id = 0x10_06_54)]
    #[pss(label = "SMART Armed", name = "smart_valves_armed")]
    ValvesArmedSmartBox(ArmingState),

    /// Whether the rocket is given power and is connected.
    #[mavlink(id = 0x10_06_55)]
    #[pss(label = "Rocket Connected", name = "rocket_con_detect")]
    RocketConnected(DigitalIn),

    // -------- Pressure sensors --------
    /// The pressure from the igniter block.
    ///
    /// # CAN
    /// - **Kind**: Sensor message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x364)]
    #[mavlink(id = 0x10_06_60)]
    #[pss(label = "Igniter P.", name = "ign_cc_p")]
    IgniterPressure(BarG),

    /// Pressure of the N2 pre-booster line.
    #[mavlink(id = 0x10_06_61)]
    #[pss(label = "N2 Pre Booster P.", name = "n2_pre_bst_p")]
    N2PreBoosterPressure(BarG),

    /// Pressure of the H2 purge line.
    #[mavlink(id = 0x10_06_62)]
    #[pss(label = "H2 purge P.", name = "h2_prg_p")]
    H2PurgePressure(BarG),

    /// Pressure of the N2 post-booster line.
    #[mavlink(id = 0x10_06_63)]
    #[pss(label = "N2 Post Booster P.", name = "n2_post_bst_p")]
    N2PostBoosterPressure(BarG),

    /// Pressure of the LOX line.
    #[mavlink(id = 0x10_06_64)]
    #[pss(label = "LOx P.", name = "lox_line_p")]
    LoxPressure(BarG),

    /// Pressure of the purge line.
    #[mavlink(id = 0x10_06_65)]
    #[pss(label = "Purge P.", name = "n2_prg_p")]
    PurgePressure(BarG),

    /// Pressure of the pneumatic line.
    #[mavlink(id = 0x10_06_66)]
    #[pss(label = "Pneuma. P.", name = "n2_pnm_p")]
    PneumaticPressure(BarG),

    /// Pressure of the GOX igniter line.
    #[mavlink(id = 0x10_06_67)]
    #[pss(label = "GOx P.", name = "gox_line_p")]
    GoxPressure(BarG),

    /// Pressure of the H2 igniter line.
    #[mavlink(id = 0x10_06_68)]
    #[pss(label = "H2 P.", name = "h2_line_p")]
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

    /// Controls the engine purge valve on the RFS from the FC.
    ///
    /// This is needed during the ignition sequence where the flight computer controls the
    /// ignition and thus needs to purge the engine.
    ///
    /// # CAN
    /// - **Kind**: Event message
    /// - **Sent by**: Flight Computer
    #[can(id = 0x20f)]
    N2RocketPurgeValveControl(NormallyClosedValve),
}
