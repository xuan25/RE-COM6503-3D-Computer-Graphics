//! Rust counterparts of `legacy/scene/animator`.

pub mod environment_animator;
pub mod interfaces;
pub mod robot_animator;
pub mod swinging_spotlight_animator;
pub mod utils;

pub use environment_animator::EnvironmentAnimator;
pub use robot_animator::RobotAnimator;
pub use swinging_spotlight_animator::SwingingSpotlightAnimator;
