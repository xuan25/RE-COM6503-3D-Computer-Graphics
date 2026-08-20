//! Port of `legacy/scene/component/Smartphone.java`.

use super::{SceneBuilder, SceneResourceError, interfaces::Component};
use crate::{
    gmaths::Vec3,
    graphics::{
        material::Material,
        node::{BasicNode, CenterTransformable, MeshTransformable, ModelNode, Node, NodeLink},
    },
};
use std::{cell::RefCell, rc::Rc};

pub struct Smartphone {
    root_node: BasicNode,
    stand_node: Rc<RefCell<ModelNode>>,
    smartphone_node: Rc<RefCell<ModelNode>>,
}

impl Smartphone {
    pub fn new(builder: &SceneBuilder) -> Result<Self, SceneResourceError> {
        let mut root_node = BasicNode::new("SmartphoneWithStand");
        let stand_node = Rc::new(RefCell::new(ModelNode::new(
            "Stand",
            Some(builder.model(
                builder.default_material(),
                "cube",
                &["phone_stand", "phone_stand_spec"],
            )?),
        )));
        {
            let mut node = stand_node.borrow_mut();
            node.set_mesh_scale(2.0, 1.0, 1.0);
            node.set_center_translation(0.0, 0.5, 0.0);
        }
        let material = Material::new(
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(0.2, 0.2, 0.2),
            16.0,
        );
        let smartphone_node = Rc::new(RefCell::new(ModelNode::new(
            "Smartphone",
            Some(builder.model(material, "smartphone", &["phone", "phone_spec"])?),
        )));
        {
            let mut node = smartphone_node.borrow_mut();
            node.set_mesh_scale(3.5, 6.0, 0.4);
            node.set_mesh_translation(0.0, 3.0, 0.0);
        }
        stand_node
            .borrow_mut()
            .add_child(Box::new(NodeLink::new(smartphone_node.clone())));
        root_node.add_child(Box::new(NodeLink::new(stand_node.clone())));
        Ok(Self {
            root_node,
            stand_node,
            smartphone_node,
        })
    }
    pub fn root_node(&self) -> &BasicNode {
        &self.root_node
    }
    pub fn stand_node(&self) -> Rc<RefCell<ModelNode>> {
        self.stand_node.clone()
    }
    pub fn smartphone_node(&self) -> Rc<RefCell<ModelNode>> {
        self.smartphone_node.clone()
    }
}

impl Component for Smartphone {
    fn node(&self) -> &dyn Node {
        &self.root_node
    }
    fn node_mut(&mut self) -> &mut dyn Node {
        &mut self.root_node
    }
}
