//! Port of `legacy/scene/animator/interfaces/Animator.java`.

pub trait Animator {
    fn forward(&mut self, seconds: f64);
}
