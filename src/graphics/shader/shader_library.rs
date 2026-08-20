//! Port of `legacy/graphics/shader/ShaderLibrary.java`.

#![allow(unsafe_op_in_unsafe_fn)]

use super::Shader;
use std::{path::Path, rc::Rc};

#[derive(Default)]
pub struct ShaderLibrary {
    loaded_shaders: Vec<Rc<Shader>>,
}

impl ShaderLibrary {
    pub const fn new() -> Self {
        Self {
            loaded_shaders: Vec::new(),
        }
    }

    pub unsafe fn load_shader(
        &mut self,
        vertex_path: impl AsRef<Path>,
        fragment_path: impl AsRef<Path>,
    ) -> Result<Rc<Shader>, std::io::Error> {
        println!(
            "Loading shader - {} | {}",
            vertex_path.as_ref().display(),
            fragment_path.as_ref().display()
        );
        self.load_shader_impl(vertex_path, fragment_path)
    }

    /// Load a shader that was constructed directly by an owning component,
    /// rather than through Java's `ShaderLibrary.loadShader` API.
    pub unsafe fn load_shader_silent(
        &mut self,
        vertex_path: impl AsRef<Path>,
        fragment_path: impl AsRef<Path>,
    ) -> Result<Rc<Shader>, std::io::Error> {
        self.load_shader_impl(vertex_path, fragment_path)
    }

    unsafe fn load_shader_impl(
        &mut self,
        vertex_path: impl AsRef<Path>,
        fragment_path: impl AsRef<Path>,
    ) -> Result<Rc<Shader>, std::io::Error> {
        let shader = Rc::new(Shader::from_files(vertex_path, fragment_path)?);
        self.loaded_shaders.push(Rc::clone(&shader));
        Ok(shader)
    }

    pub fn unload(&mut self, shader: &Rc<Shader>) -> bool {
        if let Some(index) = self
            .loaded_shaders
            .iter()
            .position(|item| Rc::ptr_eq(item, shader))
        {
            self.loaded_shaders.remove(index);
            true
        } else {
            false
        }
    }

    pub fn unload_all(&mut self) {
        self.loaded_shaders.clear();
    }
}
