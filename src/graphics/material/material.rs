//! Port of `legacy/graphics/material/Material.java`.

use crate::gmaths::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
    emission: Vec3,
    shininess: f32,
}

impl Material {
    pub const DEFAULT_AMBIENT: Vec3 = Vec3::new(0.2, 0.2, 0.2);
    pub const DEFAULT_DIFFUSE: Vec3 = Vec3::new(0.8, 0.8, 0.8);
    pub const DEFAULT_SPECULAR: Vec3 = Vec3::new(0.5, 0.5, 0.5);
    pub const DEFAULT_EMISSION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    pub const DEFAULT_SHININESS: f32 = 32.0;

    pub const fn new(ambient: Vec3, diffuse: Vec3, specular: Vec3, shininess: f32) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            emission: Self::DEFAULT_EMISSION,
            shininess,
        }
    }

    pub fn set_ambient(&mut self, red: f32, green: f32, blue: f32) {
        self.ambient = Vec3::new(red, green, blue);
    }

    pub fn set_ambient_vec3(&mut self, rgb: Vec3) {
        self.ambient = rgb;
    }

    pub const fn ambient(&self) -> Vec3 {
        self.ambient
    }

    pub fn set_diffuse(&mut self, red: f32, green: f32, blue: f32) {
        self.diffuse = Vec3::new(red, green, blue);
    }

    pub fn set_diffuse_vec3(&mut self, rgb: Vec3) {
        self.diffuse = rgb;
    }

    pub const fn diffuse(&self) -> Vec3 {
        self.diffuse
    }

    pub fn set_specular(&mut self, red: f32, green: f32, blue: f32) {
        self.specular = Vec3::new(red, green, blue);
    }

    pub fn set_specular_vec3(&mut self, rgb: Vec3) {
        self.specular = rgb;
    }

    pub const fn specular(&self) -> Vec3 {
        self.specular
    }

    pub fn set_emission(&mut self, red: f32, green: f32, blue: f32) {
        self.emission = Vec3::new(red, green, blue);
    }

    pub fn set_emission_vec3(&mut self, rgb: Vec3) {
        self.emission = rgb;
    }

    pub const fn emission(&self) -> Vec3 {
        self.emission
    }

    pub fn set_shininess(&mut self, shininess: f32) {
        self.shininess = shininess;
    }

    pub const fn shininess(&self) -> f32 {
        self.shininess
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new(
            Self::DEFAULT_AMBIENT,
            Self::DEFAULT_DIFFUSE,
            Self::DEFAULT_SPECULAR,
            Self::DEFAULT_SHININESS,
        )
    }
}
