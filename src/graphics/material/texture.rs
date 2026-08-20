//! Port of `legacy/graphics/material/Texture.java`.

#![allow(unsafe_op_in_unsafe_fn)]

use std::path::Path;

pub struct Texture {
    id: u32,
    target: u32,
}

impl Texture {
    pub unsafe fn from_file(
        filename: impl AsRef<Path>,
        wrapping_s: i32,
        wrapping_t: i32,
        filter_min: i32,
        filter_mag: i32,
    ) -> Result<Self, image::ImageError> {
        // `JPEGImage.getData()` in the JOGL source supplies a GL-oriented
        // pixel buffer. `image` exposes decoded JPEG rows top-to-bottom, so
        // make the conversion explicitly for every 2D surface.  This keeps
        // the legacy UV data of Plane, Cube, Sphere and OBJ meshes unchanged.
        let image = image::open(filename)?.flipv().to_rgb8();
        let (width, height) = image.dimensions();
        let mut id = 0;
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrapping_s);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrapping_t);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter_min);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter_mag);
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::SRGB as i32,
            width as i32,
            height as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            image.as_ptr().cast(),
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::BindTexture(gl::TEXTURE_2D, 0);
        Ok(Self {
            id,
            target: gl::TEXTURE_2D,
        })
    }

    pub unsafe fn cubemap_from_files<P: AsRef<Path>>(
        filenames: &[P],
        wrapping_s: i32,
        wrapping_t: i32,
        filter_min: i32,
        filter_mag: i32,
    ) -> Result<Self, image::ImageError> {
        assert_eq!(filenames.len(), 6, "a cubemap needs six faces");
        let mut id = 0;
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, id);
        for (index, filename) in filenames.iter().enumerate() {
            // Cubemap faces use the same JOGL JPEG upload convention as 2D
            // textures.  The source filenames being `*_flipped.jpg` describe
            // their face orientation, not their in-memory row order.
            let image = image::open(filename)?.flipv().to_rgb8();
            let (width, height) = image.dimensions();
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexImage2D(
                gl::TEXTURE_CUBE_MAP_POSITIVE_X + index as u32,
                0,
                gl::SRGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                image.as_ptr().cast(),
            );
        }
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, wrapping_s);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, wrapping_t);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, filter_min);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, filter_mag);
        gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);
        gl::TexParameteri(
            gl::TEXTURE_CUBE_MAP,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
        Ok(Self {
            id,
            target: gl::TEXTURE_CUBE_MAP,
        })
    }

    pub const fn id(&self) -> u32 {
        self.id
    }

    pub const fn target(&self) -> u32 {
        self.target
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) }
    }
}
