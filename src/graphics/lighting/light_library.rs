//! Port of `legacy/graphics/lighting/LightLibrary.java`.

use super::{DirectionalLightNode, PointLightNode, SpotLightNode};
use crate::graphics::{material::Material, model::Model};
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
pub struct LightLibrary {
    directional_lights: Vec<Rc<RefCell<DirectionalLightNode>>>,
    point_lights: Vec<Rc<RefCell<PointLightNode>>>,
    spot_lights: Vec<Rc<RefCell<SpotLightNode>>>,
}

impl LightLibrary {
    pub fn new() -> Self {
        // Direct counterpart of the first line printed by Java's
        // `LightLibrary(GL3, Camera)` constructor.
        println!("Loading lighting resources");
        Self {
            directional_lights: Vec::new(),
            point_lights: Vec::new(),
            spot_lights: Vec::new(),
        }
    }

    pub fn directional_lights(&self) -> &[Rc<RefCell<DirectionalLightNode>>] {
        &self.directional_lights
    }
    pub fn point_lights(&self) -> &[Rc<RefCell<PointLightNode>>] {
        &self.point_lights
    }
    pub fn spot_lights(&self) -> &[Rc<RefCell<SpotLightNode>>] {
        &self.spot_lights
    }

    pub fn create_directional_light(
        &mut self,
        name: impl Into<String>,
        material: Material,
    ) -> Rc<RefCell<DirectionalLightNode>> {
        let light = Rc::new(RefCell::new(DirectionalLightNode::new(name, material)));
        self.directional_lights.push(light.clone());
        light
    }

    pub fn create_point_light(
        &mut self,
        name: impl Into<String>,
        material: Material,
        model: Model,
    ) -> Rc<RefCell<PointLightNode>> {
        let light = Rc::new(RefCell::new(PointLightNode::new(
            name,
            material,
            Some(model),
        )));
        self.point_lights.push(light.clone());
        light
    }

    pub fn create_spot_light(
        &mut self,
        name: impl Into<String>,
        material: Material,
        model: Model,
    ) -> Rc<RefCell<SpotLightNode>> {
        let light = Rc::new(RefCell::new(SpotLightNode::new(name, material, model)));
        self.spot_lights.push(light.clone());
        light
    }

    pub fn remove_all(&mut self) {
        self.directional_lights.clear();
        self.point_lights.clear();
        self.spot_lights.clear();
    }
}
