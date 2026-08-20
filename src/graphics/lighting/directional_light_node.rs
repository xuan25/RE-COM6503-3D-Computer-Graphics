//! Port of `legacy/graphics/lighting/DirectionalLightNode.java`.

use super::interfaces::{Directional, Lighting};
use crate::{
    gmaths::{Mat4, Vec3, Vec4},
    graphics::{
        material::Material,
        node::{BasicNode, CenterTransformable, Node},
    },
};

/// A directional light is a `BasicNode`; its direction is derived from the
/// node transform exactly like the Java source rather than stored separately.
pub struct DirectionalLightNode {
    node: BasicNode,
    material: Material,
}

impl DirectionalLightNode {
    pub fn new(name: impl Into<String>, material: Material) -> Self {
        Self {
            node: BasicNode::new(name),
            material,
        }
    }
    pub fn set_direction(&mut self, direction: Vec3) {
        let direction = direction.normalized();
        // BasicNode composes X, Y, then Z rotations.  Pick Y=0 and solve the
        // remaining X/Z rotations for its local down axis.
        let roll = direction.x.asin().to_degrees();
        let pitch = direction.z.atan2(-direction.y).to_degrees();
        self.node.set_center_rotation(pitch, 0., roll);
    }
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

impl Lighting for DirectionalLightNode {
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

impl Directional for DirectionalLightNode {
    fn direction(&self) -> Vec3 {
        // Preserve `DirectionalLightNode.getDirection()` exactly: the JOGL
        // implementation transforms a homogeneous local-down vector by the
        // inverse-transpose of the node transform and returns it unnormalised.
        Mat4::transpose(
            Mat4::inverse(self.node.center_transform())
                .expect("directional-light transform must be invertible"),
        )
        .multiply_vec4(Vec4::new(0., -1., 0., 1.))
        .to_vec3()
    }
}

impl CenterTransformable for DirectionalLightNode {
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

impl Node for DirectionalLightNode {
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
        "DirectionalLightNode"
    }
}
