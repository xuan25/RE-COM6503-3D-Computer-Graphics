//! Rust counterparts of `legacy/scene/component`.

pub mod egg;
pub mod interfaces;
pub mod robot;
pub mod room;
pub mod scene_builder;
pub mod smartphone;
pub mod swinging_spotlight;

pub use egg::Egg;
pub use robot::Robot;
pub use room::Room;
pub use scene_builder::{SceneBuilder, SceneResourceError};
pub use smartphone::Smartphone;
pub use swinging_spotlight::SwingingSpotlight;
