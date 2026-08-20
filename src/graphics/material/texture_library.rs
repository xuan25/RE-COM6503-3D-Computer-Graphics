//! Port of `legacy/graphics/material/TextureLibrary.java`.

#![allow(unsafe_op_in_unsafe_fn)]

use super::Texture;
use std::{path::Path, rc::Rc};

#[derive(Default)]
pub struct TextureLibrary {
    loaded_textures: Vec<Rc<Texture>>,
}

impl TextureLibrary {
    pub const fn new() -> Self {
        Self {
            loaded_textures: Vec::new(),
        }
    }

    pub unsafe fn load_texture(
        &mut self,
        filename: impl AsRef<Path>,
    ) -> Result<Rc<Texture>, image::ImageError> {
        self.load_texture_with_parameters(
            filename,
            gl::REPEAT as i32,
            gl::REPEAT as i32,
            gl::LINEAR as i32,
            gl::LINEAR as i32,
        )
    }

    pub unsafe fn load_texture_with_parameters(
        &mut self,
        filename: impl AsRef<Path>,
        wrapping_s: i32,
        wrapping_t: i32,
        filter_min: i32,
        filter_mag: i32,
    ) -> Result<Rc<Texture>, image::ImageError> {
        println!("Loading texture - {}", filename.as_ref().display());
        let texture = Rc::new(Texture::from_file(
            filename, wrapping_s, wrapping_t, filter_min, filter_mag,
        )?);
        self.loaded_textures.push(Rc::clone(&texture));
        Ok(texture)
    }

    pub unsafe fn load_cubemap<P: AsRef<Path>>(
        &mut self,
        filenames: &[P],
    ) -> Result<Rc<Texture>, image::ImageError> {
        self.load_cubemap_with_parameters(
            filenames,
            gl::REPEAT as i32,
            gl::REPEAT as i32,
            gl::LINEAR as i32,
            gl::LINEAR as i32,
        )
    }

    pub unsafe fn load_cubemap_with_parameters<P: AsRef<Path>>(
        &mut self,
        filenames: &[P],
        wrapping_s: i32,
        wrapping_t: i32,
        filter_min: i32,
        filter_mag: i32,
    ) -> Result<Rc<Texture>, image::ImageError> {
        let names = filenames
            .iter()
            .map(|filename| filename.as_ref().display().to_string())
            .collect::<Vec<_>>();
        println!("Loading texture - [{}]", names.join(", "));
        let texture = Rc::new(Texture::cubemap_from_files(
            filenames, wrapping_s, wrapping_t, filter_min, filter_mag,
        )?);
        self.loaded_textures.push(Rc::clone(&texture));
        Ok(texture)
    }

    pub fn unload(&mut self, texture: &Rc<Texture>) -> bool {
        if let Some(index) = self
            .loaded_textures
            .iter()
            .position(|item| Rc::ptr_eq(item, texture))
        {
            self.loaded_textures.remove(index);
            true
        } else {
            false
        }
    }

    pub fn unload_all(&mut self) {
        self.loaded_textures.clear();
    }
}
