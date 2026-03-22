#![cfg_attr(not(feature = "std"), no_std)]

use data_core::Message;
#[cfg(feature = "can")]
use data_core::can;
#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "derive_pss")]
use data_core::pss;
use datatypes::actuator::{DigitalOut, NormallyClosedValve};
use datatypes::status::BuildInformationCommon;
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
    #[pss(label = "Spark Plug", name = "ign_spk_plg", kind = "digital_actuator")]
    IgniterSparkPlugState(DigitalOut),

    /// The current active state of the normally closed GOX igniter valve.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x332)]
    #[pss(label = "GOX Igniter", name = "gox_ign_vlv", kind = "digital_actuator")]
    GoxIgniterValveState(NormallyClosedValve),

    /// The current active state of the normally closed H2 igniter valve.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x333)]
    #[pss(label = "H2 Igniter", name = "h2_ign_vlv", kind = "digital_actuator")]
    H2IgniterValveState(NormallyClosedValve),

    /// The current active state of the normally closed GOX purge valve.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x334)]
    #[pss(label = "GOX Purge", name = "gox_prg_vlv", kind = "digital_actuator")]
    GoxPurgeValveState(NormallyClosedValve),

    /// The current active state of the normally closed H2 purge valve.
    ///
    /// # CAN
    /// - **Kind**: Status message
    /// - **Sent by**: RFS Computer
    #[can(id = 0x335)]
    #[pss(label = "H2 Purge", name = "h2_prg_vlv", kind = "digital_actuator")]
    H2PurgeValveState(NormallyClosedValve),

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
