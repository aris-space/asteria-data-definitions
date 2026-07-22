#[cfg(feature = "mavlink")]
use data_core::mavlink;
use gated_derive::gated_derive;

/// Heartbeat payload of mavlink minimal.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[derive(Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Heartbeat {
    /// MAVLink field `type`.
    ///
    /// Vehicle or component type. For a flight controller component the vehicle type (quadrotor,
    /// helicopter, etc.). For other components the component type (e.g. camera, gimbal,
    /// etc.). This should be used in preference to component id for identifying the component
    /// type.
    #[mavlink(refine)]
    pub type_: MavType,
    /// MAVLink field `autopilot`.
    ///
    /// Autopilot type / class. Use MAV_AUTOPILOT_INVALID for components that are not flight
    /// controllers.
    #[mavlink(refine)]
    pub autopilot: MavAutopilot,
    /// MAVLink field `base_mode`.
    ///
    /// System mode bitmap.
    pub base_mode: u8,
    /// MAVLink field `custom_mode`.
    ///
    /// A bitfield for use for autopilot-specific flags
    pub custom_mode: u32,
    /// MAVLink field `system_status`.
    ///
    /// System status flag.
    #[mavlink(refine)]
    pub system_status: MavState,
    /// MAVLink field `mavlink_version`.
    ///
    /// MAVLink version, not writable by user, gets added by protocol because of magic data
    /// type: uint8_t_mavlink_version
    pub mavlink_version: u8,
}

/// MAVLink enum `MAV_STATE` for `minimal` dialect.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MavState {
    #[default]
    /// MAVLink enum entry `MAV_STATE_UNINIT`.
    ///
    /// Uninitialized system, state is unknown.
    Uninit = 0,
    /// MAVLink enum entry `MAV_STATE_BOOT`.
    ///
    /// System is booting up.
    Boot = 1,
    /// MAVLink enum entry `MAV_STATE_CALIBRATING`.
    ///
    /// System is calibrating and not flight-ready.
    Calibrating = 2,
    /// MAVLink enum entry `MAV_STATE_STANDBY`.
    ///
    /// System is grounded and on standby. It can be launched any time.
    Standby = 3,
    /// MAVLink enum entry `MAV_STATE_ACTIVE`.
    ///
    /// System is active and might be already airborne. Motors are engaged.
    Active = 4,
    /// MAVLink enum entry `MAV_STATE_CRITICAL`.
    ///
    /// System is in a non-normal flight mode (failsafe). It can however still navigate.
    Critical = 5,
    /// MAVLink enum entry `MAV_STATE_EMERGENCY`.
    ///
    /// System is in a non-normal flight mode (failsafe). It lost control over parts or
    /// over the whole airframe. It is in mayday and going down.
    Emergency = 6,
    /// MAVLink enum entry `MAV_STATE_POWEROFF`.
    ///
    /// System just initialized its power-down sequence, will shut down now.
    Poweroff = 7,
    /// MAVLink enum entry `MAV_STATE_FLIGHT_TERMINATION`.
    ///
    /// System is terminating itself (failsafe or commanded).
    FlightTermination = 8,
}

/// MAVLink enum `MAV_TYPE` for `minimal` dialect.
///
/// MAVLINK component type reported in HEARTBEAT message. Flight controllers must report
/// the type of the vehicle on which they are mounted (e.g. MAV_TYPE_OCTOROTOR). All
/// other components must report a value appropriate for their type (e.g. a camera must
/// use MAV_TYPE_CAMERA).
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MavType {
    #[default]
    /// MAVLink enum entry `MAV_TYPE_GENERIC`.
    ///
    /// Generic micro air vehicle
    Generic = 0,
    /// MAVLink enum entry `MAV_TYPE_FIXED_WING`.
    ///
    /// Fixed wing aircraft.
    FixedWing = 1,
    /// MAVLink enum entry `MAV_TYPE_QUADROTOR`.
    ///
    /// Quadrotor
    Quadrotor = 2,
    /// MAVLink enum entry `MAV_TYPE_COAXIAL`.
    ///
    /// Coaxial helicopter
    Coaxial = 3,
    /// MAVLink enum entry `MAV_TYPE_HELICOPTER`.
    ///
    /// Normal helicopter with tail rotor.
    Helicopter = 4,
    /// MAVLink enum entry `MAV_TYPE_ANTENNA_TRACKER`.
    ///
    /// Ground installation
    AntennaTracker = 5,
    /// MAVLink enum entry `MAV_TYPE_GCS`.
    ///
    /// Operator control unit / ground control station
    Gcs = 6,
    /// MAVLink enum entry `MAV_TYPE_AIRSHIP`.
    ///
    /// Airship, controlled
    Airship = 7,
    /// MAVLink enum entry `MAV_TYPE_FREE_BALLOON`.
    ///
    /// Free balloon, uncontrolled
    FreeBalloon = 8,
    /// MAVLink enum entry `MAV_TYPE_ROCKET`.
    ///
    /// Rocket
    Rocket = 9,
    /// MAVLink enum entry `MAV_TYPE_GROUND_ROVER`.
    ///
    /// Ground rover
    GroundRover = 10,
    /// MAVLink enum entry `MAV_TYPE_SURFACE_BOAT`.
    ///
    /// Surface vessel, boat, ship
    SurfaceBoat = 11,
    /// MAVLink enum entry `MAV_TYPE_SUBMARINE`.
    ///
    /// Submarine
    Submarine = 12,
    /// MAVLink enum entry `MAV_TYPE_HEXAROTOR`.
    ///
    /// Hexarotor
    Hexarotor = 13,
    /// MAVLink enum entry `MAV_TYPE_OCTOROTOR`.
    ///
    /// Octorotor
    Octorotor = 14,
    /// MAVLink enum entry `MAV_TYPE_TRICOPTER`.
    ///
    /// Tricopter
    Tricopter = 15,
    /// MAVLink enum entry `MAV_TYPE_FLAPPING_WING`.
    ///
    /// Flapping wing
    FlappingWing = 16,
    /// MAVLink enum entry `MAV_TYPE_KITE`.
    ///
    /// Kite
    Kite = 17,
    /// MAVLink enum entry `MAV_TYPE_ONBOARD_CONTROLLER`.
    ///
    /// Onboard companion controller
    OnboardController = 18,
    /// MAVLink enum entry `MAV_TYPE_VTOL_TAILSITTER_DUOROTOR`.
    ///
    /// Two-rotor Tailsitter VTOL that additionally uses control surfaces in vertical operation.
    /// Note, value previously named MAV_TYPE_VTOL_DUOROTOR.
    VtolTailsitterDuorotor = 19,
    /// MAVLink enum entry `MAV_TYPE_VTOL_TAILSITTER_QUADROTOR`.
    ///
    /// Quad-rotor Tailsitter VTOL using a V-shaped quad config in vertical operation. Note:
    /// value previously named MAV_TYPE_VTOL_QUADROTOR.
    VtolTailsitterQuadrotor = 20,
    /// MAVLink enum entry `MAV_TYPE_VTOL_TILTROTOR`.
    ///
    /// Tiltrotor VTOL. Fuselage and wings stay (nominally) horizontal in all flight phases.
    /// It able to tilt (some) rotors to provide thrust in cruise flight.
    VtolTiltrotor = 21,
    /// MAVLink enum entry `MAV_TYPE_VTOL_FIXEDROTOR`.
    ///
    /// VTOL with separate fixed rotors for hover and cruise flight. Fuselage and wings
    /// stay (nominally) horizontal in all flight phases.
    VtolFixedrotor = 22,
    /// MAVLink enum entry `MAV_TYPE_VTOL_TAILSITTER`.
    ///
    /// Tailsitter VTOL. Fuselage and wings orientation changes depending on flight phase:
    /// vertical for hover, horizontal for cruise. Use more specific VTOL MAV_TYPE_VTOL_TAILSITTER_DUOROTOR
    /// or MAV_TYPE_VTOL_TAILSITTER_QUADROTOR if appropriate.
    VtolTailsitter = 23,
    /// MAVLink enum entry `MAV_TYPE_VTOL_TILTWING`.
    ///
    /// Tiltwing VTOL. Fuselage stays horizontal in all flight phases. The whole wing, along
    /// with any attached engine, can tilt between vertical and horizontal mode.
    VtolTiltwing = 24,
    /// MAVLink enum entry `MAV_TYPE_VTOL_RESERVED5`.
    ///
    /// VTOL reserved 5
    VtolReserved5 = 25,
    /// MAVLink enum entry `MAV_TYPE_GIMBAL`.
    ///
    /// Gimbal
    Gimbal = 26,
    /// MAVLink enum entry `MAV_TYPE_ADSB`.
    ///
    /// ADSB system
    Adsb = 27,
    /// MAVLink enum entry `MAV_TYPE_PARAFOIL`.
    ///
    /// Steerable, nonrigid airfoil
    Parafoil = 28,
    /// MAVLink enum entry `MAV_TYPE_DODECAROTOR`.
    ///
    /// Dodecarotor
    Dodecarotor = 29,
    /// MAVLink enum entry `MAV_TYPE_CAMERA`.
    ///
    /// Camera
    Camera = 30,
    /// MAVLink enum entry `MAV_TYPE_CHARGING_STATION`.
    ///
    /// Charging station
    ChargingStation = 31,
    /// MAVLink enum entry `MAV_TYPE_FLARM`.
    ///
    /// FLARM collision avoidance system
    Flarm = 32,
    /// MAVLink enum entry `MAV_TYPE_SERVO`.
    ///
    /// Servo
    Servo = 33,
    /// MAVLink enum entry `MAV_TYPE_ODID`.
    ///
    /// Open Drone ID. See <https://mavlink.io/en/services/opendroneid.html>.
    Odid = 34,
    /// MAVLink enum entry `MAV_TYPE_DECAROTOR`.
    ///
    /// Decarotor
    Decarotor = 35,
    /// MAVLink enum entry `MAV_TYPE_BATTERY`.
    ///
    /// Battery
    Battery = 36,
    /// MAVLink enum entry `MAV_TYPE_PARACHUTE`.
    ///
    /// Parachute
    Parachute = 37,
    /// MAVLink enum entry `MAV_TYPE_LOG`.
    ///
    /// Log
    Log = 38,
    /// MAVLink enum entry `MAV_TYPE_OSD`.
    ///
    /// OSD
    Osd = 39,
    /// MAVLink enum entry `MAV_TYPE_IMU`.
    ///
    /// IMU
    Imu = 40,
    /// MAVLink enum entry `MAV_TYPE_GPS`.
    ///
    /// GPS
    Gps = 41,
    /// MAVLink enum entry `MAV_TYPE_WINCH`.
    ///
    /// Winch
    Winch = 42,
    /// MAVLink enum entry `MAV_TYPE_GENERIC_MULTIROTOR`.
    ///
    /// Generic multirotor that does not fit into a specific type or whose type is unknown
    GenericMultirotor = 43,
    /// MAVLink enum entry `MAV_TYPE_ILLUMINATOR`.
    ///
    /// Illuminator. An illuminator is a light source that is used for lighting up dark
    /// areas external to the sytstem: e.g. a torch or searchlight (as opposed to a light
    /// source for illuminating the system itself, e.g. an indicator light).
    Illuminator = 44,
    /// MAVLink enum entry `MAV_TYPE_SPACECRAFT_ORBITER`.
    ///
    /// Orbiter spacecraft. Includes satellites orbiting terrestrial and extra-terrestrial
    /// bodies. Follows NASA Spacecraft Classification.
    SpacecraftOrbiter = 45,
    /// MAVLink enum entry `MAV_TYPE_GROUND_QUADRUPED`.
    ///
    /// A generic four-legged ground vehicle (e.g., a robot dog).
    GroundQuadruped = 46,
    /// MAVLink enum entry `MAV_TYPE_VTOL_GYRODYNE`.
    ///
    /// VTOL hybrid of helicopter and autogyro. It has a main rotor for lift and separate
    /// propellers for forward flight. The rotor must be powered for hover but can autorotate
    /// in cruise flight. See: <https://en.wikipedia.org/wiki/Gyrodyne>
    VtolGyrodyne = 47,
}

/// MAVLink enum `MAV_AUTOPILOT` for `minimal` dialect.
///
/// Micro air vehicle / autopilot classes. This identifies the individual model.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[gated_derive(mavlink::Encode, mavlink::Decode, mavlink::FieldSet, mavlink::Enum)]
#[gated_derive(utoipa::ToSchema, facet::Facet)]
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MavAutopilot {
    #[default]
    /// MAVLink enum entry `MAV_AUTOPILOT_GENERIC`.
    ///
    /// Generic autopilot, full support for everything
    Generic = 0,
    /// MAVLink enum entry `MAV_AUTOPILOT_RESERVED`.
    ///
    /// Reserved for future use.
    Reserved = 1,
    /// MAVLink enum entry `MAV_AUTOPILOT_SLUGS`.
    ///
    /// SLUGS autopilot, <http://slugsuav.soe.ucsc.edu>
    Slugs = 2,
    /// MAVLink enum entry `MAV_AUTOPILOT_ARDUPILOTMEGA`.
    ///
    /// ArduPilot - Plane/Copter/Rover/Sub/Tracker, <https://ardupilot.org>
    Ardupilotmega = 3,
    /// MAVLink enum entry `MAV_AUTOPILOT_OPENPILOT`.
    ///
    /// OpenPilot, <http://openpilot.org>
    Openpilot = 4,
    /// MAVLink enum entry `MAV_AUTOPILOT_GENERIC_WAYPOINTS_ONLY`.
    ///
    /// Generic autopilot only supporting simple waypoints
    GenericWaypointsOnly = 5,
    /// MAVLink enum entry `MAV_AUTOPILOT_GENERIC_WAYPOINTS_AND_SIMPLE_NAVIGATION_ONLY`.
    ///
    /// Generic autopilot supporting waypoints and other simple navigation commands
    GenericWaypointsAndSimpleNavigationOnly = 6,
    /// MAVLink enum entry `MAV_AUTOPILOT_GENERIC_MISSION_FULL`.
    ///
    /// Generic autopilot supporting the full mission command set
    GenericMissionFull = 7,
    /// MAVLink enum entry `MAV_AUTOPILOT_INVALID`.
    ///
    /// No valid autopilot, e.g. a GCS or other MAVLink component
    Invalid = 8,
    /// MAVLink enum entry `MAV_AUTOPILOT_PPZ`.
    ///
    /// PPZ UAV - <http://nongnu.org/paparazzi>
    Ppz = 9,
    /// MAVLink enum entry `MAV_AUTOPILOT_UDB`.
    ///
    /// UAV Dev Board
    Udb = 10,
    /// MAVLink enum entry `MAV_AUTOPILOT_FP`.
    ///
    /// FlexiPilot
    Fp = 11,
    /// MAVLink enum entry `MAV_AUTOPILOT_PX4`.
    ///
    /// PX4 Autopilot - <http://px4.io/>
    Px4 = 12,
    /// MAVLink enum entry `MAV_AUTOPILOT_SMACCMPILOT`.
    ///
    /// SMACCMPilot - <http://smaccmpilot.org>
    Smaccmpilot = 13,
    /// MAVLink enum entry `MAV_AUTOPILOT_AUTOQUAD`.
    ///
    /// AutoQuad -- <http://autoquad.org>
    Autoquad = 14,
    /// MAVLink enum entry `MAV_AUTOPILOT_ARMAZILA`.
    ///
    /// Armazila -- <http://armazila.com>
    Armazila = 15,
    /// MAVLink enum entry `MAV_AUTOPILOT_AEROB`.
    ///
    /// Aerob -- <http://aerob.ru>
    Aerob = 16,
    /// MAVLink enum entry `MAV_AUTOPILOT_ASLUAV`.
    ///
    /// ASLUAV autopilot -- <http://www.asl.ethz.ch>
    Asluav = 17,
    /// MAVLink enum entry `MAV_AUTOPILOT_SMARTAP`.
    ///
    /// SmartAP Autopilot - <http://sky-drones.com>
    Smartap = 18,
    /// MAVLink enum entry `MAV_AUTOPILOT_AIRRAILS`.
    ///
    /// AirRails - <http://uaventure.com>
    Airrails = 19,
    /// MAVLink enum entry `MAV_AUTOPILOT_REFLEX`.
    ///
    /// Fusion Reflex - <https://fusion.engineering>
    Reflex = 20,
}
