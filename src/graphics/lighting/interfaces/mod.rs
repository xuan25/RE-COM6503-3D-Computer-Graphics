//! Direct translations of `legacy/graphics/lighting/interfaces`.

pub mod attenuated;
pub mod directional;
pub mod lighting;
pub mod positional;
pub mod ranged;

pub use attenuated::Attenuated;
pub use directional::Directional;
pub use lighting::Lighting;
pub use positional::Positional;
pub use ranged::Ranged;
