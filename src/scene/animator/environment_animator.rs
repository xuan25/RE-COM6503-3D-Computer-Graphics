//! Port of `legacy/scene/animator/EnvironmentAnimator.java`.

use super::interfaces::Animator;
use crate::{
    gmaths::Vec3,
    graphics::{
        lighting::DirectionalLightNode,
        model::{Skybox, Skysphere},
        node::ModelNode,
    },
};
use std::{cell::RefCell, rc::Rc};
pub struct EnvironmentAnimator {
    timestamp: f64,
    /// Java's `windowViewNode` constructor argument.
    window_view_node: Rc<RefCell<ModelNode>>,
    skybox: Rc<RefCell<Skybox>>,
    skysphere: Rc<RefCell<Skysphere>>,
    /// Java's `daylightNode` constructor argument.
    daylight_node: Rc<RefCell<DirectionalLightNode>>,
    custom_lighting_intensity: f32,
}
impl EnvironmentAnimator {
    pub const DAY_NIGHT_CYCLE_SPEED: f32 = 0.4;
    pub const SNOW_SPEED_X: f32 = 0.5;
    pub const SNOW_SPEED_Y: f32 = 0.5;
    pub const TURBULENCE_SPEED: f32 = 1.0;
    pub const TURBULENCE_STRENGTH: f32 = 0.6;
    pub const DAYLIGHT_DELAY_R: f32 = 0.4;
    pub const DAYLIGHT_DELAY_G: f32 = 0.3;
    pub const DAYLIGHT_DELAY_B: f32 = 0.0;
    pub const DAYLIGHT_STRENGTH_OFFSET: f32 = 1.2;
    pub const DAYLIGHT_AMBIENT: f32 = 0.4;
    pub const DAYLIGHT_DIFFUSE: f32 = 0.2;
    pub const DAYLIGHT_SPECULAR: f32 = 0.2;
    pub const SKY_ROTATE_SPEED: f32 = 0.001;

    pub fn new(
        window_view_node: Rc<RefCell<ModelNode>>,
        skybox: Rc<RefCell<Skybox>>,
        skysphere: Rc<RefCell<Skysphere>>,
        daylight_node: Rc<RefCell<DirectionalLightNode>>,
    ) -> Self {
        Self {
            timestamp: 0.,
            window_view_node,
            skybox,
            skysphere,
            daylight_node,
            custom_lighting_intensity: 1.,
        }
    }
    pub fn set_custom_lighting_intensity(&mut self, intensity: f32) {
        self.custom_lighting_intensity = intensity;
    }
}
impl Animator for EnvironmentAnimator {
    fn forward(&mut self, seconds: f64) {
        self.timestamp += seconds;
        // Java keeps `timestamp` as a double for every calculation and only
        // converts each final result to float.  Do not truncate the timer
        // first: that changes the day/night and UV animation over time.
        let t = self.timestamp;
        let x = (t * Self::SNOW_SPEED_X as f64) as f32;
        let y = (t * Self::SNOW_SPEED_Y as f64) as f32;
        let z = ((t * Self::TURBULENCE_SPEED as f64).sin() as f32) * Self::TURBULENCE_STRENGTH;
        let rgb = Vec3::new(
            (((t * Self::DAY_NIGHT_CYCLE_SPEED as f64 - Self::DAYLIGHT_DELAY_R as f64).sin()
                as f32
                + Self::DAYLIGHT_STRENGTH_OFFSET)
                * 0.5
                * self.custom_lighting_intensity),
            (((t * Self::DAY_NIGHT_CYCLE_SPEED as f64 - Self::DAYLIGHT_DELAY_G as f64).sin()
                as f32
                + Self::DAYLIGHT_STRENGTH_OFFSET)
                * 0.5
                * self.custom_lighting_intensity),
            (((t * Self::DAY_NIGHT_CYCLE_SPEED as f64 - Self::DAYLIGHT_DELAY_B as f64).sin()
                as f32
                + Self::DAYLIGHT_STRENGTH_OFFSET)
                * 0.5
                * self.custom_lighting_intensity),
        );
        if let Some(model) = self.window_view_node.borrow_mut().model_mut() {
            model.set_uv_offset(x + z, y);
            model.material_mut().set_diffuse_vec3(rgb);
        }
        self.skybox.borrow_mut().set_diffuse(rgb);
        self.skysphere.borrow_mut().set_diffuse(rgb);
        self.skysphere
            .borrow_mut()
            .set_uv_offset((t * Self::SKY_ROTATE_SPEED as f64) as f32, 0.);
        self.daylight_node
            .borrow_mut()
            .set_material(crate::graphics::material::Material::new(
                rgb * Self::DAYLIGHT_AMBIENT * self.custom_lighting_intensity,
                rgb * Self::DAYLIGHT_DIFFUSE * self.custom_lighting_intensity,
                rgb * Self::DAYLIGHT_SPECULAR * self.custom_lighting_intensity,
                16.,
            ));
    }
}
