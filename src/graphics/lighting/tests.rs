use super::{Directional, DirectionalLightNode, PointLightNode, Positional};
use crate::graphics::{
    material::Material,
    node::{BasicNode, CenterTransformable, MeshTransformable, Node, NodeLink},
};
use std::{cell::RefCell, rc::Rc};

#[test]
fn point_light_position_follows_parent_scene_transform() {
    let light = Rc::new(RefCell::new(PointLightNode::new(
        "Light",
        Material::default(),
        None,
    )));
    {
        let mut light = light.borrow_mut();
        light.set_mesh_translation(6., 0., 0.);
        light.set_center_translation(0., 12., 0.);
    }
    let mut group = BasicNode::new("Group");
    group.set_center_rotation(0., 90., 0.);
    group.add_child(Box::new(NodeLink::new(light.clone())));
    group.update();

    let position = light.borrow().position();
    assert!(position.x.abs() < 0.0001);
    assert!((position.y - 12.).abs() < 0.0001);
    assert!((position.z + 6.).abs() < 0.0001);
}

#[test]
fn directional_light_direction_follows_node_rotation() {
    let mut light = DirectionalLightNode::new("Daylight", Material::default());
    light.set_direction(crate::gmaths::Vec3::new(1., 0., 0.));
    light.update();
    let direction = light.direction();
    assert!((direction.x - 1.).abs() < 0.0001);
    assert!(direction.y.abs() < 0.0001);
    assert!(direction.z.abs() < 0.0001);
}
