//! Port of `legacy/graphics/model/MeshLibrary.java`.

#![allow(unsafe_op_in_unsafe_fn)]

use super::Mesh;
use std::rc::Rc;

#[derive(Default)]
pub struct MeshLibrary {
    loaded_meshes: Vec<Rc<Mesh>>,
}

impl MeshLibrary {
    pub const fn new() -> Self {
        Self {
            loaded_meshes: Vec::new(),
        }
    }

    pub unsafe fn create_mesh(&mut self, vertices: &[f32], indices: &[u32]) -> Rc<Mesh> {
        let mesh = Rc::new(Mesh::new(vertices, indices));
        self.loaded_meshes.push(Rc::clone(&mesh));
        mesh
    }

    pub fn unload(&mut self, mesh: &Rc<Mesh>) -> bool {
        if let Some(index) = self
            .loaded_meshes
            .iter()
            .position(|item| Rc::ptr_eq(item, mesh))
        {
            self.loaded_meshes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn unload_all(&mut self) {
        self.loaded_meshes.clear();
    }
}
