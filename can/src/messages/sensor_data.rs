//! ## CAN Message Group: Sensor Data
//!
//! - **ID Range:** 0x400–0x5FF
//! - **Count:** 512 messages
//! - **Message Class:** Sensor Data
//! - **Priority:** Medium
//! - **Type:** Periodic
//!
//! Data produced by sensors, including raw and processed values.
//!
//! For more information on coordinate frames see the
//! [HERMES data standards](https://wiki.aris-space.ch/i/518)

use bincode::{Decode, Encode};
use hermes_can_derive::can_message;

////////////////////////////////////////////////////////////////////
// Navigation Data
////////////////////////////////////////////////////////////////////

/// # `PositionData`
/// Periodic telemetry message emitted by the Sensor Carrier.
///
/// ## Type
/// - `Periodic @ 20 Hz`
///
/// ## Direction
/// - **Emitted by:** `SensorCarrier`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Provides fused and filtered position data in the WGS84 geodetic frame.
///
/// ## Reaction
/// No action required.
///
/// ## Data Origin & Quality
/// This data is produced by an extended Kalman filter that fuses:
/// - Two GNSS receivers (single-antenna each)
/// - Two IMUs (accelerometer + gyroscope)
/// - Two barometric pressure sensors
///
/// The system estimates position in the WGS84 geodetic frame, using:
/// - GNSS as the primary position reference
/// - IMU and barometric data for dead-reckoning and short-term vertical stability
///
/// GNSS data provides reliable long-term accuracy but is subject to multipath and signal dropout.
/// Short-term dynamics are improved by inertial fusion, and barometric height assists with
/// vertical resolution. If GNSS data becomes unreliable, the filter will tend to dead-reckoning using
/// IMU and barometric data, which is subject to drift over time.
///
/// Accuracy estimates (`horizontal_accuracy` and `vertical_accuracy`) represent **1-sigma**
/// standard deviations from the Kalman filter covariance and take into account the GNSS
/// hdop and vdop values.
///
/// During GNSS dropout, position will degrade based on IMU drift and model assumptions.
/// Barometer-based altitude is relative to the initialization pressure and is sensitive to weather.
/// GNSS height is referred to mean sea level (HAMSL).
#[derive(Default)]
#[can_message(0x420)]
pub struct PositionData {
    /// Latitude in degrees (WGS84). Range: -90.0° to +90.0°
    pub location_latitude: f64,
    /// Longitude in degrees (WGS84). Range: -180.0° to +180.0°
    pub location_longitude: f64,
    /// Height above mean sea level in meters.
    pub location_hamsl: f32,
    /// Estimated horizontal position uncertainty (1-sigma) in meters.
    pub horizontal_accuracy: f32,
    /// Estimated vertical position uncertainty (1-sigma) in meters.
    pub vertical_accuracy: f32,
}

/// # `ImuData`
/// Periodic telemetry message emitted by the Sensor Carrier.
///
/// ## Type
/// - `Periodic @ 40 Hz`
///
/// ## Direction
/// - **Emitted by:** `SensorCarrier`
///
/// ## Purpose
/// Provides fused and filtered acceleration and angular velocity in both the body-fixed
/// and inertial NED frames.
///
/// ## Reaction
/// No action required.
///
/// ## Data Origin & Quality
/// This data is derived from two IMUs (each an accelerometer + gyroscope pair), mounted
/// closely together and aligned in the same direction.
///
/// The body-fixed frame is the sensor-native reference frame, and all acceleration and angular
/// velocity data in this frame is directly measured and highly reliable.
///
/// The inertial NED frame is computed by rotating the body-frame data using the estimated
/// orientation quaternion. This transformation introduces small errors due to attitude estimation
/// uncertainty. Roll and pitch are generally accurate. Yaw, while observed by a magnetometer,
/// is more prone to environmental noise and remains a larger source of error than roll and pitch.
///
/// Coordinate frame conversion therefore introduces small errors in the inertial NED frame data for:
/// - `angular_velocity_north`
/// - `angular_velocity_east`
/// - `acceleration_north`
/// - `acceleration_east`
///
/// Note that `acceleration_down` and `angular_velocity_down` are not affected by errors in
/// inertial yaw.
#[derive(Default)]
#[can_message(0x421)]
pub struct ImuData {
    // --- Body-fixed NED frame (X: forward, Y: right, Z: down) ---
    /// Acceleration along body X-axis (forward) in m/s². Range: ±78.48 m/s² (±8g)
    pub acceleration_x: f32,
    /// Acceleration along body Y-axis (right) in m/s². Range: ±78.48 m/s² (±8g)
    pub acceleration_y: f32,
    /// Acceleration along body Z-axis (down) in m/s². Range: ±78.48 m/s² (±8g)
    pub acceleration_z: f32,

    /// Angular velocity around body X-axis in °/s. Range: ±2000 °/s
    pub angular_velocity_x: f32,
    /// Angular velocity around body Y-axis in °/s. Range: ±2000 °/s
    pub angular_velocity_y: f32,
    /// Angular velocity around body Z-axis in °/s. Range: ±2000 °/s
    pub angular_velocity_z: f32,

    // --- Inertial NED frame (North, East, Down) ---
    /// Acceleration along North axis in m/s². Range: ±78.48 m/s² (±8g)
    pub acceleration_north: f32,
    /// Acceleration along East axis in m/s². Range: ±78.48 m/s² (±8g)
    pub acceleration_east: f32,
    /// Acceleration along Down axis in m/s². Range: ±78.48 m/s² (±8g)
    pub acceleration_down: f32,

    /// Angular velocity around North axis in °/s. Range: ±2000 °/s
    pub angular_velocity_north: f32,
    /// Angular velocity around East axis in °/s. Range: ±2000 °/s
    pub angular_velocity_east: f32,
    /// Angular velocity around Down axis in °/s. Range: ±2000 °/s
    pub angular_velocity_down: f32,
}

/// # `VelocityData`
/// Periodic telemetry message emitted by the Sensor Carrier.
///
/// ## Type
/// - `Periodic @ 20 Hz`
///
/// ## Direction
/// - **Emitted by:** `SensorCarrier`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Reports linear velocity in both the body-fixed and inertial NED frames.
///
/// ## Reaction
/// No action required.
///
/// ## Data Origin & Quality
/// Velocity is estimated by a Kalman filter fusing:
/// - Two GNSS receivers
/// - Two IMUs (accelerometer + gyroscope)
///
/// The **inertial NED frame** (North, East, Down) is the **native reference frame** for this data.
/// Velocity is provided directly by GNSS and is globally referenced and stable. GNSS velocity is
/// typically accurate to within ±0.1 m/s and valid up to ±500 m/s under normal conditions.
///
/// If GNSS data becomes unreliable, the filter will tend to dead-reckoning using inertial data.
///
/// The **body-fixed frame** (X forward, Y right, Z down) velocity is computed by rotating the
/// inertial velocity into the body frame using the estimated orientation quaternion. Since the
/// orientation estimate (especially yaw) is subject to small errors, this transformation introduces
/// small errors in the corresponding body-frame velocity components. This primarily affects the
/// horizontal components:
/// - `velocity_x`
/// - `velocity_y`
///
/// The vertical component (`velocity_z`) is not affected by errors in inertial yaw.
#[derive(Default)]
#[can_message(0x422)]
pub struct VelocityData {
    // --- Body-fixed frame (X: forward, Y: right, Z: down) ---
    /// Velocity along body X-axis (forward) in m/s. Range: ±500 m/s
    pub velocity_x: f32,
    /// Velocity along body Y-axis (right) in m/s. Range: ±500 m/s
    pub velocity_y: f32,
    /// Velocity along body Z-axis (down) in m/s. Range: ±500 m/s
    pub velocity_z: f32,

    // --- Inertial NED frame (North, East, Down) ---
    /// Velocity along North axis in m/s. Range: ±500 m/s
    pub velocity_north: f32,
    /// Velocity along East axis in m/s. Range: ±500 m/s
    pub velocity_east: f32,
    /// Velocity along Down axis in m/s. Range: ±500 m/s
    pub velocity_down: f32,
}

/// # `OrientationData`
/// Periodic telemetry message emitted by the Sensor Carrier.
///
/// ## Type
/// - `Periodic @ 40 Hz`
///
/// ## Direction
/// - **Emitted by:** `SensorCarrier`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Reports orientation as a quaternion rotating from the inertial NED zero attitude
/// to the body-fixed frame.
///
/// ## Reaction
/// No action required.
///
/// ## Data Origin & Quality
/// The orientation estimate is derived from two IMUs (accelerometer + gyroscope), fused with two
/// magnetometers and corrected for local magnetic declination to yield a stable yaw reference.
///
/// The quaternion describes a passive rotation from the **inertial NED frame** (North-East-Down)
/// to the **body-fixed frame** (X forward, Y right, Z down). This follows a right-handed,
/// 3-2-1 (ZYX) intrinsic rotation convention:
/// - Positive roll (X): Right canopy wingtip moves down
/// - Positive pitch (Y): Leading canopy edge moves up
/// - Positive yaw (Z): Heading turns right (clockwise when viewed from above)
///
/// The zero orientation is represented by the identity quaternion `(1, 0, 0, 0)`, where the body
/// frame is aligned with the NED frame. That is, the body X-axis points (True) North, Y-axis points
/// East, and Z-axis points down.
///
/// - **Roll** and **pitch** are well-constrained using the gravity vector, which can be directly
/// observed by the accelerometers.
/// - **Yaw** is initialized to zero at system startup. Yaw is observed by the magnetometers
/// and corrected by the gyroscopes. Under stable magnetic conditions, the yaw will converge
/// to true north over time. As soon as the yaw estimate has converged, it is expected to
/// remain stable.
///
/// The emitted quaternion is always **normalised**.
#[can_message(0x423)]
pub struct OrientationData {
    /// Normalised quaternion scalar component of the rotation from inertial NED to body-fixed frame.
    pub orientation_w: f32,
    /// Normalised quaternion x component of the rotation from inertial NED to body-fixed frame.
    pub orientation_x: f32,
    /// Normalised quaternion y component of the rotation from inertial NED to body-fixed frame.
    pub orientation_y: f32,
    /// Normalised quaternion z component of the rotation from inertial NED to body-fixed frame.
    pub orientation_z: f32,
}

impl Default for OrientationData {
    /// Default implementation for `OrientationData` is the identity quaternion,
    /// representing no rotation from the inertial NED frame to the body-fixed frame.
    fn default() -> Self {
        Self {
            orientation_w: 1.0,
            orientation_x: 0.0,
            orientation_y: 0.0,
            orientation_z: 0.0,
        }
    }
}

/// # `IgniterPressure`
/// Periodic igniter pressure readings emitted by the remote filling station.
///
/// ## Type
/// - `Periodic @ 40 Hz`
///
/// ## Direction
/// - **Emitted by:** `RemoteFillingStation`
/// - **Received by:** `EngineControlBoard`
///
/// ## Purpose
/// This is needed during the ignition sequence where the engine control board controlls the igniter
/// and thus needs to know its pressure and temperature for detecting successful ignition.
///
/// ## Reaction
/// No action required, but the ECU may decide to continue or abort based on the pressure.
#[can_message(0x440)]
pub struct IgniterPressure {
    /// The igniter pressure in barg.
    pub igniter_pressure: f32,
}

/// # `IgniterTemperature`
/// Periodic igniter temperature readings emitted by the remote filling station.
///
/// ## Type
/// - `Periodic @ 10 Hz`
///
/// ## Direction
/// - **Emitted by:** `RemoteFillingStation`
/// - **Received by:** `EngineControlBoard`
///
/// ## Purpose
/// This is needed during the ignition sequence where the engine control board controlls the igniter
/// and thus needs to know its pressure and temperature for detecting successful ignition.
///
/// ## Reaction
/// No action required, but the ECU may decide to continue or abort based on the temperature.
#[can_message(0x441)]
pub struct IgniterTemperature {
    /// The igniter temperature in °C.
    pub igniter_temperature: f32,
}

/// # `PressurizationLinePressure`
/// Periodic Pressure Data message emitted by the Fuel Control Board
///
/// ## Type
/// - `Periodic @ 20 Hz`
///
/// ## Direction
/// - **Emitted by:** `FuelControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Reports pressure of the Pressurization Line.
///
/// ## Reaction
/// No action required.
/// ## Data Origin
/// Data originates from a digital Keller Pressure Sensor, ranging from 0 to 400 barg.
/// The value will be `NaN` if communication with the sensor fails or the sensor is not working properly.
/// The value will be `inf` if the measuring range of the sensor is saturated.
#[can_message(0x450)]
pub struct PressurizationLinePressure {
    /// Pressurization Line Pressure in barg.
    pub prz_mnl_p: f32,
}

/// # `OxidizerTankPressure`
/// Periodic Pressure Data message emitted by the Oxidizer Control Board
///
/// ## Type
/// - `Periodic @ 20 Hz`
///
/// ## Direction
/// - **Emitted by:** `OxidizerControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Reports Oxidizer Tank Pressures.
/// These are mainly used as an input to the Control Loop running on the Oxidizer Control Board.
///
/// ## Reaction
/// No action required.
/// ## Data Origin
/// Data originates from two digital Keller Pressure Sensors, both ranging from 0 to 150 barg.
/// The value will be `NaN` if communication with the sensor fails or the sensor is not working properly.
/// The value will be `inf` if the measuring range of the sensor is saturated.
#[can_message(0x451)]
pub struct OxidizerTankPressure {
    /// Oxidizer Tank Pressure 1 in barg.
    pub oss_tnk_p1: f32,
    /// Oxidizer Tank Pressure 2 in barg.
    pub oss_tnk_p2: f32,
    /// Filtered Oxidizer Tank Pressure in barg.
    pub oss_tnk_p_filtered: f32,
}

/// # `FuelTankPressure`
/// Periodic Pressure Data message emitted by the Fuel Control Board
///
/// ## Type
/// - `Periodic @ 20 Hz`
///
/// ## Direction
/// - **Emitted by:** `FuelControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Reports Fuel Tank Pressures.
/// These are mainly used as an input to the Control Loop running on the Fuel Control Board.
///
/// ## Reaction
/// No action required.
/// ## Data Origin
/// Data originates from two digital Keller Pressure Sensors, both ranging from 0 to 150 barg.
/// The value will be `NaN` if communication with the sensor fails or the sensor is not working properly.
/// The value will be `inf` if the measuring range of the sensor is saturated.
#[can_message(0x452)]
pub struct FuelTankPressure {
    /// Fuel Tank Pressure 1 in barg.
    pub fss_tnk_p1: f32,
    /// Fuel Tank Pressure 2 in barg.
    pub fss_tnk_p2: f32,
    /// Filtered Fuel Tank Pressure in barg.
    pub fss_tnk_p_filtered: f32,
}

/// # `FuelTankTemperature`
/// Periodic Temperature Data emitted by the Fuel Control Board
///
/// ## Type
/// - `Periodic @ 10 Hz`
///
/// ## Direction
/// - **Emitted by:** `FuelControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Reports the Temperature of the Ethanol inside the Fuel Tank.
///
/// ## Reaction
/// No action required.
///
/// ## Data Origin
/// Data originates from a Type T Thermocouple connected to a high precision adc (ADS1120).
/// If the value read in firmware is outside the specific temperature range of [-200.0, 200.0] °C,
/// the value will be set to `NaN`.
#[can_message(0x453)]
pub struct FuelTankTemperature {
    /// Fuel Tank Temperature in °C.
    pub fss_tnk_t: f32,
}

/// # `OxidizerTankLevel`
/// Periodic Tank Level Data emitted by the Oxidizer Control Board
///
/// ## Type
/// - `Periodic @ 10 Hz`
///
/// ## Direction
/// - **Emitted by:** `OxidizerControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Reports the tank level of the Oxidizer Tank.
///
/// ## Reaction
/// No action required.
///
/// ## Data Origin
/// Data originates from a differential pressure sensor from Keller, measuring the differential pressure between the Oxidizer Tank Top and Bottom.
/// The sensor is calibrated to measure the fill level of the tank, with the calibration being done on the Oxidizer Control Board.
/// The value will be `NaN` if communication with the sensor fails or the sensor is not working properly.
/// If the measured vales of the sensor exceed the reference values for a full tank, the value will be set to `inf` by firmware
/// to indicate measurement saturation.
#[can_message(0x454)]
pub struct OxidizerTankLevel {
    /// Level in percent, 0 means empty, 100 means full, increments of 1%
    pub fill_fraction: u8,
}

/// # `EnginePressure`
/// Periodic telemetry message emitted by the Engine Control Board.
///
/// ## Type
/// - `Periodic @ 20 Hz`
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Reports various Pressures from the Engine.
/// This includes the Combustion Chamber Pressure, Injector Fuel Pressure and Injector Oxidizer Pressure.
///
/// ## Reaction
/// No action required.
///
/// ## Data Origin
/// Data originates from analog Trafag pressure sensors installed on the engine. They range from 0 to 60 barg
/// The value will be `NaN` if reading the sensor fails.
#[derive(Default)]
#[can_message(0x460)]
pub struct EnginePressure {
    /// Engine Chamber pressure in barg.
    pub eng_cc_p: f32,
    /// Injector Fuel pressure in barg.
    pub fss_inj_p: f32,
    /// Injector Oxidizer pressure in barg.
    pub oss_inj_p: f32,
}

/// # `EngineBayTemperature`
/// Periodic telemetry message emitted by the Engine Control Board.
///
/// ## Type
/// - `Periodic @ 10 Hz`
///
/// ## Direction
/// - **Emitted by:** `EngineControlBoard`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Reports various Data from Thermocouples in the Engine Bay.
/// This includes the OSS Runline Temperature and the OSS Tank Bottom Temperature.
///
/// ## Reaction
/// No action required.
///
/// ## Data Origin
/// Data originates from high precision ADCs which read the differential thermocouple voltage together with a thermistor from which the temperature is calculated.
/// Thermocouples installed can be either Type K or Type T.
/// If the value read in firmware is outside the specific temperature range of [-200.0, 200.0] °C,
/// the value will be set to `NaN`.
#[derive(Default)]
#[can_message(0x461)]
pub struct EngineBayTemperature {
    /// Oxidizer Runline temperature in °C.
    pub oss_rnl_t: f32,
    /// Oxidizer Tank Bottom temperature in °C.
    pub oss_tnk_t: f32,
}

/// Shared rail status structure for both 5V and 24V power rails.
#[derive(Clone, Debug, PartialEq, Default, Encode, Decode)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RailStatus {
    /// Voltage in volts.
    pub voltage: f32,
    /// Current in amps.
    pub current: f32,
    /// Power in watts.
    pub power: f32,
}

/// # `RailStatus5V`
/// Periodic telemetry message reporting the 5V rail parameters.
///
/// ## Type
/// - Periodic @ 10Hz
///
/// ## Direction
/// - **Emitted by:** `PowerBoard`
/// - **Received by:** unspecified
///
/// ## Purpose
/// Reports the measured voltage, current, and power on the 5V power rail.
///
/// ## Reaction
/// Receivers may log, display, or track these values. They may trigger warnings if any measurement is out of nominal range.
#[derive(Default)]
#[can_message(0x500)]
pub struct RailStatus5V {
    pub data: RailStatus,
}

/// # `RailStatus24V`
/// Periodic telemetry message reporting the 24V rail parameters.
///
/// ## Type
/// - Periodic @ 10Hz
///
/// ## Direction
/// - **Emitted by:** `PowerBoard`
/// - **Received by:** unspecified
///
/// ## Purpose
/// Reports the measured voltage, current, and power on the 24V power rail.
///
/// ## Reaction
/// Receivers may log, display, or track these values. They may trigger warnings if any measurement is out of nominal range.
#[derive(Default)]
#[can_message(0x501)]
pub struct RailStatus24V {
    pub data: RailStatus,
}

/// # `EnvironmentalData`
/// Periodic telemetry message emitted by the Sensor Carrier.
///
/// ## Type
/// - `Periodic @ 1 Hz`
///
/// ## Direction
/// - **Emitted by:** `SensorCarrier`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Provides low-rate environmental data in the form of temperature, humidity, and
/// smoothed pressure values.
///
/// ## Reaction
/// No action required.
#[derive(Default)]
#[can_message(0x510)]
pub struct EnvironmentalData {
    /// Temperature in Celsius.
    pub temperature: f32,
    /// Relative humidity in percent (0–100%).
    pub humidity: f32,
    /// Atmospheric pressure in hPa.
    ///
    /// **Note:** This data may differ from the `PressureData` message, which reports data
    /// at a much higher rate and with less filtering to improve timeliness.
    /// This data point should only be used for low-frequency applications.
    pub pressure: f32,
}

/// # `MagnetometerData`
/// Periodic telemetry message emitted by the Sensor Carrier.
///
/// ## Type
/// - `Periodic @ 10 Hz`
///
/// ## Direction
/// - **Emitted by:** `SensorCarrier`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Provides fused and filtered magnetic field strength in both the body-fixed
/// and inertial NED frames.
///
/// ## Reaction
/// No action required.
///
/// ## Data Origin & Quality
/// This data is derived from two magnetometers mounted closely and aligned in the same direction.
///
/// Hard and soft iron correction **has** been applied to this data to compensate for the
/// vehicle's own magnetic distortions.
///
/// The body-fixed frame values represent this corrected field. The inertial NED frame is
/// rotated from the body-fixed frame using the orientation quaternion. This transformation
/// introduces small conversion errors due to minor inaccuracies in the attitude estimate.
///
/// Small errors in the attitude estimate will primarily affect the horizontal components:
/// - `magnetic_field_north`
/// - `magnetic_field_east`
///
/// Note that `magnetic_field_down` is not affected by errors in inertial yaw.
#[derive(Default)]
#[can_message(0x520)]
pub struct MagnetometerData {
    // --- Body-fixed NED frame (X: forward, Y: right, Z: down) ---
    /// Magnetic field along body X-axis (forward) in μT. Range: ±4915.2 μT
    pub magnetic_field_x: f32,
    /// Magnetic field along body Y-axis (right) in μT. Range: ±4915.2 μT
    pub magnetic_field_y: f32,
    /// Magnetic field along body Z-axis (down) in μT. Range: ±4915.2 μT
    pub magnetic_field_z: f32,

    // --- Inertial NED frame (North, East, Down) ---
    /// Magnetic field along North axis in μT. Range: ±4915.2 μT
    pub magnetic_field_north: f32,
    /// Magnetic field along East axis in μT. Range: ±4915.2 μT
    pub magnetic_field_east: f32,
    /// Magnetic field along Down axis in μT. Range: ±4915.2 μT
    pub magnetic_field_down: f32,
}

/// # `PressureData`
/// Periodic telemetry message emitted by the Sensor Carrier.
///
/// ## Type
/// - `Periodic @ 40 Hz`
///
/// ## Direction
/// - **Emitted by:** `SensorCarrier`
/// - **Received by:** `unspecified`
///
/// ## Purpose
/// Reports filtered atmospheric pressure (scalar) for altitude estimation.
///
/// ## Reaction
/// No action required.
///
/// ## Note
/// This differs from `EnvironmentalData`, which is lower-rate and smoothed.
#[derive(Default)]
#[can_message(0x530)]
pub struct PressureData {
    /// Atmospheric pressure in hPa.
    pub pressure: f32,
}
