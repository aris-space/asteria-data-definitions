use gated_derive::gated_derive;

use data_core::Message;
#[cfg(feature = "can")]
use data_core::can;
#[cfg(feature = "mavlink")]
use data_core::mavlink;

#[derive(Message)]
#[gated_derive(can::Message, Debug)]
#[gated_derive(mavlink::Message)]
#[gated_derive(utoipa::ToSchema, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", content = "content")]
#[schema(as = ManualActionMessage)]
pub enum Message {
    /// # `ManualStateTransition`
    /// Manually transition the state machine into another state using the Mission Control.
    ///
    /// ## Type
    /// - `Event`: sent by the Mission Control to trigger a state machine transition
    ///
    /// ## Direction
    /// - **Emitted by:** `Mission Control`
    /// - **Received by:** `Flight Computer` or `RFS Computer`
    ///
    /// ## Purpose
    /// Commands a manual state transition on the receiving system.
    ///
    /// ## Reaction
    /// The state machine performs a state transition and emits a state change event.
    #[mavlink(id = 0x20_09_00)]
    ManualStateTransition(ManualStateTransition),

    /// # `ManualActuatorChange`
    /// Manually change an actuator using the Mission Control.
    ///
    /// ## Type
    /// - `Event`: sent by the Mission Control to trigger an actuator change
    ///
    /// ## Direction
    /// - **Emitted by:** `Mission Control`
    /// - **Received by:** `Flight Computer` or `RFS Computer`
    ///
    /// ## Purpose
    /// Commands a specific actuator to move to a specific target position.
    /// The actuator name and the semantics of the value are defined using
    /// an `#[pss(name = "...", kind  = "...")]` attribute from the [`data_core::pss::Data`]
    /// macro.
    ///
    /// ## Reaction
    /// This is echoed back using "actuator states" messages.
    #[mavlink(id = 0x20_09_01)]
    ManualActuatorChange(ManualActuatorChange),
}

#[gated_derive(serde::Serialize, serde::Deserialize)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ManualStateTransition {
    /// The system this command is for.
    pub target_system: u8,
    /// The index in the state path where the new state should end up at.
    pub transition_index: u32,
    /// The name of the state to which the state machine should transition.
    pub state: [u8; 30],
}

#[gated_derive(serde::Serialize, serde::Deserialize)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ManualActuatorChange {
    /// The system this command is for.
    pub target_system: u8,
    /// The target value of the actuator.
    /// This depends on the actuator type, for valves 0.0 is fully closed
    /// and 1.0 is fully open.
    pub value: f32,
    /// The index in the state path where the current state is.
    /// For the initial state this is 0 and increments with every transition.
    pub path_index: u32,
    /// The name of the actuator whose value should be changed.
    pub actuator: [u8; 20],
}
