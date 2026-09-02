use super::{
    BlockPosition, Point, Velocity, impl_vector3_dot_ops_with_extra, impl_vector3_ops_with_extra,
};
use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
}

impl Position {
    pub const ZERO: Position = Position::new(0., 0., 0., 0., 0.);

    pub const fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Self {
        let yaw = Self::fix_yaw(yaw);
        Self {
            x,
            y,
            z,
            yaw,
            pitch,
        }
    }

    /// Returns the yaw in degrees.
    pub const fn yaw(&self) -> f32 {
        self.yaw
    }

    /// Returns the pitch in degrees.
    pub const fn pitch(&self) -> f32 {
        self.pitch
    }

    /// Returns a copy with the yaw replaced.
    pub const fn with_yaw(&self, yaw: f32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
            yaw,
            pitch: self.pitch,
        }
    }

    /// Returns a copy with the pitch replaced.
    pub const fn with_pitch(&self, pitch: f32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
            yaw: self.yaw,
            pitch,
        }
    }

    pub const fn relative_to(&self, base: Self, flags: &TeleportFlags) -> Self {
        let x = if flags.contains(TeleportFlags::X) {
            base.x + self.x
        } else {
            self.x
        };
        let y = if flags.contains(TeleportFlags::Y) {
            base.y + self.y
        } else {
            self.y
        };
        let z = if flags.contains(TeleportFlags::Z) {
            base.z + self.z
        } else {
            self.z
        };
        let yaw = if flags.contains(TeleportFlags::YAW) {
            base.yaw + self.yaw
        } else {
            self.yaw
        };
        let pitch = if flags.contains(TeleportFlags::PITCH) {
            base.pitch + self.pitch
        } else {
            self.pitch
        };

        Self::new(x, y, z, yaw, pitch)
    }

    pub const fn clamp_max(this: Self) -> Self {
        const MAX_COORDINATE: f64 = 2_000_000_000.;

        if this.x.is_nan() || this.y.is_nan() || this.z.is_nan() {
            return Self {
                x: 0.,
                y: 0.,
                z: 0.,
                yaw: this.yaw,
                pitch: this.pitch,
            };
        }

        Self {
            x: this.x.clamp(-MAX_COORDINATE, MAX_COORDINATE),
            y: this.y.clamp(-MAX_COORDINATE, MAX_COORDINATE),
            z: this.z.clamp(-MAX_COORDINATE, MAX_COORDINATE),
            yaw: this.yaw,
            pitch: this.pitch,
        }
    }

    const fn fix_yaw(yaw: f32) -> f32 {
        let mut yaw = yaw % 360.0;
        if yaw < -180.0 {
            yaw += 360.0;
        } else if yaw > 180.0 {
            yaw -= 360.0;
        }
        yaw
    }
}

impl_vector3_ops_with_extra!(Position, f64, extra: yaw, pitch);
impl_vector3_dot_ops_with_extra!(Position, f64, extra: yaw, pitch);

impl<A> From<[A; 3]> for Position
where
    A: Into<f64>,
{
    fn from(value: [A; 3]) -> Self {
        let [x, y, z] = value;
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            ..Position::ZERO
        }
    }
}

impl<A> From<[A; 5]> for Position
where
    A: Into<f64>,
{
    fn from(value: [A; 5]) -> Self {
        let [x, y, z, yaw, pitch] = value;
        Self::new(
            x.into(),
            y.into(),
            z.into(),
            yaw.into() as f32,
            pitch.into() as f32,
        )
    }
}

impl From<BlockPosition> for Position {
    fn from(value: BlockPosition) -> Self {
        Self {
            x: value.x() as f64,
            y: value.y() as f64,
            z: value.z() as f64,
            yaw: 0.,
            pitch: 0.,
        }
    }
}

impl From<Point> for Position {
    fn from(value: Point) -> Self {
        Self {
            x: value.x(),
            y: value.y(),
            z: value.z(),
            yaw: 0.,
            pitch: 0.,
        }
    }
}

impl From<Velocity> for Position {
    fn from(value: Velocity) -> Self {
        Self {
            x: value.x(),
            y: value.y(),
            z: value.z(),
            yaw: 0.,
            pitch: 0.,
        }
    }
}

bitflags! {
    /// Indicates whether a value is treated as a delta or as an absolute
    /// value.
    ///
    /// If a flag is not set, that axis is absolute, even if its value is `0.0`.
    /// To leave an axis unchanged, set its flag and pass `0.0` as the delta.
    ///
    /// # Examples
    ///
    /// Individual flags can be combined to mark several axes relative at once.
    /// ```
    /// # use shulkr::util::{Position, TeleportFlags};
    /// let p = Position::from([100, 64, 100]);
    /// let v = Position::from([8, 70, 9]);
    /// assert_eq!(
    ///     v.relative_to(p, &(TeleportFlags::X | TeleportFlags::Z)),
    ///     Position::from([108, 70, 109])
    /// );
    /// ```
    ///
    /// With no flags set, every axis is absolute, so `v` comes through unchanged.
    /// ```
    /// # use shulkr::util::{Position, TeleportFlags};
    /// let p = Position::from([100, 64, 100]);
    /// let v = Position::from([5, 6, 7]);
    /// assert_eq!(v.relative_to(p, &TeleportFlags::NONE), v);
    /// ```
    #[derive(Debug, Clone, Copy)]
    pub struct TeleportFlags: i32 {
        /// Every value is absolute, with no deltas applied.
        ///
        /// ```
        /// # use shulkr::util::{Position, TeleportFlags};
        /// let p = Position::from([100, 64, 100]);
        /// let v = Position::from([5, 6, 7]);
        /// assert_eq!(
        ///     v.relative_to(p, &TeleportFlags::NONE),
        ///     Position::from([5, 6, 7])
        /// );
        /// ```
        const NONE = 0x0000;

        /// The `x` value is added to the current x as a delta, rather than used as an absolute coordinate.
        ///
        /// ```
        /// # use shulkr::util::{Position, TeleportFlags};
        /// let p = Position::from([100, 64, 100]);
        /// let v = Position::from([5, 0, 0]);
        /// assert_eq!(
        ///     v.relative_to(p, &TeleportFlags::X),
        ///     Position::from([105, 0, 0])
        /// );
        /// ```
        const X = 0x0001;
        /// The `y` value is added to the current y as a delta, rather than used as an absolute coordinate.
        ///
        /// ```
        /// # use shulkr::util::{Position, TeleportFlags};
        /// let p = Position::from([100, 64, 100]);
        /// let v = Position::from([0, 2, 0]);
        /// assert_eq!(
        ///     v.relative_to(p, &TeleportFlags::Y),
        ///     Position::from([0, 66, 0])
        /// );
        /// ```
        const Y = 0x0002;
        /// The `z` value is added to the current z as a delta, rather than used as an absolute coordinate.
        ///
        /// ```
        /// # use shulkr::util::{Position, TeleportFlags};
        /// let p = Position::from([100, 64, 100]);
        /// let v = Position::from([0, 0, 5]);
        /// assert_eq!(
        ///     v.relative_to(p, &TeleportFlags::Z),
        ///     Position::from([0, 0, 105])
        /// );
        /// ```
        const Z = 0x0004;
        /// The `yaw` value is added to the current yaw as a delta, rather than used as an absolute facing angle.
        ///
        /// ```
        /// # use shulkr::util::{Position, TeleportFlags};
        /// let p = Position::new(100.0, 64.0, 200.0, 90.0, 0.0);
        /// let v = Position::new(0.0, 0.0, 0.0, 10.0, 0.0);
        /// assert_eq!(
        ///     v.relative_to(p, &TeleportFlags::YAW),
        ///     Position::new(0.0, 0.0, 0.0, 100.0, 0.0)
        /// );
        /// ```
        const YAW        = 0x0008;
        /// The `pitch` value is added to the current pitch as a delta, rather than used as an absolute facing angle.
        ///
        /// ```
        /// # use shulkr::util::{Position, TeleportFlags};
        /// let p = Position::new(100.0, 64.0, 200.0, 0.0, 0.0);
        /// let v = Position::new(0.0, 0.0, 0.0, 0.0, -10.0);
        /// assert_eq!(
        ///     v.relative_to(p, &TeleportFlags::PITCH),
        ///     Position::new(0.0, 0.0, 0.0, 0.0, -10.0)
        /// );
        /// ```
        const PITCH      = 0x0010;

        const VELOCITY_X = 0x0020;
        const VELOCITY_Y = 0x0040;
        const VELOCITY_Z = 0x0080;

        const ROTATE     = 0x0100;
    }
}
