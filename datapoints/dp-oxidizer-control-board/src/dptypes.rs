#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "pss")]
use data_core::pss;
use datatypes::status::{SensorStatus, DprGainInfo, DprLoopInfo, SensorStatus, StatusCommonMessage};
use datatypes::units::BarG;
use gated_derive::gated_derive;

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct OxidizerControlBoardStatus {
    #[mavlink(flatten)]
    pub common: StatusCommonMessage,
    /// State of the ADS1120 device used to read thermocouples
    #[mavlink(refine)]
    pub thermocouple_status: SensorStatus,
    /// State of the Keller BUS used to read pressure, indicates whether a single sensor is online or not.
    #[mavlink(refine)]
    pub pressure_bus: SensorStatus,
    /// State of the DPR Control Loop
    #[mavlink(flatten)]
    pub dpr_loop_info: DprLoopInfo,
    /// State of the DPR Control Gain
    #[mavlink(flatten)]
    pub dpr_gain_info: DprGainInfo,
}
