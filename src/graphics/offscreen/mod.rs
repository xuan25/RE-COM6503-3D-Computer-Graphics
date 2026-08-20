//! Rust counterparts of `legacy/graphics/offscreen`.

pub mod hdr_offscreen_buffer;
pub mod interfaces;
pub mod msaa_offscreen_buffer;
pub mod shader_offscreen_buffer;

pub use hdr_offscreen_buffer::HdrOffscreenBuffer;
pub use interfaces::OffscreenBuffer;
pub use msaa_offscreen_buffer::MsaaOffscreenBuffer;
pub use shader_offscreen_buffer::ShaderOffscreenBuffer;
