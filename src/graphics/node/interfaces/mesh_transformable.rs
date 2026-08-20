use crate::gmaths::{Mat4, Vec3};

pub trait MeshTransformable {
    fn mesh_translation(&self) -> Vec3;
    fn set_mesh_translation(&mut self, x: f32, y: f32, z: f32);
    fn mesh_rotation(&self) -> Vec3;
    fn set_mesh_rotation(&mut self, x: f32, y: f32, z: f32);
    fn mesh_scale(&self) -> Vec3;
    fn set_mesh_scale(&mut self, x: f32, y: f32, z: f32);
    fn mesh_transform(&self) -> Mat4;
}
