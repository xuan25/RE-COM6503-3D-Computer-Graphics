//! Port of `legacy/graphics/node/ModelNode.java`.

#![allow(unsafe_op_in_unsafe_fn)]

use super::{
    BasicNode,
    interfaces::{CenterTransformable, MeshTransformable, Node},
};
use crate::{
    gmaths::{Mat4, Vec3, mat4_transform},
    graphics::{interfaces::TransformRenderable, model::Model},
};

pub struct ModelNode {
    base: BasicNode,
    model: Option<Model>,
    mesh_translation: Vec3,
    mesh_rotation: Vec3,
    mesh_scale: Vec3,
    mesh_transform: Mat4,
}

impl ModelNode {
    pub fn new(name: impl Into<String>, model: Option<Model>) -> Self {
        Self {
            base: BasicNode::new(name),
            model,
            mesh_translation: Vec3::default(),
            mesh_rotation: Vec3::default(),
            mesh_scale: Vec3::new(1.0, 1.0, 1.0),
            mesh_transform: Mat4::identity(),
        }
    }

    pub fn model(&self) -> Option<&Model> {
        self.model.as_ref()
    }
    pub fn model_mut(&mut self) -> Option<&mut Model> {
        self.model.as_mut()
    }
    pub fn set_model(&mut self, model: Option<Model>) {
        self.model = model;
    }

    fn update_model_transform(&mut self, parent_transform: Mat4) {
        self.base.update_center(parent_transform);
        let mut transform = self.base.center_transform;
        transform = Mat4::multiply(transform, mat4_transform::translate(self.mesh_translation));
        transform = Mat4::multiply(transform, mat4_transform::rotate_x(self.mesh_rotation.x));
        transform = Mat4::multiply(transform, mat4_transform::rotate_y(self.mesh_rotation.y));
        transform = Mat4::multiply(transform, mat4_transform::rotate_z(self.mesh_rotation.z));
        self.mesh_transform = Mat4::multiply(transform, mat4_transform::scale(self.mesh_scale));
    }
}

impl CenterTransformable for ModelNode {
    fn center_translation(&self) -> Vec3 {
        self.base.center_translation()
    }
    fn set_center_translation(&mut self, x: f32, y: f32, z: f32) {
        self.base.set_center_translation(x, y, z);
    }
    fn center_rotation(&self) -> Vec3 {
        self.base.center_rotation()
    }
    fn set_center_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.base.set_center_rotation(x, y, z);
    }
    fn center_scale(&self) -> Vec3 {
        self.base.center_scale()
    }
    fn set_center_scale(&mut self, x: f32, y: f32, z: f32) {
        self.base.set_center_scale(x, y, z);
    }
    fn center_transform(&self) -> Mat4 {
        self.base.center_transform()
    }
}

impl MeshTransformable for ModelNode {
    fn mesh_translation(&self) -> Vec3 {
        self.mesh_translation
    }
    fn set_mesh_translation(&mut self, x: f32, y: f32, z: f32) {
        self.mesh_translation = Vec3::new(x, y, z);
    }
    fn mesh_rotation(&self) -> Vec3 {
        self.mesh_rotation
    }
    fn set_mesh_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.mesh_rotation = Vec3::new(x, y, z);
    }
    fn mesh_scale(&self) -> Vec3 {
        self.mesh_scale
    }
    fn set_mesh_scale(&mut self, x: f32, y: f32, z: f32) {
        self.mesh_scale = Vec3::new(x, y, z);
    }
    fn mesh_transform(&self) -> Mat4 {
        self.mesh_transform
    }
}

impl Node for ModelNode {
    fn name(&self) -> String {
        self.base.name()
    }
    fn set_name(&mut self, name: String) {
        self.base.set_name(name);
    }
    fn update(&mut self) {
        self.update_model_transform(self.base.parent_transform);
    }
    fn update_with_parent(&mut self, parent_transform: Mat4) {
        self.update_model_transform(parent_transform);
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.base.add_child(child);
    }
    fn remove_child_at(&mut self, index: usize) -> Option<Box<dyn Node>> {
        self.base.remove_child_at(index)
    }
    fn child_count(&self) -> usize {
        self.base.child_count()
    }
    unsafe fn render(&self) {
        if let Some(model) = &self.model {
            model.render_with_transform(self.mesh_transform);
        }
        self.base.render_children();
    }
    fn dispose(&mut self) {
        self.base.dispose_children();
    }
    fn build_string(&self, output: &mut String, last_children: &mut Vec<bool>) {
        let indent = last_children.len();
        output.push_str(&format!("[{} - ModelNode]\n", self.name()));
        last_children.push(false);
        for (index, child) in self.base.children.iter().enumerate() {
            for is_last in last_children.iter().take(indent) {
                output.push_str(if *is_last { "  " } else { "│ " });
            }
            let is_last = index + 1 == self.base.children.len();
            output.push_str(if is_last { "└─" } else { "├─" });
            if is_last {
                last_children[indent] = true;
            }
            child.build_string(output, last_children);
        }
        last_children.pop();
    }
    fn kind(&self) -> &'static str {
        "ModelNode"
    }
}
