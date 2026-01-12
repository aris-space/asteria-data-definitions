use data_core::Message;
use datatypes::status::BuildInformationCommon;
use datatypes::units::HPa;
use gated_derive::gated_derive;

mod dptypes;
pub use dptypes::*;

#[derive(Message)]
#[gated_derive(data_core::CanMessage(feature = "can"))]
pub enum Message {
    // ======================== TX ========================
    /// # `SensorCarrierBuildInfo`
    /// Firmware build metadata emitted by the Sensor Carrier.
    ///
    /// ## Type
    /// - `Periodic @ 0.2 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `SensorCarrier`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Allows version tracking and verification of the Sensor Carrier firmware.
    ///
    /// ## Reaction
    /// Receivers may log or display the firmware version and build metadata.
    #[can(id = 0x612)]
    SensorCarrierBuildInfo(BuildInformationCommon),

    /// # `SensorCarrierStatus`
    /// Periodic status message emitted by the Sensor Carrier.
    ///
    /// ## Type
    /// - `Periodic @ 1 Hz`
    ///
    /// ## Direction
    /// - **Emitted by:** `SensorCarrier`
    /// - **Received by:** `unspecified`
    ///
    /// ## Purpose
    /// Communicates the runtime health of the Sensor Carrier.
    ///
    /// ## Reaction
    /// Receivers may log the data or use it for diagnostics and system health checks.
    #[can(id = 0x312)]
    SensorCarrierStatusMsg(SensorCarrierStatus),

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
    #[can(id = 0x420)]
    PositionDataMsg(PositionData),

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
    #[can(id = 0x421)]
    ImuDataMsg(ImuData),

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
    #[can(id = 0x422)]
    VelocityDataMsg(VelocityData),

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
    #[can(id = 0x423)]
    OrientationDataMsg(OrientationData),

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
    #[can(id = 0x510)]
    EnvironmentalDataMsg(EnvironmentalData),

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
    #[can(id = 0x520)]
    MagnetometerDataMsg(MagnetometerData),

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
    #[can(id = 0x530)]
    PressureData(HPa),
    // ======================== RX ========================
}
