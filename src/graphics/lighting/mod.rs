//! Rust counterparts of `legacy/graphics/lighting`.

pub mod directional_light_node;
pub mod interfaces;
pub mod light_library;
pub mod point_light_node;
pub mod spot_light_node;
#[cfg(test)]
mod tests;

pub use directional_light_node::DirectionalLightNode;
pub use interfaces::{Attenuated, Directional, Lighting, Positional, Ranged};
pub use light_library::LightLibrary;
pub use point_light_node::PointLightNode;
pub use spot_light_node::SpotLightNode;
