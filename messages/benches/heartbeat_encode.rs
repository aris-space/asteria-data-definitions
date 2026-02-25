//! This benchmark compares the Mavio encoding of a single message with ours.
//!
//! Benchmarks can be run with `cargo bench`
//!
//! ## Results (just a raw message)
//! - mavio: 88.3ns +- 0.3ns
//! - ours: 32.0ns +- 0.2ns
//! - bincode: approx 19.5ns +- 0.2ns
//!
//! Honestly, since they use the same tbytes library and not much work is done, this isn't that interesting.
//! But mavio by default allocates a Vec (iirc), while we can stay on the stack.
//!
//! ## Results (dialect message)
//! - mavio: 90ns
//! - ours: 90ns
//! - bincode: 20ns
//!
//! Now we see that the `Payload::new` from mavio will allocate also with our encoding in the background.
//! No real difference.
//!

use criterion::{Criterion, criterion_group, criterion_main};
use data_core::mavlink::mavio;
use std::hint::black_box;

fn encode_mavio(
    msg: &mavio::dialects::Minimal,
) -> Result<mavio::protocol::Payload, mavio::mavspec::rust::spec::SpecError> {
    use mavio::protocol::IntoPayload as _;
    msg.encode(mavio::prelude::MavLinkVersion::V2)
}

fn mavio_benchmark(c: &mut Criterion) {
    let msg = mavio::dialects::minimal::Minimal::Heartbeat(
        mavio::dialects::minimal::messages::Heartbeat {
            type_: mavio::default_dialect::enums::MavType::Rocket,
            autopilot: mavio::default_dialect::enums::MavAutopilot::Invalid,
            base_mode: mavio::default_dialect::enums::MavModeFlag::CUSTOM_MODE_ENABLED,
            custom_mode: 42,
            system_status: mavio::default_dialect::enums::MavState::Standby,
            mavlink_version: 3, // minor version of minimal.xml
        },
    );
    c.bench_function("enc heartbeat mavio", |b| {
        b.iter(|| encode_mavio(black_box(&msg)));
    });
}

fn encode_ours(
    msg: &heartbeat::Dialect,
) -> Result<mavio::protocol::Payload, mavio::mavspec::rust::spec::SpecError> {
    use mavio::protocol::IntoPayload as _;
    msg.encode(mavio::prelude::MavLinkVersion::V2)
}

fn ours_benchmark(c: &mut Criterion) {
    let msg = heartbeat::Dialect::Minimal(heartbeat::Minimal::Heartbeat(heartbeat::Heartbeat {
        type_: heartbeat::MavType::Rocket,
        autopilot: heartbeat::MavAutopilot::Invalid,
        base_mode: 1,
        custom_mode: 42,
        system_status: heartbeat::MavState::Standby,
        mavlink_version: 3, // minor version of minimal.xml
    }));
    c.bench_function("enc heartbeat ours", |b| {
        b.iter(|| encode_ours(black_box(&msg)));
    });
}

fn inlineable_ours_benchmark(c: &mut Criterion) {
    let inner = heartbeat::Heartbeat {
        type_: heartbeat::MavType::Rocket,
        autopilot: heartbeat::MavAutopilot::Invalid,
        base_mode: 1,
        custom_mode: 42,
        system_status: heartbeat::MavState::Standby,
        mavlink_version: 3, // minor version of minimal.xml
    };
    c.bench_function("enc heartbeat ours inlineable", |b| {
        b.iter(|| {
            encode_ours(&heartbeat::Dialect::Minimal(heartbeat::Minimal::Heartbeat(
                black_box(inner),
            )))
        });
    });
}

fn encode_bincode(msg: &heartbeat::Dialect) -> Result<[u8; 64], bincode::error::EncodeError> {
    let mut pay = [0; 64];
    let _len = bincode::encode_into_slice(
        msg,
        &mut pay,
        bincode::config::standard()
            .with_fixed_int_encoding()
            .with_limit::<64>(),
    )?;
    Ok(pay)
}

fn bincode_benchmark(c: &mut Criterion) {
    let msg = heartbeat::Dialect::Minimal(heartbeat::Minimal::Heartbeat(heartbeat::Heartbeat {
        type_: heartbeat::MavType::Rocket,
        autopilot: heartbeat::MavAutopilot::Invalid,
        base_mode: 1,
        custom_mode: 42,
        system_status: heartbeat::MavState::Standby,
        mavlink_version: 3, // minor version of minimal.xml
    }));
    c.bench_function("enc heartbeat bincode", |b| {
        b.iter(|| encode_bincode(black_box(&msg)));
    });
}

criterion_group!(
    benches,
    mavio_benchmark,
    ours_benchmark,
    inlineable_ours_benchmark,
    bincode_benchmark
);
criterion_main!(benches);

mod heartbeat {

    #[derive(data_core::mavlink::Dialect, bincode::Encode)]
    #[dialect(id = 1, name = "minimal", version = 2)] // arbitrary
    pub enum Dialect {
        /// Stuff from minimal
        Minimal(Minimal),
    }

    #[derive(data_core::mavlink::Message, bincode::Encode)]
    pub enum Minimal {
        /// Heartbeat
        #[mavlink(id = 0)]
        Heartbeat(Heartbeat),
        /// Protocol version message
        #[mavlink(id = 300)]
        ProtocolVersion(ProtocolVersion),
    }

    // crc_extra = 217
    #[derive(
        data_core::mavlink::FieldSet,
        data_core::mavlink::Encode,
        data_core::mavlink::Decode,
        core::marker::Copy,
        core::clone::Clone,
        core::fmt::Debug,
        core::default::Default,
        core::cmp::PartialEq,
        bincode::Encode,
    )]
    pub struct ProtocolVersion {
        /// MAVLink field `version`.
        ///
        /// Currently active MAVLink version number * 100: v1.0 is 100, v2.0 is 200, etc.
        pub version: u16,
        /// MAVLink field `min_version`.
        ///
        /// Minimum MAVLink version supported
        pub min_version: u16,
        /// MAVLink field `max_version`.
        ///
        /// Maximum MAVLink version supported (set to the same value as version by default)
        pub max_version: u16,
        /// MAVLink field `spec_version_hash`.
        ///
        /// The first 8 bytes (not characters printed in hex!) of the git hash.
        pub spec_version_hash: [u8; 8],
        /// MAVLink field `library_version_hash`.
        ///
        /// The first 8 bytes (not characters printed in hex!) of the git hash.
        pub library_version_hash: [u8; 8],
    }

    #[derive(
        data_core::mavlink::FieldSet,
        data_core::mavlink::Encode,
        data_core::mavlink::Decode,
        core::marker::Copy,
        core::clone::Clone,
        core::fmt::Debug,
        core::default::Default,
        core::cmp::PartialEq,
        bincode::Encode,
    )]
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
    ///
    ///
    #[derive(
        data_core::mavlink::FieldSet,
        data_core::mavlink::Enum,
        data_core::mavlink::Encode,
        data_core::mavlink::Decode,
        core::marker::Copy,
        core::clone::Clone,
        core::fmt::Debug,
        core::default::Default,
        core::cmp::PartialEq,
        bincode::Encode,
    )]
    #[repr(u8)]
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
    #[derive(
        data_core::mavlink::FieldSet,
        data_core::mavlink::Enum,
        data_core::mavlink::Encode,
        data_core::mavlink::Decode,
        core::marker::Copy,
        core::clone::Clone,
        core::fmt::Debug,
        core::default::Default,
        core::cmp::PartialEq,
        bincode::Encode,
    )]
    #[repr(u8)]
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
        /// Open Drone ID. See https://mavlink.io/en/services/opendroneid.html.
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
        /// in cruise flight. See: https://en.wikipedia.org/wiki/Gyrodyne
        VtolGyrodyne = 47,
    }

    /// MAVLink enum `MAV_AUTOPILOT` for `minimal` dialect.
    ///
    /// Micro air vehicle / autopilot classes. This identifies the individual model.
    #[derive(
        data_core::mavlink::FieldSet,
        data_core::mavlink::Enum,
        data_core::mavlink::Encode,
        data_core::mavlink::Decode,
        core::marker::Copy,
        core::clone::Clone,
        core::fmt::Debug,
        core::default::Default,
        core::cmp::PartialEq,
        bincode::Encode,
    )]
    #[repr(u8)]
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
        /// SLUGS autopilot, http://slugsuav.soe.ucsc.edu
        Slugs = 2,
        /// MAVLink enum entry `MAV_AUTOPILOT_ARDUPILOTMEGA`.
        ///
        /// ArduPilot - Plane/Copter/Rover/Sub/Tracker, https://ardupilot.org
        Ardupilotmega = 3,
        /// MAVLink enum entry `MAV_AUTOPILOT_OPENPILOT`.
        ///
        /// OpenPilot, http://openpilot.org
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
        /// PPZ UAV - http://nongnu.org/paparazzi
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
        /// PX4 Autopilot - http://px4.io/
        Px4 = 12,
        /// MAVLink enum entry `MAV_AUTOPILOT_SMACCMPILOT`.
        ///
        /// SMACCMPilot - http://smaccmpilot.org
        Smaccmpilot = 13,
        /// MAVLink enum entry `MAV_AUTOPILOT_AUTOQUAD`.
        ///
        /// AutoQuad -- http://autoquad.org
        Autoquad = 14,
        /// MAVLink enum entry `MAV_AUTOPILOT_ARMAZILA`.
        ///
        /// Armazila -- http://armazila.com
        Armazila = 15,
        /// MAVLink enum entry `MAV_AUTOPILOT_AEROB`.
        ///
        /// Aerob -- http://aerob.ru
        Aerob = 16,
        /// MAVLink enum entry `MAV_AUTOPILOT_ASLUAV`.
        ///
        /// ASLUAV autopilot -- http://www.asl.ethz.ch
        Asluav = 17,
        /// MAVLink enum entry `MAV_AUTOPILOT_SMARTAP`.
        ///
        /// SmartAP Autopilot - http://sky-drones.com
        Smartap = 18,
        /// MAVLink enum entry `MAV_AUTOPILOT_AIRRAILS`.
        ///
        /// AirRails - http://uaventure.com
        Airrails = 19,
        /// MAVLink enum entry `MAV_AUTOPILOT_REFLEX`.
        ///
        /// Fusion Reflex - https://fusion.engineering
        Reflex = 20,
    }
}
