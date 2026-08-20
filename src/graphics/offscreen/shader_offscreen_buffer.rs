//! Port of `legacy/graphics/offscreen/ShaderOffscreenBuffer.java`.

#![allow(unsafe_op_in_unsafe_fn)]

use super::interfaces::OffscreenBuffer;
use crate::graphics::shader::Shader;
use std::path::Path;

pub struct ShaderOffscreenBuffer {
    pub(crate) color_buffer_id: u32,
    depth_stencil_buffer_id: u32,
    framebuffer_id: u32,
    pub(crate) shader: Shader,
    pub(crate) quad_vertex_array_id: u32,
    quad_vertex_buffer_id: u32,
}

impl ShaderOffscreenBuffer {
    const QUAD_VERTICES: [f32; 24] = [
        -1.0, 1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0,
        -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
    ];

    pub unsafe fn new(
        vertex_path: impl AsRef<Path>,
        fragment_path: impl AsRef<Path>,
    ) -> Result<Self, std::io::Error> {
        let shader = Shader::from_files(vertex_path, fragment_path)?;
        let mut result = Self {
            color_buffer_id: 0,
            depth_stencil_buffer_id: 0,
            framebuffer_id: 0,
            shader,
            quad_vertex_array_id: 0,
            quad_vertex_buffer_id: 0,
        };
        result.initialize_quad();
        Ok(result)
    }

    unsafe fn initialize_quad(&mut self) {
        gl::GenVertexArrays(1, &mut self.quad_vertex_array_id);
        gl::GenBuffers(1, &mut self.quad_vertex_buffer_id);
        gl::BindVertexArray(self.quad_vertex_array_id);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_vertex_buffer_id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&Self::QUAD_VERTICES) as isize,
            Self::QUAD_VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            4 * std::mem::size_of::<f32>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            4 * std::mem::size_of::<f32>() as i32,
            (2 * std::mem::size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);
        gl::BindVertexArray(0);
    }

    unsafe fn clear_image_buffers(&mut self) {
        if self.framebuffer_id != 0 {
            gl::DeleteFramebuffers(1, &self.framebuffer_id);
        }
        if self.color_buffer_id != 0 {
            gl::DeleteTextures(1, &self.color_buffer_id);
        }
        if self.depth_stencil_buffer_id != 0 {
            gl::DeleteRenderbuffers(1, &self.depth_stencil_buffer_id);
        }
        self.framebuffer_id = 0;
        self.color_buffer_id = 0;
        self.depth_stencil_buffer_id = 0;
    }
}

impl OffscreenBuffer for ShaderOffscreenBuffer {
    unsafe fn reshape(&mut self, width: i32, height: i32) -> Result<(), String> {
        self.clear_image_buffers();
        gl::GenFramebuffers(1, &mut self.framebuffer_id);
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id);
        gl::GenTextures(1, &mut self.color_buffer_id);
        gl::BindTexture(gl::TEXTURE_2D, self.color_buffer_id);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA16F as i32,
            width,
            height,
            0,
            gl::RGBA,
            gl::FLOAT,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            self.color_buffer_id,
            0,
        );
        gl::GenRenderbuffers(1, &mut self.depth_stencil_buffer_id);
        gl::BindRenderbuffer(gl::RENDERBUFFER, self.depth_stencil_buffer_id);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, width, height);
        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            self.depth_stencil_buffer_id,
        );
        let complete = gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE;
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        complete
            .then_some(())
            .ok_or_else(|| "FRAMEBUFFER is not complete".to_owned())
    }

    unsafe fn use_buffer(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id);
        gl::Enable(gl::DEPTH_TEST);
    }
    unsafe fn render(&self, destination_framebuffer: u32) {
        let depth_test = gl::IsEnabled(gl::DEPTH_TEST) == gl::TRUE;
        gl::BindFramebuffer(gl::FRAMEBUFFER, destination_framebuffer);
        gl::Disable(gl::DEPTH_TEST);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        self.shader.use_program();
        self.shader.set_int("screenTexture", 0);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, self.color_buffer_id);
        gl::BindVertexArray(self.quad_vertex_array_id);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        gl::BindVertexArray(0);
        if depth_test {
            gl::Enable(gl::DEPTH_TEST);
        }
    }
    fn framebuffer_id(&self) -> u32 {
        self.framebuffer_id
    }
}

impl Drop for ShaderOffscreenBuffer {
    fn drop(&mut self) {
        unsafe {
            self.clear_image_buffers();
            gl::DeleteBuffers(1, &self.quad_vertex_buffer_id);
            gl::DeleteVertexArrays(1, &self.quad_vertex_array_id);
        }
    }
}
