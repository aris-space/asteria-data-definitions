use datatypes::status::{SensorStatus, StatusCommonMessage};
use datatypes::units::{Celsius, HPa};
use gated_derive::gated_derive;

/// Represents the online/offline status of all sensors on the Sensor Carrier board.
/// Each field corresponds to a specific sensor instance.
#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SensorsHealth {
    /// Status of barometer on I2C2 (bus 1: PA9 = SCL, PA8 = SDA)
    pub barometer_bus_1: SensorStatus,
    /// Status of barometer on I2C3 (bus 2: PC8 = SCL, PC9 = SDA)
    pub barometer_bus_2: SensorStatus,
    /// Status of `SHT4x` temperature/humidity sensor on I2C2 (bus 1)
    pub dht_bus_1: SensorStatus,
    /// Status of `SHT4x` temperature/humidity sensor on I2C3 (bus 2)
    pub dht_bus_2: SensorStatus,
    /// Status of magnetometer on I2C2 (bus 1), INT on PA0
    pub magnetometer_bus_1: SensorStatus,
    /// Status of magnetometer on I2C3 (bus 2), INT on PA1
    pub magnetometer_bus_2: SensorStatus,
    /// Status of IMU (LSM6DSO32) on SPI3 (PB3–PB6, INT on PC13)
    pub imu_1: SensorStatus,
    /// Status of IMU (LSM6DSO32) on SPI1 (PA4–PA7, INT on PC14)
    pub imu_2: SensorStatus,
    /// Status of GPS module on USART1 (PC5 = TX, PC4 = RX)
    pub gps_1: SensorStatus,
    /// Status of GPS module on USART3 (PC11 = TX, PC10 = RX)
    pub gps_2: SensorStatus,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SensorCarrierStatus {
    pub common: StatusCommonMessage,
    pub sensor_health: SensorsHealth,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Clone, Debug, PartialEq, Default)]
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

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Clone, Debug, PartialEq, Default)]
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

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Clone, Debug, PartialEq, Default)]
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

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Clone, Debug, PartialEq)]
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

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct EnvironmentalData {
    /// Temperature in Celsius.
    pub temperature: Celsius,
    /// Relative humidity in percent (0–100%).
    pub humidity: f32,
    /// Atmospheric pressure in hPa.
    ///
    /// **Note:** This data may differ from the `PressureData` message, which reports data
    /// at a much higher rate and with less filtering to improve timeliness.
    /// This data point should only be used for low-frequency applications.
    pub pressure: HPa,
}

#[gated_derive(serde::Serialize, serde::Deserialize, defmt::Format)]
#[gated_derive(bincode::Encode, bincode::Decode, arbitrary::Arbitrary)]
#[derive(Clone, Debug, PartialEq, Default)]
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
