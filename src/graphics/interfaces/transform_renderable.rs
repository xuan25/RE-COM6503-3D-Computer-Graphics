use super::Renderable;
use crate::gmaths::Mat4;

pub trait TransformRenderable: Renderable {
    unsafe fn render_with_transform(&self, transform: Mat4);
}
