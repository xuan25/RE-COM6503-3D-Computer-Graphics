//! Port of `legacy/graphics/offscreen/interfaces/OffscreenBuffer.java`.

pub trait OffscreenBuffer {
    unsafe fn reshape(&mut self, width: i32, height: i32) -> Result<(), String>;
    unsafe fn use_buffer(&self);
    unsafe fn render(&self, destination_framebuffer: u32);
    fn framebuffer_id(&self) -> u32;
}
