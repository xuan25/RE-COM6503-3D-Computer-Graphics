use std::fmt;

use crate::gmaths::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    pub const fn from_vec3(vector: Vec3) -> Self {
        Self::new(vector.x, vector.y, vector.z, 1.)
    }
    pub const fn from_vec3_with_w(vector: Vec3, w: f32) -> Self {
        Self::new(vector.x, vector.y, vector.z, w)
    }
    pub const fn to_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}
impl Default for Vec4 {
    /// Java's zero-argument constructor represents a point, not a direction.
    fn default() -> Self {
        Self::new(0., 0., 0., 1.)
    }
}
impl fmt::Display for Vec4 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({},{},{},{})", self.x, self.y, self.z, self.w)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec4;

    #[test]
    fn default_vector_is_a_homogeneous_point() {
        assert_eq!(Vec4::default().w, 1.);
    }
}
