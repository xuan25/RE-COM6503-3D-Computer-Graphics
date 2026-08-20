//! Port of `legacy/graphics/offscreen/HDROffscreenBuffer.java`.

#![allow(unsafe_op_in_unsafe_fn)]

use super::{ShaderOffscreenBuffer, interfaces::OffscreenBuffer};

pub struct HdrOffscreenBuffer {
    buffer: ShaderOffscreenBuffer,
    exposure: f32,
}

impl HdrOffscreenBuffer {
    pub unsafe fn new() -> Result<Self, std::io::Error> {
        Ok(Self {
            buffer: ShaderOffscreenBuffer::new(
                "shaders/offscreen_default.vert",
                "shaders/offscreen_gamma_correction_hdr.frag",
            )?,
            exposure: 2.5,
        })
    }
    pub fn set_exposure(&mut self, exposure: f32) {
        self.exposure = exposure;
    }
}

impl OffscreenBuffer for HdrOffscreenBuffer {
    unsafe fn reshape(&mut self, width: i32, height: i32) -> Result<(), String> {
        self.buffer.reshape(width, height)
    }
    unsafe fn use_buffer(&self) {
        self.buffer.use_buffer();
    }
    unsafe fn render(&self, destination_framebuffer: u32) {
        let depth_test = gl::IsEnabled(gl::DEPTH_TEST) == gl::TRUE;
        gl::BindFramebuffer(gl::FRAMEBUFFER, destination_framebuffer);
        gl::Disable(gl::DEPTH_TEST);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        self.buffer.shader.use_program();
        self.buffer.shader.set_int("screenTexture", 0);
        self.buffer.shader.set_float("exposure", self.exposure);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, self.buffer.color_buffer_id);
        gl::BindVertexArray(self.buffer.quad_vertex_array_id);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        gl::BindVertexArray(0);
        if depth_test {
            gl::Enable(gl::DEPTH_TEST);
        }
    }
    fn framebuffer_id(&self) -> u32 {
        self.buffer.framebuffer_id()
    }
}
