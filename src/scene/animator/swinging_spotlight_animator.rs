//! Port of `legacy/scene/animator/SwingingSpotlightAnimator.java`.

use super::interfaces::Animator;
use crate::scene::component::SwingingSpotlight;
use std::{cell::RefCell, rc::Rc};

pub struct SwingingSpotlightAnimator {
    timestamp: f64,
    spotlight: Rc<RefCell<SwingingSpotlight>>,
}
impl SwingingSpotlightAnimator {
    pub fn new(spotlight: Rc<RefCell<SwingingSpotlight>>) -> Self {
        Self {
            timestamp: 0.0,
            spotlight,
        }
    }
}
impl Animator for SwingingSpotlightAnimator {
    fn forward(&mut self, seconds: f64) {
        self.timestamp += seconds;
        self.spotlight
            .borrow_mut()
            // Java converts sin(timestamp) to float before multiplying by 30.
            .set_swinging_angle(self.timestamp.sin() as f32 * 30.0);
        self.spotlight.borrow_mut().sync_light_transform();
    }
}
