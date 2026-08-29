mod blockpos;
mod point;
mod position;
mod velocity;

pub use blockpos::*;
pub use point::*;
pub use position::*;
pub use velocity::*;

macro_rules! impl_vector3_ops_with_extra {
    ($name:ident, $ty:ty, extra: $($extra:ident),* $(,)?) => {
        impl $name {
            /// Returns the x coordinate.
            pub const fn x(&self) -> $ty {
                self.x
            }

            /// Returns the y coordinate.
            pub const fn y(&self) -> $ty {
                self.y
            }

            /// Returns the z coordinate.
            pub const fn z(&self) -> $ty {
                self.z
            }

            /// Returns a copy with the x coordinate replaced.
            pub const fn with_x(&self, x: $ty) -> Self {
                Self {
                    x,
                    y: self.y,
                    z: self.z,
                    $($extra: self.$extra),*
                }
            }

            /// Returns a copy with the y coordinate replaced.
            pub const fn with_y(&self, y: $ty) -> Self {
                Self {
                    x: self.x,
                    y,
                    z: self.z,
                    $($extra: self.$extra),*
                }
            }

            /// Returns a copy with the z coordinate replaced.
            pub const fn with_z(&self, z: $ty) -> Self {
                Self {
                    x: self.x,
                    y: self.y,
                    z,
                    $($extra: self.$extra),*
                }
            }

            /// Returns a copy with `x`, `y` and `z` each negated.
            pub const fn neg(&self) -> Self {
                Self {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    $($extra: self.$extra),*
                }
            }

            /// Returns a copy with `x`, `y` and `z` each replaced by its
            /// absolute value.
            pub fn abs(&self) -> Self {
                Self {
                    x: self.x.abs(),
                    y: self.y.abs(),
                    z: self.z.abs(),
                    $($extra: self.$extra),*
                }
            }

            pub fn min(&self, other: impl Into<Self>) -> Self {
                let other = other.into();
                Self {
                    x: if self.x < other.x { self.x } else { other.x },
                    y: if self.y < other.y { self.y } else { other.y },
                    z: if self.z < other.z { self.z } else { other.z },
                    $($extra: self.$extra),*
                }
            }

            pub fn max(&self, other: impl Into<Self>) -> Self {
                let other = other.into();
                Self {
                    x: if self.x > other.x { self.x } else { other.x },
                    y: if self.y > other.y { self.y } else { other.y },
                    z: if self.z > other.z { self.z } else { other.z },
                    $($extra: self.$extra),*
                }
            }
        }

        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                Self {
                    x: value,
                    y: value,
                    z: value,
                    $($extra: Default::default()),*
                }
            }
        }

        impl<T> std::ops::Add<T> for $name
        where
            T: Into<Self>,
        {
            type Output = Self;

            fn add(self, rhs: T) -> Self {
                let rhs = rhs.into();
                Self {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                    $($extra: self.$extra),*
                }
            }
        }

        impl<T> std::ops::Sub<T> for $name
        where
            T: Into<Self>,
        {
            type Output = Self;

            fn sub(self, rhs: T) -> Self {
                let rhs = rhs.into();
                Self {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                    $($extra: self.$extra),*
                }
            }
        }

        impl<T> std::ops::Mul<T> for $name
        where
            T: Into<Self>,
        {
            type Output = Self;

            fn mul(self, rhs: T) -> Self {
                let rhs = rhs.into();
                Self {
                    x: self.x * rhs.x,
                    y: self.y * rhs.y,
                    z: self.z * rhs.z,
                    $($extra: self.$extra),*
                }
            }
        }

        impl<T> std::ops::Div<T> for $name
        where
            T: Into<Self>,
        {
            type Output = Self;

            fn div(self, rhs: T) -> Self {
                let rhs = rhs.into();
                Self {
                    x: self.x / rhs.x,
                    y: self.y / rhs.y,
                    z: self.z / rhs.z,
                    $($extra: self.$extra),*
                }
            }
        }

        impl std::ops::Neg for $name {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    $($extra: self.$extra),*
                }
            }
        }

        impl<T> std::ops::AddAssign<T> for $name
        where
            T: Into<Self>,
        {
            fn add_assign(&mut self, rhs: T) {
                *self = *self + rhs.into();
            }
        }

        impl<T> std::ops::SubAssign<T> for $name
        where
            T: Into<Self>,
        {
            fn sub_assign(&mut self, rhs: T) {
                *self = *self - rhs.into();
            }
        }

        impl<T> std::ops::MulAssign<T> for $name
        where
            T: Into<Self>,
        {
            fn mul_assign(&mut self, rhs: T) {
                *self = *self * rhs.into();
            }
        }

        impl<T> std::ops::DivAssign<T> for $name
        where
            T: Into<Self>,
        {
            fn div_assign(&mut self, rhs: T) {
                *self = *self / rhs.into();
            }
        }
    };
}

macro_rules! impl_vector3_ops {
    ($name:ident, $ty:ty) => {
        $crate::util::vec3::impl_vector3_ops_with_extra!($name, $ty, extra:);
    };
}

macro_rules! impl_vector3_dot_ops_with_extra {
    ($name:ident, $ty:ty, extra: $($extra:ident),* $(,)?) => {
        impl $name {
            /// Returns the dot product of `self` and `other`.
            pub const fn dot(&self, other: Self) -> $ty {
                self.x * other.x + self.y * other.y + self.z * other.z
            }

            pub const fn length_squared(&self) -> $ty {
                self.dot(*self)
            }

            pub fn length(&self) -> f64 {
                (self.length_squared() as f64).sqrt()
            }

            pub fn distance_squared(&self, other: Self) -> $ty {
                (*self - other).length_squared()
            }

            /// Returns the distance between `self` and `other`.
            pub fn distance(&self, other: Self) -> f64 {
                (self.distance_squared(other) as f64).sqrt()
            }

            /// Returns the cross product of `self` and `other`.
            pub const fn cross(&self, other: Self) -> Self {
                Self {
                    x: self.y * other.z - other.y * self.z,
                    y: self.z * other.x - other.z * self.x,
                    z: self.x * other.y - other.x * self.y,
                    $($extra: self.$extra),*
                }
            }

            /// Returns this vector scaled to a length of `1.0`.
            pub fn normalize(&self) -> Self {
                let length = self.length();
                if length == 0.0 {
                    return *self;
                }
                *self / length
            }

            pub fn lerp(&self, other: Self, alpha: f64) -> Self {
                *self + (other - *self) * alpha
            }
        }
    };
}

macro_rules! impl_vector3_dot_ops {
    ($name:ident, $ty:ty) => {
        $crate::util::vec3::impl_vector3_dot_ops_with_extra!($name, $ty, extra:);
    };
}

pub(crate) use impl_vector3_dot_ops;
pub(crate) use impl_vector3_dot_ops_with_extra;
pub(crate) use impl_vector3_ops;
pub(crate) use impl_vector3_ops_with_extra;
