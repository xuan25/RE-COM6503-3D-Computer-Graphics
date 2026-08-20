//! Port of `legacy/scene/component/Robot.java`.

use super::{SceneBuilder, SceneResourceError, interfaces::Component};
use crate::graphics::node::{
    BasicNode, CenterTransformable, MeshTransformable, ModelNode, Node, NodeLink,
};
use std::{cell::RefCell, rc::Rc};

type ModelRef = Rc<RefCell<ModelNode>>;
type BasicNodeRef = Rc<RefCell<BasicNode>>;
fn model_node(
    b: &SceneBuilder,
    name: &str,
    mesh: &str,
    texture: &str,
    specular: &str,
) -> Result<ModelRef, SceneResourceError> {
    Ok(Rc::new(RefCell::new(ModelNode::new(
        name,
        Some(b.model(b.default_material(), mesh, &[texture, specular])?),
    ))))
}
pub struct Robot {
    // Keep the same retained component graph as `Robot.java`, rather than
    // retaining only nodes the animator currently mutates.
    root: BasicNode,
    wheel: ModelRef,
    body: ModelRef,
    head_joint: ModelRef,
    head: ModelRef,
    eye: ModelRef,
    antenna_stand: ModelRef,
    antenna_joint: ModelRef,
    antenna: ModelRef,
    sub_antenna_stand_rotate: BasicNodeRef,
    sub_antenna_stand: ModelRef,
    sub_antenna_joint: ModelRef,
    sub_antenna: ModelRef,
}
impl Robot {
    pub fn new(b: &SceneBuilder) -> Result<Self, SceneResourceError> {
        let mut root = BasicNode::new("Robot");
        let wheel = model_node(b, "Wheel", "sphere", "robot_wheel", "robot_wheel_spec")?;
        {
            let mut n = wheel.borrow_mut();
            n.set_mesh_rotation(0., 0., 90.);
            n.set_mesh_scale(1.6, 0.5, 1.6);
            n.set_center_translation(0., 0.8, 0.);
        }
        let body = model_node(b, "Body", "cube", "robot_primary", "robot_primary_spec")?;
        {
            let mut n = body.borrow_mut();
            n.set_mesh_scale(2., 3., 1.);
            n.set_center_translation(0., 1.5, 0.);
        }
        let head_joint = model_node(
            b,
            "HeadJoint",
            "sphere",
            "robot_secondary",
            "robot_secondary_spec",
        )?;
        {
            let mut n = head_joint.borrow_mut();
            n.set_mesh_scale(0.8, 0.8, 0.8);
            n.set_center_translation(0., 1.6, 0.);
        }
        let head = model_node(b, "Head", "sphere", "robot_primary", "robot_primary_spec")?;
        {
            let mut n = head.borrow_mut();
            n.set_mesh_scale(3., 1., 3.);
            n.set_center_translation(0., 0.6, 0.);
        }
        let eye = model_node(b, "Eye", "sphere", "robot_accent", "robot_accent_spec")?;
        {
            let mut n = eye.borrow_mut();
            n.set_mesh_scale(0.75, 0.75, 0.25);
            n.set_mesh_translation(0., 0., 0.25);
            n.set_center_translation(0., 0., 1.25);
        }
        let antenna_joint = model_node(
            b,
            "AntennaJoint",
            "sphere",
            "robot_secondary",
            "robot_secondary_spec",
        )?;
        {
            let mut n = antenna_joint.borrow_mut();
            n.set_mesh_scale(0.1, 0.3, 0.3);
            n.set_center_translation(0., 0.1, 0.);
        }
        let antenna_stand = model_node(
            b,
            "AntennaStand",
            "cube",
            "robot_primary",
            "robot_primary_spec",
        )?;
        {
            let mut n = antenna_stand.borrow_mut();
            n.set_mesh_scale(0.25, 0.5, 0.25);
            n.set_mesh_translation(0., -0.25, 0.);
            n.set_center_translation(0., 0.5, -1.25);
        }
        let antenna = model_node(
            b,
            "Antenna",
            "cube",
            "robot_secondary",
            "robot_secondary_spec",
        )?;
        {
            let mut n = antenna.borrow_mut();
            n.set_mesh_scale(0.1, 2., 0.2);
            n.set_mesh_translation(0., 1.1, 0.);
        }
        let sub_antenna_joint = model_node(
            b,
            "SubAntennaJoint",
            "sphere",
            "robot_secondary",
            "robot_secondary_spec",
        )?;
        {
            let mut n = sub_antenna_joint.borrow_mut();
            n.set_mesh_scale(0.1, 0.2, 0.2);
            n.set_center_translation(0., 0.05, 0.);
        }
        let sub_antenna_stand_rotate =
            Rc::new(RefCell::new(BasicNode::new("SubAntennaStandRotate")));
        sub_antenna_stand_rotate
            .borrow_mut()
            .set_center_rotation(0., -45., 0.);
        let sub_antenna_stand = model_node(
            b,
            "SubAntennaStand",
            "cube",
            "robot_primary",
            "robot_primary_spec",
        )?;
        {
            let mut n = sub_antenna_stand.borrow_mut();
            n.set_mesh_scale(0.2, 0.5, 0.2);
            n.set_mesh_translation(0., -0.25, 0.);
            n.set_center_translation(0., 0.5, -1.25);
        }
        let sub_antenna = model_node(
            b,
            "SubAntenna",
            "cube",
            "robot_secondary",
            "robot_secondary_spec",
        )?;
        {
            let mut n = sub_antenna.borrow_mut();
            n.set_mesh_scale(0.05, 1., 0.1);
            n.set_mesh_translation(0., 0.55, 0.);
        }
        antenna_joint
            .borrow_mut()
            .add_child(Box::new(NodeLink::new(antenna.clone())));
        antenna_stand
            .borrow_mut()
            .add_child(Box::new(NodeLink::new(antenna_joint.clone())));
        sub_antenna_joint
            .borrow_mut()
            .add_child(Box::new(NodeLink::new(sub_antenna.clone())));
        sub_antenna_stand
            .borrow_mut()
            .add_child(Box::new(NodeLink::new(sub_antenna_joint.clone())));
        sub_antenna_stand_rotate
            .borrow_mut()
            .add_child(Box::new(NodeLink::new(sub_antenna_stand.clone())));
        head.borrow_mut()
            .add_child(Box::new(NodeLink::new(eye.clone())));
        head.borrow_mut()
            .add_child(Box::new(NodeLink::new(antenna_stand.clone())));
        head.borrow_mut()
            .add_child(Box::new(NodeLink::new(sub_antenna_stand_rotate.clone())));
        head_joint
            .borrow_mut()
            .add_child(Box::new(NodeLink::new(head.clone())));
        body.borrow_mut()
            .add_child(Box::new(NodeLink::new(head_joint.clone())));
        wheel
            .borrow_mut()
            .add_child(Box::new(NodeLink::new(body.clone())));
        root.add_child(Box::new(NodeLink::new(wheel.clone())));
        Ok(Self {
            root,
            wheel,
            body,
            head_joint,
            head,
            eye,
            antenna_stand,
            antenna_joint,
            antenna,
            sub_antenna_stand_rotate,
            sub_antenna_stand,
            sub_antenna_joint,
            sub_antenna,
        })
    }
    pub fn set_robot_position(&mut self, x: f32, z: f32) {
        self.root.set_center_translation(x, 0., z)
    }
    pub fn set_body_rotation(&mut self, d: f32) {
        self.root.set_center_rotation(0., d, 0.)
    }
    pub fn set_body_pitch(&mut self, d: f32) {
        self.wheel.borrow_mut().set_center_rotation(d, 0., 0.)
    }
    pub fn set_head_pitch(&mut self, d: f32) {
        self.head_joint.borrow_mut().set_center_rotation(d, 0., 0.)
    }
    pub fn set_head_yaw(&mut self, d: f32) {
        self.head.borrow_mut().set_center_rotation(0., d, 0.)
    }
    pub fn set_eye_pitch(&mut self, d: f32) {
        self.eye.borrow_mut().set_center_rotation(d, 0., 0.)
    }
    pub fn set_antenna_pitch(&mut self, d: f32) {
        self.antenna_joint
            .borrow_mut()
            .set_center_rotation(d, 0., 0.)
    }
    pub fn set_sub_antenna_pitch(&mut self, d: f32) {
        self.sub_antenna_joint
            .borrow_mut()
            .set_center_rotation(d, 0., 0.)
    }
}
impl Component for Robot {
    fn node(&self) -> &dyn Node {
        &self.root
    }
    fn node_mut(&mut self) -> &mut dyn Node {
        &mut self.root
    }
}
