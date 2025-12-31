use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// The ID of a board in the rocket.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Encode, Decode)]
pub enum BoardId {
    FlightComputer = 0,
    RfsComputer = 1,
    SensorCarrier = 2,
    CommunicationBoard = 3,
    EngineControlBoard = 4,
    PowerBoard = 5,
    RecoveryBoard = 6,
    FuelControlBoard = 7,
    OxidizerControlBoard = 8,
}

/// UTC timestamp given in microseconds since the Unix epoch (`1970-01-01T00:00:00Z`).
///
/// This value is derived from the GPS time sensors and is therefore highly accurate.
#[derive(bincode::Encode, bincode::Decode)]
pub struct UTCTime(pub u64);

/// A temperature in degrees Celsius.
///
/// Is `NAN` by default.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Encode, Decode)]
pub struct Celsius(pub f32);

/// A pressure in bar relative to atmospheric pressure (`barg`).
///
/// Is `NAN` by default.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Encode, Decode)]
pub struct BarG(pub f32);

/// Atmospheric pressure in hPa.
///
/// Is `NAN` by default.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Encode, Decode)]
pub struct HPa(pub f32);

/// Pressurization Proportional Gain in [barg/ms]
/// opening_time = kp * (target_pressure - current_pressure), in milliseconds
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Encode, Decode)]
pub struct PressurizationGain(pub f32);

/// The state of a valve that is normally open.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub enum NormallyOpenValve {
    #[default]
    Open,
    Closed,
}

/// The state of a valve that is normally closed.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub enum NormallyClosedValve {
    Open,
    #[default]
    Closed,
}

#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub struct StatusCommonMessage {
    /// Number of errors encountered since startup.
    pub errors: u32,
    /// Number of microseconds since the last board restart.
    pub micros_since_restart: u64,
}

/// Shared firmware build information used by all boards.
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct BuildInformationCommon {
    /// Unix timestamp when the build was compiled, provided in seconds since
    /// the Unix epoch `(1970-01-01 00:00:00 UTC)`. While this is a 32-bit integer,
    /// overflow is not a concern as we don't intend to use this code up to 2038.
    pub unix_timestamp: u32,
    /// Initials of the person who compiled the firmware, or `\0\0` if not set.
    pub author_initials: [u8; 2],
    /// Whether the build is a release build.
    pub is_release: bool,
    /// Whether the build includes debugging defmt over RTT.
    pub debug_defmt_rtt: bool,
    /// Firmware version information.
    pub commit_hash: [u8; 7],
    /// Indicates if the git repository was dirty at build time.
    pub is_git_dirty: bool,
    /// CAN library version information (major, minor, patch).
    pub can_semver: [u8; 3],
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct CameraPowerStatus {
    /// Whether the recording camera(s) should be powered on (true=on, false=off).
    pub recording_pwr_enabled: bool,
    /// Whether the livestream camera(s) should be powered on (true=on, false=off).
    pub live_pwr_enabled: bool,
}

/// Shared rail status structure for both 5V and 24V power rails.
#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub struct RailStatus {
    /// Voltage in volts.
    pub voltage: f32,
    /// Current in amps.
    pub current: f32,
    /// Power in watts.
    pub power: f32,
}

/// Enum representing the status of a sensor.
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub enum SensorStatus {
    /// Sensor is not providing data or is not responding.
    #[default]
    Offline = 0,
    /// Sensor is generating data and responding.
    Online = 1,
}

/// Enum representing the state of a valve.
#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub enum ValveState {
    /// Power should be enabled for the valve
    Active,
    /// Power should not be disabled for the valve
    #[default]
    Inactive,
}

/// Represents the current arming state of the subsystem
#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub enum ArmingState {
    /// System is safe to approach, arming action is necessary to use the system.
    /// This can mean for example that actuators do not have power.
    #[default]
    Safe,
    /// System is not safe to approach anymore. System can be used to its full capacity
    /// This can mean for example that previously unpowered actuators now have power.
    Armed,
}

#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
pub enum DprState {
    /// Enabled with pressure setpoint in Barg. 0-60.
    Enabled(f32),
    /// Disabled
    #[default]
    Disabled,
}
