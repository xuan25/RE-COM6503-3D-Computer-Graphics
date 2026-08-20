//! Port of `legacy/graphics/model/Skybox.java`.

#![allow(unsafe_op_in_unsafe_fn)]

use super::{MeshLibrary, Model};
use crate::{
    gmaths::Mat4,
    graphics::{
        basic::Cube,
        camera::Camera,
        interfaces::{Renderable, TransformRenderable},
        material::{Material, Texture},
        shader::ShaderLibrary,
    },
};
use std::{cell::RefCell, rc::Rc};

pub struct Skybox {
    model: Model,
}

impl Skybox {
    pub unsafe fn new(
        camera: Rc<RefCell<Camera>>,
        shader_library: &mut ShaderLibrary,
        material: Material,
        mesh_library: &mut MeshLibrary,
        texture: Rc<Texture>,
    ) -> Result<Self, std::io::Error> {
        let shader = shader_library.load_shader("shaders/skybox.vert", "shaders/skybox.frag")?;
        let mut indices = Cube::indices();
        for triangle in indices.chunks_exact_mut(3) {
            triangle.swap(1, 2);
        }
        let mesh = mesh_library.create_mesh(&Cube::vertices(), &indices);
        Ok(Self {
            model: Model::new(camera, None, shader, material, mesh, vec![texture]),
        })
    }
    pub fn set_diffuse(&mut self, color: crate::gmaths::Vec3) {
        self.model.material_mut().set_diffuse_vec3(color);
    }
}

impl Renderable for Skybox {
    unsafe fn render(&self) {
        self.model.render_internal(Mat4::identity(), true, false);
    }
}
impl TransformRenderable for Skybox {
    unsafe fn render_with_transform(&self, transform: Mat4) {
        self.model.render_internal(transform, true, false);
    }
}
