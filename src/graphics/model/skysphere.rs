//! Port of `legacy/graphics/model/Skysphere.java`.

#![allow(unsafe_op_in_unsafe_fn)]

use super::{MeshLibrary, Model};
use crate::{
    gmaths::Mat4,
    graphics::{
        basic::Sphere,
        camera::Camera,
        interfaces::{Renderable, TransformRenderable},
        material::{Material, Texture},
        shader::ShaderLibrary,
    },
};
use std::{cell::RefCell, rc::Rc};

pub struct Skysphere {
    model: Model,
}

impl Skysphere {
    pub unsafe fn new(
        camera: Rc<RefCell<Camera>>,
        shader_library: &mut ShaderLibrary,
        material: Material,
        mesh_library: &mut MeshLibrary,
        texture: Rc<Texture>,
    ) -> Result<Self, std::io::Error> {
        let shader =
            shader_library.load_shader("shaders/skysphere.vert", "shaders/skysphere.frag")?;
        let mut indices = Sphere::indices();
        for triangle in indices.chunks_exact_mut(3) {
            triangle.swap(1, 2);
        }
        let mesh = mesh_library.create_mesh(&Sphere::vertices(), &indices);
        Ok(Self {
            model: Model::new(camera, None, shader, material, mesh, vec![texture]),
        })
    }
    pub fn set_diffuse(&mut self, color: crate::gmaths::Vec3) {
        self.model.material_mut().set_diffuse_vec3(color);
    }
    pub fn set_uv_offset(&mut self, u: f32, v: f32) {
        self.model.set_uv_offset(u, v);
    }
}

impl Renderable for Skysphere {
    unsafe fn render(&self) {
        self.model.render_internal(Mat4::identity(), true, true);
    }
}
impl TransformRenderable for Skysphere {
    unsafe fn render_with_transform(&self, transform: Mat4) {
        self.model.render_internal(transform, true, true);
    }
}
