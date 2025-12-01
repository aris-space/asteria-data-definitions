use data_core::{CanMessage, Message};
use datatypes::{BuildInformationCommon, Celsius, NormallyOpenValve, StatusCommonMessage};

#[derive(Message, CanMessage)]
pub enum Message {
    #[can(id = 0x021)]
    ResetBoard,
    #[can(id = 0x022)]
    BoardStatus(StatusCommonMessage),
    #[can(id = 0x023)]
    BuildInfo(BuildInformationCommon),
    #[can(id = 0x024)]
    OxidizerVentControlRFS(NormallyOpenValve),
    #[can(id = 0x025)]
    OxidizerVentControlFC(NormallyOpenValve),
    #[can(id = 0x026)]
    OxidizerVentControlECU(NormallyOpenValve),
    #[can(id = 0x027)]
    OxidizerTankTemperature(Celsius),
}
