//! Port of `legacy/graphics/lighting/PointLightNode.java`.

use super::interfaces::{Attenuated, Lighting, Positional};
use crate::{
    gmaths::{Mat4, Vec3, Vec4},
    graphics::{
        material::Material,
        model::Model,
        node::{CenterTransformable, MeshTransformable, ModelNode, Node},
    },
};

/// A point light is a retained `ModelNode`, just as in the JOGL implementation.
/// The same node renders the light-source mesh and supplies the authoritative
/// transform uploaded to the lighting shader.
pub struct PointLightNode {
    node: ModelNode,
    material: Material,
    attenuation_constant: f32,
    attenuation_linear: f32,
    attenuation_quadratic: f32,
}

impl PointLightNode {
    pub fn new(name: impl Into<String>, material: Material, model: Option<Model>) -> Self {
        let mut node = ModelNode::new(name, model);
        node.set_mesh_scale(0.3, 0.3, 0.3);
        Self {
            node,
            material,
            attenuation_constant: 1.0,
            attenuation_linear: 0.045,
            attenuation_quadratic: 0.0075,
        }
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.node
            .set_center_translation(position.x, position.y, position.z);
    }
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
        if let Some(model) = self.node.model_mut() {
            *model.material_mut() = material;
        }
    }
}

impl Lighting for PointLightNode {
    fn ambient(&self) -> Vec3 {
        self.material.ambient()
    }
    fn diffuse(&self) -> Vec3 {
        self.material.diffuse()
    }
    fn specular(&self) -> Vec3 {
        self.material.specular()
    }
}

impl Positional for PointLightNode {
    fn position(&self) -> Vec3 {
        self.node
            .mesh_transform()
            .multiply_vec4(Vec4::new(0., 0., 0., 1.))
            .to_vec3()
    }
}

impl Attenuated for PointLightNode {
    fn attenuation_constant(&self) -> f32 {
        self.attenuation_constant
    }
    fn set_attenuation_constant(&mut self, value: f32) {
        self.attenuation_constant = value;
    }
    fn attenuation_linear(&self) -> f32 {
        self.attenuation_linear
    }
    fn set_attenuation_linear(&mut self, value: f32) {
        self.attenuation_linear = value;
    }
    fn attenuation_quadratic(&self) -> f32 {
        self.attenuation_quadratic
    }
    fn set_attenuation_quadratic(&mut self, value: f32) {
        self.attenuation_quadratic = value;
    }
}

impl CenterTransformable for PointLightNode {
    fn center_translation(&self) -> Vec3 {
        self.node.center_translation()
    }
    fn set_center_translation(&mut self, x: f32, y: f32, z: f32) {
        self.node.set_center_translation(x, y, z)
    }
    fn center_rotation(&self) -> Vec3 {
        self.node.center_rotation()
    }
    fn set_center_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.node.set_center_rotation(x, y, z)
    }
    fn center_scale(&self) -> Vec3 {
        self.node.center_scale()
    }
    fn set_center_scale(&mut self, x: f32, y: f32, z: f32) {
        self.node.set_center_scale(x, y, z)
    }
    fn center_transform(&self) -> Mat4 {
        self.node.center_transform()
    }
}

impl MeshTransformable for PointLightNode {
    fn mesh_translation(&self) -> Vec3 {
        self.node.mesh_translation()
    }
    fn set_mesh_translation(&mut self, x: f32, y: f32, z: f32) {
        self.node.set_mesh_translation(x, y, z)
    }
    fn mesh_rotation(&self) -> Vec3 {
        self.node.mesh_rotation()
    }
    fn set_mesh_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.node.set_mesh_rotation(x, y, z)
    }
    fn mesh_scale(&self) -> Vec3 {
        self.node.mesh_scale()
    }
    fn set_mesh_scale(&mut self, x: f32, y: f32, z: f32) {
        self.node.set_mesh_scale(x, y, z)
    }
    fn mesh_transform(&self) -> Mat4 {
        self.node.mesh_transform()
    }
}

impl Node for PointLightNode {
    fn name(&self) -> String {
        self.node.name()
    }
    fn set_name(&mut self, name: String) {
        self.node.set_name(name)
    }
    fn update(&mut self) {
        self.node.update()
    }
    fn update_with_parent(&mut self, parent_transform: Mat4) {
        self.node.update_with_parent(parent_transform)
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.node.add_child(child)
    }
    fn remove_child_at(&mut self, index: usize) -> Option<Box<dyn Node>> {
        self.node.remove_child_at(index)
    }
    fn child_count(&self) -> usize {
        self.node.child_count()
    }
    unsafe fn render(&self) {
        unsafe { self.node.render() }
    }
    fn dispose(&mut self) {
        self.node.dispose()
    }
    fn build_string(&self, output: &mut String, last_children: &mut Vec<bool>) {
        self.node.build_string(output, last_children)
    }
    fn kind(&self) -> &'static str {
        "PointLightNode"
    }
}
