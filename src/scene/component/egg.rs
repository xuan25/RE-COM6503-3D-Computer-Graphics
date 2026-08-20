//! Port of `legacy/scene/component/Egg.java`.

use super::{SceneBuilder, SceneResourceError, interfaces::Component};
use crate::{
    gmaths::Vec3,
    graphics::{
        material::Material,
        node::{BasicNode, CenterTransformable, MeshTransformable, ModelNode, Node, NodeLink},
    },
};
use std::{cell::RefCell, rc::Rc};

pub struct Egg {
    root_node: BasicNode,
    stand_node: Rc<RefCell<ModelNode>>,
    egg_node: Rc<RefCell<ModelNode>>,
}

impl Egg {
    pub fn new(builder: &SceneBuilder) -> Result<Self, SceneResourceError> {
        let mut root_node = BasicNode::new("EggWithStand");
        let stand_node = Rc::new(RefCell::new(ModelNode::new(
            "Stand",
            Some(builder.model(
                builder.default_material(),
                "cube",
                &["egg_stand", "egg_stand_spec"],
            )?),
        )));
        {
            let mut node = stand_node.borrow_mut();
            node.model_mut()
                .expect("stand model")
                .set_uv_scale(0.5, 0.5);
            node.set_mesh_scale(5.0, 0.2, 5.0);
            node.set_mesh_translation(0.0, 0.1, 0.0);
        }
        let egg_material = Material::new(
            Vec3::new(0.5, 0.5, 0.5),
            Vec3::new(0.5, 0.5, 0.5),
            Vec3::new(1.0, 1.0, 1.0),
            16.0,
        );
        let egg_node = Rc::new(RefCell::new(ModelNode::new(
            "Egg",
            Some(builder.model(egg_material, "sphere", &["egg", "egg_spec"])?),
        )));
        {
            let mut node = egg_node.borrow_mut();
            node.model_mut().expect("egg model").set_uv_scale(2.0, 2.0);
            node.set_mesh_scale(5.0, 8.0, 5.0);
            node.set_mesh_translation(0.0, 4.0, 0.0);
            node.set_center_translation(0.0, 0.2, 0.0);
        }
        stand_node
            .borrow_mut()
            .add_child(Box::new(NodeLink::new(egg_node.clone())));
        root_node.add_child(Box::new(NodeLink::new(stand_node.clone())));
        Ok(Self {
            root_node,
            stand_node,
            egg_node,
        })
    }
    pub fn root_node(&self) -> &BasicNode {
        &self.root_node
    }
    pub fn stand_node(&self) -> Rc<RefCell<ModelNode>> {
        self.stand_node.clone()
    }
    pub fn egg_node(&self) -> Rc<RefCell<ModelNode>> {
        self.egg_node.clone()
    }
}

impl Component for Egg {
    fn node(&self) -> &dyn Node {
        &self.root_node
    }
    fn node_mut(&mut self) -> &mut dyn Node {
        &mut self.root_node
    }
}
