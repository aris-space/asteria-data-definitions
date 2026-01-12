use datatypes::status::{BoardId, InstantUs};

#[derive(Message, CanMessage)]
pub enum Message {
    // ======================== TX ========================
    /// # `UTCTimeUpdate`
    /// Synchronizes system time using GPS-derived UTC.
    ///
    /// ## Type
    /// - `Periodic @ 10 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `SensorCarrier`
    /// - **Received by:** `All nodes`
    ///
    /// ## Purpose
    /// Ensures consistent timekeeping across all nodes using GPS as a source of truth.
    ///
    /// ## Reaction
    /// Boards shall update their internal time reference used for logging and
    /// time-sensitive operations *once at startup*. While this value could in theory
    /// be used to correct for drift, this may result in non-monotonic timekeeping
    /// and is therefore discouraged.
    #[can(id = 0x30)]
    UTCTimeUpdate(InstantUs),

    /// # `ResetSpecific`
    /// Instructs a specific board to reset.
    ///
    /// ## Type
    /// - `Event`: triggered by user command or fault condition
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`
    /// - **Received by:** All nodes except `FlightComputer` and `RfsComputer`
    ///
    /// ## Purpose
    /// Allows targeted recovery or restart of a malfunctioning board.
    ///
    /// ## Reaction
    /// The addressed board performs an immediate reset. Others shall ignore the message.
    #[can(id = 0x20)]
    ResetSpecific(BoardId),

    /// # `ResetAll`
    /// Initiates a system-wide reset on all nodes.
    ///
    /// ## Type
    /// - `Event`: triggered by user command or fault condition
    ///
    /// ## Direction
    /// - **Emitted by:** `FlightComputer`, `RfsComputer`
    /// - **Received by:** All nodes except `FlightComputer` and `RfsComputer`
    ///
    /// ## Purpose
    /// Forces an immediate reset of all boards for system recovery or coordination.
    ///
    /// ## Reaction
    /// All boards perform a full software reset immediately on reception.
    #[can(id = 0x10)]
    ResetAll,
    // ======================== RX ========================
}
