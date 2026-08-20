//! Port of `legacy/graphics/lighting/SpotLightNode.java`.

use super::{
    PointLightNode,
    interfaces::{Attenuated, Directional, Lighting, Positional, Ranged},
};
use crate::{
    gmaths::{Mat4, Vec3, Vec4},
    graphics::{
        material::Material,
        model::Model,
        node::{CenterTransformable, MeshTransformable, Node},
    },
};

pub struct SpotLightNode {
    point_light: PointLightNode,
    cut_off_coefficient: f32,
    outer_cut_off_coefficient: f32,
}

impl SpotLightNode {
    pub fn new(name: impl Into<String>, material: Material, model: Model) -> Self {
        let mut result = Self {
            point_light: PointLightNode::new(name, material, Some(model)),
            cut_off_coefficient: 0.0,
            outer_cut_off_coefficient: 0.0,
        };
        result.set_cut_off(12.5);
        result.set_outer_cut_off(17.5);
        result
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.point_light.set_position(position);
    }
    pub fn set_direction(&mut self, direction: Vec3) {
        // Direction follows the retained mesh transform.  Retain this setter
        // for callers which explicitly orient a detached light node.
        let direction = direction.normalized();
        let roll = direction.x.asin().to_degrees();
        let pitch = direction.z.atan2(-direction.y).to_degrees();
        self.point_light.set_mesh_rotation(pitch, 0., roll);
    }
    pub fn set_material(&mut self, material: Material) {
        self.point_light.set_material(material);
    }
}

impl Lighting for SpotLightNode {
    fn ambient(&self) -> Vec3 {
        self.point_light.ambient()
    }
    fn diffuse(&self) -> Vec3 {
        self.point_light.diffuse()
    }
    fn specular(&self) -> Vec3 {
        self.point_light.specular()
    }
}

impl Positional for SpotLightNode {
    fn position(&self) -> Vec3 {
        self.point_light.position()
    }
}

impl Directional for SpotLightNode {
    fn direction(&self) -> Vec3 {
        // Match Java `SpotLightNode.getDirection()`: use the inverse
        // transpose and its original homogeneous vector, without a Rust-side
        // normalisation step.
        Mat4::transpose(
            Mat4::inverse(self.point_light.mesh_transform())
                .expect("spotlight mesh transform must be invertible"),
        )
        .multiply_vec4(Vec4::new(0., -1., 0., 1.))
        .to_vec3()
    }
}

impl Attenuated for SpotLightNode {
    fn attenuation_constant(&self) -> f32 {
        self.point_light.attenuation_constant()
    }
    fn set_attenuation_constant(&mut self, value: f32) {
        self.point_light.set_attenuation_constant(value);
    }
    fn attenuation_linear(&self) -> f32 {
        self.point_light.attenuation_linear()
    }
    fn set_attenuation_linear(&mut self, value: f32) {
        self.point_light.set_attenuation_linear(value);
    }
    fn attenuation_quadratic(&self) -> f32 {
        self.point_light.attenuation_quadratic()
    }
    fn set_attenuation_quadratic(&mut self, value: f32) {
        self.point_light.set_attenuation_quadratic(value);
    }
}

impl Ranged for SpotLightNode {
    fn cut_off(&self) -> f32 {
        // Java widens the float coefficient for `Math.acos` and
        // `Math.toDegrees`, then narrows the result back to float.
        (self.cut_off_coefficient as f64).acos().to_degrees() as f32
    }
    fn cut_off_coefficient(&self) -> f32 {
        self.cut_off_coefficient
    }
    fn set_cut_off(&mut self, degree: f32) {
        self.cut_off_coefficient = (degree as f64 * std::f64::consts::PI / 180.0).cos() as f32;
    }
    fn outer_cut_off(&self) -> f32 {
        (self.outer_cut_off_coefficient as f64).acos().to_degrees() as f32
    }
    fn outer_cut_off_coefficient(&self) -> f32 {
        self.outer_cut_off_coefficient
    }
    fn set_outer_cut_off(&mut self, degree: f32) {
        self.outer_cut_off_coefficient =
            (degree as f64 * std::f64::consts::PI / 180.0).cos() as f32;
    }
}

impl CenterTransformable for SpotLightNode {
    fn center_translation(&self) -> Vec3 {
        self.point_light.center_translation()
    }
    fn set_center_translation(&mut self, x: f32, y: f32, z: f32) {
        self.point_light.set_center_translation(x, y, z)
    }
    fn center_rotation(&self) -> Vec3 {
        self.point_light.center_rotation()
    }
    fn set_center_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.point_light.set_center_rotation(x, y, z)
    }
    fn center_scale(&self) -> Vec3 {
        self.point_light.center_scale()
    }
    fn set_center_scale(&mut self, x: f32, y: f32, z: f32) {
        self.point_light.set_center_scale(x, y, z)
    }
    fn center_transform(&self) -> Mat4 {
        self.point_light.center_transform()
    }
}

impl MeshTransformable for SpotLightNode {
    fn mesh_translation(&self) -> Vec3 {
        self.point_light.mesh_translation()
    }
    fn set_mesh_translation(&mut self, x: f32, y: f32, z: f32) {
        self.point_light.set_mesh_translation(x, y, z)
    }
    fn mesh_rotation(&self) -> Vec3 {
        self.point_light.mesh_rotation()
    }
    fn set_mesh_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.point_light.set_mesh_rotation(x, y, z)
    }
    fn mesh_scale(&self) -> Vec3 {
        self.point_light.mesh_scale()
    }
    fn set_mesh_scale(&mut self, x: f32, y: f32, z: f32) {
        self.point_light.set_mesh_scale(x, y, z)
    }
    fn mesh_transform(&self) -> Mat4 {
        self.point_light.mesh_transform()
    }
}

impl Node for SpotLightNode {
    fn name(&self) -> String {
        self.point_light.name()
    }
    fn set_name(&mut self, name: String) {
        self.point_light.set_name(name)
    }
    fn update(&mut self) {
        self.point_light.update()
    }
    fn update_with_parent(&mut self, parent_transform: Mat4) {
        self.point_light.update_with_parent(parent_transform)
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.point_light.add_child(child)
    }
    fn remove_child_at(&mut self, index: usize) -> Option<Box<dyn Node>> {
        self.point_light.remove_child_at(index)
    }
    fn child_count(&self) -> usize {
        self.point_light.child_count()
    }
    unsafe fn render(&self) {
        unsafe { self.point_light.render() }
    }
    fn dispose(&mut self) {
        self.point_light.dispose()
    }
    fn build_string(&self, output: &mut String, last_children: &mut Vec<bool>) {
        self.point_light.build_string(output, last_children)
    }
    fn kind(&self) -> &'static str {
        "SpotLightNode"
    }
}
