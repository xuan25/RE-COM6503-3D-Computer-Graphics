//! Port of `legacy/scene/component/SwingingSpotlight.java`.

use super::{SceneBuilder, SceneResourceError, interfaces::Component};
use crate::{
    gmaths::Vec3,
    graphics::{
        lighting::SpotLightNode,
        material::Material,
        node::{BasicNode, CenterTransformable, MeshTransformable, ModelNode, Node, NodeLink},
    },
};
use std::{cell::RefCell, rc::Rc};

pub struct SwingingSpotlight {
    root_node: BasicNode,
    stand_node: Rc<RefCell<ModelNode>>,
    pole_node: Rc<RefCell<ModelNode>>,
    arm_node: Rc<RefCell<ModelNode>>,
    lamp_joint_node: Rc<RefCell<ModelNode>>,
    lampshade_node: Rc<RefCell<ModelNode>>,
    light_node: Rc<RefCell<SpotLightNode>>,
    custom_lighting_intensity: f32,
    light_material: Material,
    pole_material: Material,
}

impl SwingingSpotlight {
    pub fn new(builder: &SceneBuilder) -> Result<Self, SceneResourceError> {
        let mut root_node = BasicNode::new("SwingingSpotlight");
        let pole_material = Material::new(
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(0.2, 0.2, 0.2),
            16.0,
        );
        let stand = Rc::new(RefCell::new(ModelNode::new(
            "Stand",
            Some(builder.model(pole_material, "cube", &["light_pole", "light_pole_spec"])?),
        )));
        stand.borrow_mut().set_mesh_scale(1.0, 0.2, 1.0);
        stand.borrow_mut().set_mesh_translation(0.0, 0.1, 0.0);
        let pole = Rc::new(RefCell::new(ModelNode::new(
            "Pole",
            Some(builder.model(pole_material, "cube", &["light_pole", "light_pole_spec"])?),
        )));
        {
            let mut node = pole.borrow_mut();
            node.model_mut()
                .expect("pole model")
                .set_uv_scale(1.0, 10.0);
            node.set_mesh_scale(0.2, 10.0, 0.2);
            node.set_mesh_translation(0.0, 5.0, 0.0);
            node.set_center_translation(0.0, 0.2, 0.0);
        }
        let arm = Rc::new(RefCell::new(ModelNode::new(
            "Arm",
            Some(builder.model(pole_material, "cube", &["light_pole", "light_pole_spec"])?),
        )));
        {
            let mut node = arm.borrow_mut();
            node.model_mut().expect("arm model").set_uv_scale(1.0, 4.0);
            node.set_mesh_scale(0.2, 4.0, 0.2);
            node.set_mesh_translation(-1.9, 0.1, 0.0);
            node.set_mesh_rotation(0.0, 0.0, 90.0);
            node.set_center_translation(0.0, 10.0, 0.0);
        }
        let lamp_joint_node = Rc::new(RefCell::new(ModelNode::new(
            "LampJoint",
            Some(builder.model(
                pole_material,
                "sphere",
                &["robot_secondary", "robot_secondary_spec"],
            )?),
        )));
        {
            let mut node = lamp_joint_node.borrow_mut();
            node.model_mut()
                .expect("lamp model")
                .set_uv_offset(0.0, 0.2);
            node.set_mesh_scale(0.15, 0.15, 0.15);
            node.set_center_translation(-3.8, 0.0, 0.0);
        }
        let lampshade = Rc::new(RefCell::new(ModelNode::new(
            "Lampshade",
            Some(builder.model(pole_material, "cube", &["light_pole", "light_pole_spec"])?),
        )));
        {
            let mut node = lampshade.borrow_mut();
            node.set_mesh_scale(0.8, 1.0, 0.8);
            node.set_mesh_translation(0.0, -0.5, 0.0);
            node.set_center_translation(0.0, -0.05, 0.0);
        }
        lamp_joint_node
            .borrow_mut()
            .add_child(Box::new(NodeLink::new(lampshade.clone())));
        arm.borrow_mut()
            .add_child(Box::new(NodeLink::new(lamp_joint_node.clone())));
        pole.borrow_mut()
            .add_child(Box::new(NodeLink::new(arm.clone())));
        stand
            .borrow_mut()
            .add_child(Box::new(NodeLink::new(pole.clone())));
        root_node.add_child(Box::new(NodeLink::new(stand.clone())));
        let light_material = Material::default();
        let light_model = builder.light_source_model(light_material)?;
        let light_node = builder.light_library.borrow_mut().create_spot_light(
            "Spotlight",
            light_material,
            light_model,
        );
        light_node.borrow_mut().set_center_translation(0., -1., 0.);
        lampshade
            .borrow_mut()
            .add_child(Box::new(NodeLink::new(light_node.clone())));
        let mut result = Self {
            root_node,
            stand_node: stand,
            pole_node: pole,
            arm_node: arm,
            lamp_joint_node,
            lampshade_node: lampshade,
            light_node,
            custom_lighting_intensity: 1.0,
            light_material,
            pole_material,
        };
        result.set_custom_lighting_intensity(1.0);
        Ok(result)
    }
    pub fn set_custom_lighting_intensity(&mut self, intensity: f32) {
        self.custom_lighting_intensity = intensity;
        self.light_material = Material::new(
            Vec3::new(0.2 * intensity, 0.15 * intensity, 0.1 * intensity),
            Vec3::new(intensity, 0.9 * intensity, 0.7 * intensity),
            Vec3::new(intensity, 0.9 * intensity, 0.7 * intensity),
            32.0,
        );
        self.light_node
            .borrow_mut()
            .set_material(self.light_material);
    }
    pub fn set_swinging_angle(&mut self, degrees: f32) {
        self.lamp_joint_node
            .borrow_mut()
            .set_center_rotation(degrees, 0.0, 0.0);
    }
    pub fn sync_light_transform(&mut self) {
        self.root_node.update();
    }
    pub fn root_node(&self) -> &BasicNode {
        &self.root_node
    }
}
impl Component for SwingingSpotlight {
    fn node(&self) -> &dyn Node {
        &self.root_node
    }
    fn node_mut(&mut self) -> &mut dyn Node {
        &mut self.root_node
    }
}
