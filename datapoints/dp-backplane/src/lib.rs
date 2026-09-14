#![cfg_attr(not(feature = "std"), no_std)]

use data_core::Message;
#[cfg(feature = "can")]
use data_core::can;
#[cfg(feature = "mavlink")]
use data_core::mavlink;
#[cfg(feature = "pss")]
use data_core::pss;
use datatypes::status::{BuildInformationCommon, StatusCommonMessage};
use datatypes::units::RailStatus;

use gated_derive::gated_derive;

pub use datatypes::status::ActivePowerSource;

mod version;
pub use version::{VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH};

trait MessageMarker<V> {
    type Content: Sized + 'static;
    const VARIANT: V;
}

// DERIVED:

// derive FromStr, maybe using facet aliases
enum MessageVariant {
    BuildInfo,
    // alias: `fss_tnk_p1`
    TankPressure1,
}

mod messages {
    use crate::{MessageMarker, MessageVariant};
    use datatypes::status::BuildInformationCommon;

    enum BuildInfo {}
    impl MessageMarker<MessageVariant> for BuildInfo {
        type Content = BuildInformationCommon;
        const VARIANT: MessageVariant = MessageVariant::BuildInfo;
    }
}

trait DataPointDef {
    type DataPointVariant;
    fn variant(&self) -> Self::DataPointVariant;
}

impl DataPointDef for Message {
    type DataPointVariant = MessageVariant;
    fn variant(&self) -> MessageVariant {
        match self {
            Message::BuildInfo(_) => MessageVariant::BuildInfo,
            _ => todo!(),
        }
    }
}

// END DERIVED

/// Holds a value for every datapoint in `M`.
///
/// `M` is an enum of newtypes with an associated variant unit enum.
/// This map then stores a inner type for every variant.
struct Store<M> {
    // map marker variant -> watchable<dyn Any or Message>
    // EnumMap from: https://docs.rs/enum-map/3.1.0/enum_map/index.html
    datapoints: core::marker::PhantomData<M>,
}

// dummy -> see lib-remote-computers/lib-mission/src/reactive.rs
trait Watched<T> {}

impl<M: DataPointDef> Store<M> {
    // for manual stuffs (i.e. if message is known)
    fn get<V: MessageMarker<M::DataPointVariant>>(&self) -> impl Watched<V::Content> {
        self.get_value(V::VARIANT)
            .expect("message marker variant/content is to be trusted")
    }

    // for automatic stuff (e.g. sequences)
    fn get_value<T>(&self, variant: M::DataPointVariant) -> Result<impl Watched<T>, ()> {
        todo!("get message, make mapper to find offset/peek with facet and downcast")
        // Should be able to do that within a Mapped Watched implementation (feels inefficient though)
    }

    /// Sets the current value for this message variant.
    fn set(&self, message: Message) {
        let key = message.variant();
        todo!("")
    }
}

// TODO: timestamp & default/init

#[derive(Message, Clone)]
#[gated_derive(can::Message, Debug, defmt::Format)]
#[gated_derive(mavlink::Message)]
#[gated_derive(utoipa::ToSchema, serde::Deserialize, serde::Serialize)]
#[gated_derive(pss::Data, facet::Facet)]
#[repr(u8)]
#[serde(tag = "type", content = "content")]
#[schema(as = BackplaneMessage)]
pub enum Message {
    // ======================== TX ========================
    /// ## Type
    /// - `Periodic @ 0.2 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `Backplane`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Allows version tracking and verification of the backplane firmware.
    ///
    /// ## Reaction
    /// Receivers may log or display the firmware version and build metadata.
    #[can(id = 0x615)]
    #[mavlink(id = 0x10_04_00)]
    BuildInfo(BuildInformationCommon),

    /// ## Type
    /// - `Periodic @ 1 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `Backplane`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the runtime health of the backplane.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x315)]
    #[mavlink(id = 0x10_04_01)]
    BoardStatus(StatusCommonMessage),

    /// ## Type
    /// - Periodic @ 10Hz
    ///
    /// ## Direction
    /// - **Emitted by:** `Backplane`
    /// - **Received by:** unspecified
    ///
    /// ## Purpose
    /// Reports the measured voltage, current, and power on the 5V power rail.
    ///
    /// ## Reaction
    /// Receivers may log, display, or track these values. They may trigger warnings if any measurement is out of nominal range.
    #[can(id = 0x500)]
    #[mavlink(id = 0x10_04_02)]
    RailStatus5V(RailStatus),

    /// ## Type
    /// - Periodic @ 10Hz
    ///
    /// ## Direction
    /// - **Emitted by:** `Backplane`
    /// - **Received by:** unspecified
    ///
    /// ## Purpose
    /// Reports the measured voltage, current, and power on the 24V power rail.
    ///
    /// ## Reaction
    /// Receivers may log, display, or track these values. They may trigger warnings if any measurement is out of nominal range.
    #[can(id = 0x501)]
    #[mavlink(id = 0x10_04_03)]
    RailStatus24V(RailStatus),

    /// ## Type
    /// - Periodic @ 1 Hz
    ///
    /// ## Direction
    /// - **Emitted by:** `Backplane`
    /// - **Received by:** unspecified
    ///
    /// ## Purpose
    /// Reports which power source is currently active.
    ///
    /// ## Reaction
    /// Receivers may log, display, or track this value. They may trigger warnings if the power source is not valid.
    #[can(id = 0x300)]
    #[mavlink(id = 0x10_04_04)]
    ActivePowerSource(ActivePowerSource),
}
