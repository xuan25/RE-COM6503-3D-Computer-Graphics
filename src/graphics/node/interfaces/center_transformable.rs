use crate::gmaths::{Mat4, Vec3};

pub trait CenterTransformable {
    fn center_translation(&self) -> Vec3;
    fn set_center_translation(&mut self, x: f32, y: f32, z: f32);
    fn center_rotation(&self) -> Vec3;
    fn set_center_rotation(&mut self, x: f32, y: f32, z: f32);
    fn center_scale(&self) -> Vec3;
    fn set_center_scale(&mut self, x: f32, y: f32, z: f32);
    fn center_transform(&self) -> Mat4;
}
