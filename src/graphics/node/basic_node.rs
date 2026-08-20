//! Port of `legacy/graphics/node/BasicNode.java`.

#![allow(unsafe_op_in_unsafe_fn)]

use super::interfaces::{CenterTransformable, Node};
use crate::gmaths::{Mat4, Vec3, mat4_transform};

pub struct BasicNode {
    pub(crate) name: String,
    pub(crate) children: Vec<Box<dyn Node>>,
    pub(crate) parent_transform: Mat4,
    pub(crate) center_translation: Vec3,
    pub(crate) center_rotation: Vec3,
    pub(crate) center_scale: Vec3,
    pub(crate) center_transform: Mat4,
}

impl BasicNode {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            children: Vec::new(),
            parent_transform: Mat4::identity(),
            center_translation: Vec3::default(),
            center_rotation: Vec3::default(),
            center_scale: Vec3::new(1.0, 1.0, 1.0),
            center_transform: Mat4::identity(),
        }
    }

    pub(crate) fn update_center(&mut self, parent_transform: Mat4) {
        // Java `BasicNode.update(Mat4)` uses its argument for this traversal
        // only; it does not overwrite the node's retained `parentTransform`.
        // Consequently a later parameterless `update()` starts from the same
        // constructor-time parent transform (the identity matrix).
        let mut transform = parent_transform;
        transform = Mat4::multiply(
            transform,
            mat4_transform::translate(self.center_translation),
        );
        transform = Mat4::multiply(transform, mat4_transform::rotate_x(self.center_rotation.x));
        transform = Mat4::multiply(transform, mat4_transform::rotate_y(self.center_rotation.y));
        transform = Mat4::multiply(transform, mat4_transform::rotate_z(self.center_rotation.z));
        transform = Mat4::multiply(transform, mat4_transform::scale(self.center_scale));
        self.center_transform = transform;
        for child in &mut self.children {
            child.update_with_parent(transform);
        }
    }

    pub(crate) unsafe fn render_children(&self) {
        for child in &self.children {
            child.render();
        }
    }

    pub(crate) fn dispose_children(&mut self) {
        for child in &mut self.children {
            child.dispose();
        }
    }

    pub(crate) fn build_children(&self, output: &mut String, last_children: &mut Vec<bool>) {
        let indent = last_children.len();
        output.push_str(&format!("[{} - {}]\n", self.name, self.kind()));
        last_children.push(false);
        for (index, child) in self.children.iter().enumerate() {
            for is_last in last_children.iter().take(indent) {
                output.push_str(if *is_last { "  " } else { "│ " });
            }
            let is_last = index + 1 == self.children.len();
            output.push_str(if is_last { "└─" } else { "├─" });
            if is_last {
                last_children[indent] = true;
            }
            child.build_string(output, last_children);
        }
        last_children.pop();
    }

    pub fn hierarchy_string(&self) -> String {
        let mut output = String::new();
        self.build_string(&mut output, &mut Vec::new());
        output
    }
}

impl CenterTransformable for BasicNode {
    fn center_translation(&self) -> Vec3 {
        self.center_translation
    }
    fn set_center_translation(&mut self, x: f32, y: f32, z: f32) {
        self.center_translation = Vec3::new(x, y, z);
    }
    fn center_rotation(&self) -> Vec3 {
        self.center_rotation
    }
    fn set_center_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.center_rotation = Vec3::new(x, y, z);
    }
    fn center_scale(&self) -> Vec3 {
        self.center_scale
    }
    fn set_center_scale(&mut self, x: f32, y: f32, z: f32) {
        self.center_scale = Vec3::new(x, y, z);
    }
    fn center_transform(&self) -> Mat4 {
        self.center_transform
    }
}

impl Node for BasicNode {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn update(&mut self) {
        self.update_center(self.parent_transform);
    }
    fn update_with_parent(&mut self, parent_transform: Mat4) {
        self.update_center(parent_transform);
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.push(child);
    }
    fn remove_child_at(&mut self, index: usize) -> Option<Box<dyn Node>> {
        (index < self.children.len()).then(|| self.children.remove(index))
    }
    fn child_count(&self) -> usize {
        self.children.len()
    }
    unsafe fn render(&self) {
        self.render_children();
    }
    fn dispose(&mut self) {
        self.dispose_children();
    }
    fn build_string(&self, output: &mut String, last_children: &mut Vec<bool>) {
        self.build_children(output, last_children);
    }
    fn kind(&self) -> &'static str {
        "BasicNode"
    }
}
