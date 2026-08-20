//! Port of `legacy/graphics/shader/Shader.java`.

#![allow(unsafe_op_in_unsafe_fn)]

use crate::gmaths::Vec3;
use std::{ffi::CString, fs, path::Path};

pub struct Shader {
    id: u32,
    vertex_shader_source: String,
    fragment_shader_source: String,
}

impl Shader {
    pub unsafe fn from_files(
        vertex_path: impl AsRef<Path>,
        fragment_path: impl AsRef<Path>,
    ) -> Result<Self, std::io::Error> {
        let vertex_shader_source = fs::read_to_string(vertex_path)?;
        let fragment_shader_source = fs::read_to_string(fragment_path)?;
        let vertex = Self::compile(gl::VERTEX_SHADER, &vertex_shader_source)?;
        let fragment = Self::compile(gl::FRAGMENT_SHADER, &fragment_shader_source)?;
        let id = gl::CreateProgram();
        gl::AttachShader(id, vertex);
        gl::AttachShader(id, fragment);
        gl::LinkProgram(id);
        gl::DeleteShader(vertex);
        gl::DeleteShader(fragment);
        if let Err(log) = Self::program_log(id) {
            gl::DeleteProgram(id);
            return Err(std::io::Error::other(log));
        }
        Ok(Self {
            id,
            vertex_shader_source,
            fragment_shader_source,
        })
    }

    unsafe fn compile(kind: u32, source: &str) -> Result<u32, std::io::Error> {
        let id = gl::CreateShader(kind);
        let source = CString::new(source).map_err(|error| std::io::Error::other(error))?;
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
        let mut status = 0;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);
        if status == gl::TRUE as i32 {
            Ok(id)
        } else {
            let log = Self::shader_log(id);
            gl::DeleteShader(id);
            Err(std::io::Error::other(log))
        }
    }

    unsafe fn shader_log(id: u32) -> String {
        let mut len = 0;
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        let mut buffer = vec![0_u8; len.max(1) as usize];
        gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), buffer.as_mut_ptr().cast());
        String::from_utf8_lossy(&buffer)
            .trim_end_matches('\0')
            .into()
    }

    unsafe fn program_log(id: u32) -> Result<(), String> {
        let mut status = 0;
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);
        if status == gl::TRUE as i32 {
            return Ok(());
        }
        let mut len = 0;
        gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
        let mut buffer = vec![0_u8; len.max(1) as usize];
        gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), buffer.as_mut_ptr().cast());
        Err(String::from_utf8_lossy(&buffer)
            .trim_end_matches('\0')
            .into())
    }

    pub const fn id(&self) -> u32 {
        self.id
    }

    pub fn vertex_shader_source(&self) -> &str {
        &self.vertex_shader_source
    }

    pub fn fragment_shader_source(&self) -> &str {
        &self.fragment_shader_source
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    /// Equivalent to JOGL's `Shader.validate()` used by `Model.debugShader`.
    pub unsafe fn validate(&self) -> Result<(), String> {
        gl::ValidateProgram(self.id);
        let mut status = 0;
        gl::GetProgramiv(self.id, gl::VALIDATE_STATUS, &mut status);
        if status == gl::TRUE as i32 {
            return Ok(());
        }
        let mut len = 0;
        gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
        let mut buffer = vec![0_u8; len.max(1) as usize];
        gl::GetProgramInfoLog(
            self.id,
            len,
            std::ptr::null_mut(),
            buffer.as_mut_ptr().cast(),
        );
        Err(String::from_utf8_lossy(&buffer)
            .trim_end_matches('\0')
            .into())
    }

    unsafe fn location(&self, name: &str) -> i32 {
        let name = CString::new(name).expect("uniform names cannot contain NUL");
        gl::GetUniformLocation(self.id, name.as_ptr())
    }

    pub unsafe fn set_int(&self, name: &str, value: i32) {
        gl::Uniform1i(self.location(name), value);
    }

    pub unsafe fn set_float(&self, name: &str, value: f32) {
        gl::Uniform1f(self.location(name), value);
    }

    pub unsafe fn set_vec2(&self, name: &str, x: f32, y: f32) {
        gl::Uniform2f(self.location(name), x, y);
    }

    pub unsafe fn set_vec3_components(&self, name: &str, x: f32, y: f32, z: f32) {
        gl::Uniform3f(self.location(name), x, y, z);
    }

    pub unsafe fn set_vec4(&self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        gl::Uniform4f(self.location(name), x, y, z, w);
    }

    pub unsafe fn set_float_array(&self, name: &str, values: &[f32; 16]) {
        gl::UniformMatrix4fv(self.location(name), 1, gl::FALSE, values.as_ptr());
    }

    pub unsafe fn set_vec3(&self, name: &str, value: Vec3) {
        self.set_vec3_components(name, value.x, value.y, value.z);
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}
