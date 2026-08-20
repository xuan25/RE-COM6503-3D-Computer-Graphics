use std::{
    fmt,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign},
};
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn length(self) -> f32 {
        self.magnitude()
    }
    pub fn magnitude(self) -> f32 {
        // Java's `Math.sqrt` operates on a widened double before the return
        // value is narrowed to float.
        (self.dot(self) as f64).sqrt() as f32
    }
    pub fn normalize(&mut self) {
        let n = self.length();
        // Java `Vec3.normalize()` has no zero-length guard.  Retain its IEEE
        // floating-point behaviour rather than silently changing a degenerate
        // vector into a valid zero vector.
        self.x /= n;
        self.y /= n;
        self.z /= n
    }
    pub fn normalized(mut self) -> Self {
        self.normalize();
        self
    }
    pub fn dot(self, r: Self) -> f32 {
        self.x * r.x + self.y * r.y + self.z * r.z
    }
    pub fn cross(self, r: Self) -> Self {
        Self::new(
            self.y * r.z - self.z * r.y,
            self.z * r.x - self.x * r.z,
            self.x * r.y - self.y * r.x,
        )
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(self, r: Self) -> Self {
        Self::new(self.x + r.x, self.y + r.y, self.z + r.z)
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, r: Self) -> Self {
        Self::new(self.x - r.x, self.y - r.y, self.z - r.z)
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, r: f32) -> Self {
        Self::new(self.x * r, self.y * r, self.z * r)
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, r: f32) -> Self {
        Self::new(self.x / r, self.y / r, self.z / r)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, r: Self) {
        *self = *self + r
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, r: Self) {
        *self = *self - r
    }
}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, r: f32) {
        *self = *self * r
    }
}
impl fmt::Display for Vec3 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({},{},{})", self.x, self.y, self.z)
    }
}
