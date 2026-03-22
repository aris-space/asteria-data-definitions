#![cfg_attr(not(feature = "std"), no_std)]

use data_core::Message;
#[cfg(feature = "can")]
use data_core::can;
#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "derive_pss")]
use data_core::pss;
use datatypes::actuator::{DigitalOut, InvertedDigitalOut, NormallyClosedValve, NormallyOpenValve};
use datatypes::sensor::DigitalIn;
use datatypes::status::BuildInformationCommon;
use datatypes::units::Celsius;
use gated_derive::gated_derive;

mod dptypes;
pub use dptypes::*;

#[derive(Message)]
#[gated_derive(can::Message, Debug)]
#[gated_derive(mavlink::Message, pss::Data)]
#[gated_derive(utoipa::ToSchema, serde::Deserialize, serde::Serialize)]
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
    BuildInfo(BuildInformationCommon),

    /// Periodic status message emitted by the RFS Computer.
    ///
    /// Contains runtime health of the RFS Computer.
    /// It is also used to only ingite if the RFS is armed and ensure exclusive PSS control.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x311)]
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
    #[pss(label = "N2 Bleed", name = "n2_bld_vlv")]
    N2BleedValveState(NormallyOpenValve),

    /// The current active state of the normally closed LOX purge valve.
    #[pss(label = "LOX Purge", name = "lox_prg_vlv")]
    LoxPurgeValveState(NormallyClosedValve),

    /// The current active state of the normally open GOX bleed valve.
    #[pss(label = "GOX Bleed", name = "gox_bld_vlv")]
    GoxBleedValveState(NormallyOpenValve),

    /// The current active state of the normally closed N2 purge valve.
    #[pss(label = "N2 Purge", name = "n2_prg_vlv")]
    N2PurgeValveState(NormallyClosedValve),

    /// The current active state of the normally closed N2 fill valve.
    #[pss(label = "N2 Fill", name = "n2_fll_vlv")]
    N2FillValveState(NormallyClosedValve),

    /// The current active state of the normally open H2 bleed valve.
    #[pss(label = "H2 Bleed", name = "h2_bld_vlv")]
    H2BleedValveState(NormallyOpenValve),

    /// The current active state of the normally closed LOX bleed valve.
    #[pss(label = "LOX Bleed", name = "lox_bld_vlv")]
    LoxBleedValveState(NormallyClosedValve),

    /// The current active state of the normally closed pneumatic booster valve.
    #[pss(label = "Booster", name = "pnm_bstr_vlv")]
    BoosterValveState(NormallyClosedValve),

    /// The current active state of the normally closed LOX decoupler extend valve.
    #[pss(label = "LOX Dec. Extend", name = "pnm_loxdec_ext")]
    LoxDecouplerExtendValveState(NormallyClosedValve),

    /// The current active state of the normally closed LOX decoupler retract valve.
    #[pss(label = "LOX Dec. Retract", name = "pnm_loxdec_ret")]
    LoxDecouplerRetractValveState(NormallyClosedValve),

    /// The current active state of the normally closed N2 decoupler extend valve.
    #[pss(label = "N2 Dec. Extend", name = "pnm_n2dec_ext")]
    N2DecouplerExtendValveState(NormallyClosedValve),

    /// The current active state of the normally closed N2 decoupler retract valve.
    #[pss(label = "N2 Dec. Retract", name = "pnm_n2dec_ret")]
    N2DecouplerRetractValveState(NormallyClosedValve),

    /// The current active state of the normally closed N2 release valve.
    #[pss(label = "N2 Release", name = "pnm_n2rls_vlv")]
    N2ReleaseValveState(NormallyClosedValve),

    /// The current active state of the rocket power output.
    #[pss(label = "Rocket Power", name = "rocket_power")]
    RocketPowerState(InvertedDigitalOut),

    /// The current active state of the Maxon board power output.
    #[pss(label = "Maxon Power", name = "maxon_board_power")]
    MaxonBoardPowerState(InvertedDigitalOut),

    /// The current active state of the FSS heating output.
    #[pss(label = "FSS Heating", name = "fss_heating")]
    FssHeatingState(DigitalOut),

    /// The current active state of the pressure sensor heating output.
    #[pss(label = "Sensor Heat", name = "pres_sensor_heat")]
    PressureSensorHeatingState(DigitalOut),

    /// The current active state of the ETH pump power output.
    #[pss(label = "ETH Pump Power", name = "eth_pump_power")]
    EthPumpPowerState(DigitalOut),

    /// The current active state of the custom output 1.
    #[pss(label = "Custom 1", name = "custom1")]
    Custom1State(DigitalOut),

    /// The current active state of the custom output 2.
    #[pss(label = "Custom 2", name = "custom2")]
    Custom2State(DigitalOut),

    /// The current active state of the custom output 3.
    #[pss(label = "Custom 3", name = "custom3")]
    Custom3State(DigitalOut),

    // ======================== Sensors ========================
    /// The temperature of the LOx line, from sensor `R-T-L-1`.
    #[pss(label = "LOx Temperature", name = "r_t_l_1")]
    #[unit("°C")]
    LoxTemperature(Celsius),

    /// The temperature of the igniter, from sensor `I-T-I-1`.
    #[pss(label = "Igniter Temperature", name = "i_t_i_1")]
    #[unit("°C")]
    IgniterTemperature(Celsius),

    /// The temperature inside the DACS cabinet.
    #[pss(label = "Cabinet Temperature", name = "cabinet_t")]
    #[unit("°C")]
    CabinetTemperature(Celsius),

    /// The ambient temperature.
    #[pss(label = "Ambient Temperature", name = "ambient_t")]
    #[unit("°C")]
    AmbientTemperature(Celsius),

    /// Whether the LOx decoupler is in its retracted state, from the proximity sensor.
    #[pss(label = "LOx Decoupler Proximity", name = "lox_decoupler_prox")]
    LoxDecouplerProximity(DigitalIn),

    /// Whether the N2 decoupler is in its retracted state, from the proximity sensor.
    #[pss(label = "N2 Decoupler Proximity", name = "n2_decoupler_prox")]
    N2DecouplerProximity(DigitalIn),

    /// Whether the valves are receiving power (arming relay state).
    #[pss(label = "Valves Armed", name = "valves_armed")]
    ValvesArmed(DigitalIn),

    /// Whether the spark plug is receiving power (arming relay state).
    #[pss(label = "Igniter Armed", name = "igniter_armed")]
    IgniterArmed(DigitalIn),

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
