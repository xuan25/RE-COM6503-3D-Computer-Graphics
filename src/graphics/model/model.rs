//! Port of `legacy/graphics/model/Model.java`.

#![allow(unsafe_op_in_unsafe_fn)]

use super::Mesh;
use crate::{
    gmaths::{Mat4, Vec2},
    graphics::{
        camera::Camera,
        interfaces::{Renderable, TransformRenderable},
        lighting::{Attenuated, Directional, LightLibrary, Lighting, Positional, Ranged},
        material::{Material, Texture},
        shader::Shader,
    },
};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};

/// Java's `Model.debugShader` and `Model.debugWireframe` are process-wide
/// rendering flags, not state belonging to a particular mesh.
static DEBUG_SHADER: AtomicBool = AtomicBool::new(false);
static DEBUG_WIREFRAME: AtomicBool = AtomicBool::new(false);

pub struct Model {
    mesh: Rc<Mesh>,
    textures: Vec<Rc<Texture>>,
    material: Material,
    shader: Rc<Shader>,
    camera: Rc<RefCell<Camera>>,
    lights: Option<Rc<RefCell<LightLibrary>>>,
    uv_scale: Vec2,
    uv_offset: Vec2,
}

impl Model {
    pub fn new(
        camera: Rc<RefCell<Camera>>,
        lights: Option<Rc<RefCell<LightLibrary>>>,
        shader: Rc<Shader>,
        material: Material,
        mesh: Rc<Mesh>,
        textures: Vec<Rc<Texture>>,
    ) -> Self {
        Self {
            mesh,
            textures,
            material,
            shader,
            camera,
            lights,
            uv_scale: Vec2::new(1.0, 1.0),
            uv_offset: Vec2::new(0.0, 0.0),
        }
    }

    pub fn material(&self) -> Material {
        self.material
    }
    pub fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }
    pub fn set_uv_scale(&mut self, u: f32, v: f32) {
        self.uv_scale = Vec2::new(u, v);
    }
    pub fn set_uv_offset(&mut self, u: f32, v: f32) {
        self.uv_offset = Vec2::new(u, v);
    }
    pub fn set_debug_shader(enabled: bool) {
        DEBUG_SHADER.store(enabled, Ordering::Relaxed);
    }
    pub fn set_debug_wireframe(enabled: bool) {
        DEBUG_WIREFRAME.store(enabled, Ordering::Relaxed);
    }
    pub fn debug_wireframe() -> bool {
        DEBUG_WIREFRAME.load(Ordering::Relaxed)
    }

    unsafe fn setup_transform(&self, model_matrix: Mat4, remove_view_translation: bool) {
        let mut camera = self.camera.borrow_mut();
        let mut view = camera.view_matrix();
        if remove_view_translation {
            view.values[12] = 0.0;
            view.values[13] = 0.0;
            view.values[14] = 0.0;
        }
        let mvp = Mat4::multiply(
            camera.perspective_matrix(),
            Mat4::multiply(view, model_matrix),
        );
        if !remove_view_translation {
            self.shader.set_float_array("model", &model_matrix.values);
        }
        self.shader.set_float_array("mvpMatrix", &mvp.values);
        if !remove_view_translation {
            self.shader.set_vec3("viewPos", camera.position());
        }
    }

    unsafe fn setup_lighting(&self) {
        let Some(lights) = &self.lights else {
            return;
        };
        let lights = lights.borrow();
        self.shader
            .set_int("numDirLights", lights.directional_lights().len() as i32);
        for (index, light) in lights.directional_lights().iter().enumerate() {
            let light = light.borrow();
            let name = format!("dirLights[{index}]");
            self.shader
                .set_vec3(&format!("{name}.direction"), light.direction());
            self.shader
                .set_vec3(&format!("{name}.ambient"), light.ambient());
            self.shader
                .set_vec3(&format!("{name}.diffuse"), light.diffuse());
            self.shader
                .set_vec3(&format!("{name}.specular"), light.specular());
        }
        self.shader
            .set_int("numPointLights", lights.point_lights().len() as i32);
        for (index, light) in lights.point_lights().iter().enumerate() {
            let light = light.borrow();
            let name = format!("pointLights[{index}]");
            self.shader
                .set_vec3(&format!("{name}.position"), light.position());
            self.shader
                .set_float(&format!("{name}.constant"), light.attenuation_constant());
            self.shader
                .set_float(&format!("{name}.linear"), light.attenuation_linear());
            self.shader
                .set_float(&format!("{name}.quadratic"), light.attenuation_quadratic());
            self.shader
                .set_vec3(&format!("{name}.ambient"), light.ambient());
            self.shader
                .set_vec3(&format!("{name}.diffuse"), light.diffuse());
            self.shader
                .set_vec3(&format!("{name}.specular"), light.specular());
        }
        self.shader
            .set_int("numSpotLights", lights.spot_lights().len() as i32);
        for (index, light) in lights.spot_lights().iter().enumerate() {
            let light = light.borrow();
            let name = format!("spotLights[{index}]");
            self.shader
                .set_vec3(&format!("{name}.position"), light.position());
            self.shader
                .set_vec3(&format!("{name}.direction"), light.direction());
            self.shader
                .set_float(&format!("{name}.cutOff"), light.cut_off_coefficient());
            self.shader.set_float(
                &format!("{name}.outerCutOff"),
                light.outer_cut_off_coefficient(),
            );
            self.shader
                .set_float(&format!("{name}.constant"), light.attenuation_constant());
            self.shader
                .set_float(&format!("{name}.linear"), light.attenuation_linear());
            self.shader
                .set_float(&format!("{name}.quadratic"), light.attenuation_quadratic());
            self.shader
                .set_vec3(&format!("{name}.ambient"), light.ambient());
            self.shader
                .set_vec3(&format!("{name}.diffuse"), light.diffuse());
            self.shader
                .set_vec3(&format!("{name}.specular"), light.specular());
        }
    }

    unsafe fn setup_material(&self) {
        self.shader
            .set_vec3("material.ambient", self.material.ambient());
        self.shader
            .set_vec3("material.diffuse", self.material.diffuse());
        self.shader
            .set_vec3("material.specular", self.material.specular());
        self.shader
            .set_float("material.shininess", self.material.shininess());
    }

    unsafe fn setup_textures(&self) {
        for (index, texture) in self.textures.iter().enumerate() {
            self.shader
                .set_int(&format!("texture{index}"), index as i32);
            gl::ActiveTexture(gl::TEXTURE0 + index as u32);
            gl::BindTexture(texture.target(), texture.id());
        }
    }

    unsafe fn setup_uv(&self) {
        self.shader
            .set_vec2("uvScale", self.uv_scale.x, self.uv_scale.y);
        self.shader
            .set_vec2("uvOffset", self.uv_offset.x, self.uv_offset.y);
    }

    pub(crate) unsafe fn render_internal(&self, model_matrix: Mat4, sky: bool, uv: bool) {
        self.shader.use_program();
        self.setup_transform(model_matrix, sky);
        if !sky {
            self.setup_lighting();
        }
        self.setup_material();
        self.setup_textures();
        if uv {
            self.setup_uv();
        }

        // Matches `Model.renderMesh()` in the Java project: validation is
        // performed after the complete draw state and VAO are bound, directly
        // before the indexed draw call.
        self.mesh.bind();
        if DEBUG_SHADER.load(Ordering::Relaxed) {
            if let Err(log) = self.shader.validate() {
                eprintln!("{log}");
            }
        }
        self.mesh.draw_elements();
        self.mesh.unbind();
    }
}

impl Renderable for Model {
    unsafe fn render(&self) {
        self.render_internal(Mat4::identity(), false, true);
    }
}

impl TransformRenderable for Model {
    unsafe fn render_with_transform(&self, transform: Mat4) {
        self.render_internal(transform, false, true);
    }
}

#[cfg(test)]
mod tests {
    use super::Model;

    #[test]
    fn wireframe_flag_is_global_like_the_java_model_flag() {
        Model::set_debug_wireframe(true);
        assert!(Model::debug_wireframe());
        Model::set_debug_wireframe(false);
        assert!(!Model::debug_wireframe());
    }
}
