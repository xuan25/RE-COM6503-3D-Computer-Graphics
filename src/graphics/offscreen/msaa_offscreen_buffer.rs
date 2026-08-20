//! Port of `legacy/graphics/offscreen/MSAAOffScreenBuffer.java`.

#![allow(unsafe_op_in_unsafe_fn)]

use super::interfaces::OffscreenBuffer;

pub struct MsaaOffscreenBuffer {
    samples: i32,
    width: i32,
    height: i32,
    color_buffer_id: u32,
    depth_stencil_buffer_id: u32,
    framebuffer_id: u32,
}

impl MsaaOffscreenBuffer {
    pub const fn new(samples: i32) -> Self {
        Self {
            samples,
            width: 0,
            height: 0,
            color_buffer_id: 0,
            depth_stencil_buffer_id: 0,
            framebuffer_id: 0,
        }
    }
    unsafe fn clear_buffers(&mut self) {
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

impl OffscreenBuffer for MsaaOffscreenBuffer {
    unsafe fn reshape(&mut self, width: i32, height: i32) -> Result<(), String> {
        self.clear_buffers();
        self.width = width;
        self.height = height;
        gl::GenFramebuffers(1, &mut self.framebuffer_id);
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id);
        gl::GenTextures(1, &mut self.color_buffer_id);
        gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, self.color_buffer_id);
        gl::TexImage2DMultisample(
            gl::TEXTURE_2D_MULTISAMPLE,
            self.samples,
            gl::RGBA16F,
            width,
            height,
            gl::TRUE,
        );
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D_MULTISAMPLE,
            self.color_buffer_id,
            0,
        );
        gl::GenRenderbuffers(1, &mut self.depth_stencil_buffer_id);
        gl::BindRenderbuffer(gl::RENDERBUFFER, self.depth_stencil_buffer_id);
        gl::RenderbufferStorageMultisample(
            gl::RENDERBUFFER,
            self.samples,
            gl::DEPTH24_STENCIL8,
            width,
            height,
        );
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
    }
    unsafe fn render(&self, destination_framebuffer: u32) {
        gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.framebuffer_id);
        gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, destination_framebuffer);
        gl::BlitFramebuffer(
            0,
            0,
            self.width,
            self.height,
            0,
            0,
            self.width,
            self.height,
            gl::COLOR_BUFFER_BIT,
            gl::NEAREST,
        );
    }
    fn framebuffer_id(&self) -> u32 {
        self.framebuffer_id
    }
}

impl Drop for MsaaOffscreenBuffer {
    fn drop(&mut self) {
        unsafe {
            self.clear_buffers();
        }
    }
}
