//! Port of `legacy/scene/component/SceneBuilder.java`.

#![allow(unsafe_op_in_unsafe_fn)]

use super::{Egg, Robot, Room, Smartphone, SwingingSpotlight};

use crate::{
    gmaths::Vec3,
    graphics::{
        basic::{Cube, Plane, Sphere},
        camera::Camera,
        lighting::LightLibrary,
        material::{Material, Texture, TextureLibrary},
        model::{Mesh, MeshLibrary, MeshLoader, Model, Skybox, Skysphere},
        shader::{Shader, ShaderLibrary},
    },
};
use std::{
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
};

#[derive(Debug)]
pub enum SceneResourceError {
    Io(std::io::Error),
    Image(image::ImageError),
    Mesh(String),
    MissingResource(String),
}

impl From<std::io::Error> for SceneResourceError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
impl From<image::ImageError> for SceneResourceError {
    fn from(error: image::ImageError) -> Self {
        Self::Image(error)
    }
}

pub struct SceneBuilder {
    pub camera: Rc<RefCell<Camera>>,
    pub light_library: Rc<RefCell<LightLibrary>>,
    pub shader_library: RefCell<ShaderLibrary>,
    pub texture_library: RefCell<TextureLibrary>,
    pub mesh_library: RefCell<MeshLibrary>,
    asset_root: PathBuf,
    default_material: Material,
    meshes: HashMap<String, Rc<Mesh>>,
    textures: HashMap<String, Rc<Texture>>,
    shaders: HashMap<String, Rc<Shader>>,
}

impl SceneBuilder {
    pub fn new(camera: Rc<RefCell<Camera>>) -> Self {
        Self {
            camera,
            light_library: Rc::new(RefCell::new(LightLibrary::new())),
            shader_library: RefCell::new(ShaderLibrary::new()),
            texture_library: RefCell::new(TextureLibrary::new()),
            mesh_library: RefCell::new(MeshLibrary::new()),
            asset_root: PathBuf::new(),
            default_material: Material::new(
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(1.0, 1.0, 1.0),
                16.0,
            ),
            meshes: HashMap::new(),
            textures: HashMap::new(),
            shaders: HashMap::new(),
        }
    }

    pub unsafe fn initialize(
        &mut self,
        asset_root: impl AsRef<Path>,
    ) -> Result<(), SceneResourceError> {
        // The Java application loads from its working directory and reports
        // paths such as `textures/white.jpg`, rather than `./textures/...`.
        // Preserve that observable form for the normal `.` asset root.
        self.asset_root = if asset_root.as_ref() == Path::new(".") {
            PathBuf::new()
        } else {
            asset_root.as_ref().to_path_buf()
        };
        let mut meshes = self.mesh_library.borrow_mut();
        self.meshes.insert(
            "plane".into(),
            meshes.create_mesh(&Plane::vertices(), &Plane::indices()),
        );
        self.meshes.insert(
            "cube".into(),
            meshes.create_mesh(&Cube::vertices(), &Cube::indices()),
        );
        self.meshes.insert(
            "sphere".into(),
            meshes.create_mesh(&Sphere::vertices(), &Sphere::indices()),
        );
        let phone = MeshLoader::load(&mut meshes, self.asset_root.join("meshes/smartphone.obj"))
            .map_err(|error| SceneResourceError::Mesh(format!("{error:?}")))?;
        self.meshes.insert("smartphone".into(), phone);
        drop(meshes);

        for (name, filename) in [
            ("default", "white.jpg"),
            ("floor", "Wood_Plank_vgwnadk_2K_Albedo.jpg"),
            ("floor_spec", "Wood_Plank_vgwnadk_2K_Roughness.jpg"),
            ("wall", "Wood_Other_ugclefmn_2K_Albedo.jpg"),
            ("wall_spec", "Wood_Other_ugclefmn_2K_Roughness.jpg"),
            ("door", "Decals_Wood_tjxofaws_2K_Albedo.jpg"),
            ("door_spec", "Decals_Wood_tjxofaws_2K_Roughness.jpg"),
            ("paint", "Paintings_Abstract_qirpc_2K_Albedo.jpg"),
            ("paint_spec", "Paintings_Abstract_qirpc_2K_Roughness.jpg"),
            ("frames", "Wood_Board_vigjfivg_2K_Albedo.jpg"),
            ("frames_spec", "Wood_Board_vigjfivg_2K_Roughness.jpg"),
            ("window", "iHlkbr8-mt-fuji-wallpaper.jpg"),
            ("snow", "snow.jpg"),
            ("robot_primary", "Metal_Painted_vbsieik_2K_Albedo.jpg"),
            (
                "robot_primary_spec",
                "Metal_Painted_vbsieik_2K_Roughness.jpg",
            ),
            ("robot_secondary", "Metal_td1kaean_2K_Albedo.jpg"),
            ("robot_secondary_spec", "Metal_td1kaean_2K_Roughness.jpg"),
            ("robot_accent", "Metal_Painted_ui1jfaady_2K_Albedo.jpg"),
            (
                "robot_accent_spec",
                "Metal_Painted_ui1jfaady_2K_Roughness.jpg",
            ),
            ("robot_wheel", "Misc_scgvcjop_2K_Albedo.jpg"),
            ("robot_wheel_spec", "Misc_scgvcjop_2K_Roughness.jpg"),
            ("egg", "Marble_Polished_ufojbjkl_2K_Albedo.jpg"),
            ("egg_spec", "Marble_Polished_ufojbjkl_2K_Specular.jpg"),
            ("egg_stand", "Marble_Polished_vdfjbeav_2K_Albedo.jpg"),
            ("egg_stand_spec", "Marble_Polished_vdfjbeav_2K_Specular.jpg"),
            ("phone", "homtom-ht7-released-02.jpg"),
            ("phone_spec", "white.jpg"),
            ("phone_stand", "Marble_Polished_ufojbixl_2K_Albedo.jpg"),
            (
                "phone_stand_spec",
                "Marble_Polished_ufojbixl_2K_Roughness.jpg",
            ),
            ("light_pole", "Wood_Plank_ulxqcedaw_2K_Albedo.jpg"),
            ("light_pole_spec", "Wood_Plank_ulxqcedaw_2K_Roughness.jpg"),
        ] {
            let texture = self
                .texture_library
                .borrow_mut()
                .load_texture(self.asset_root.join("textures").join(filename))?;
            self.textures.insert(name.into(), texture);
        }
        for (name, vertex, fragment) in [
            ("multilighting", "multilighting.vert", "multilighting.frag"),
            ("window_view", "window_view.vert", "window_view.frag"),
        ] {
            let shader = self.shader_library.borrow_mut().load_shader(
                self.asset_root.join("shaders").join(vertex),
                self.asset_root.join("shaders").join(fragment),
            )?;
            self.shaders.insert(name.into(), shader);
        }
        // Java creates the shared light-source shader directly inside
        // `LightLibrary`, so it has no `ShaderLibrary` progress line.
        let light_source = self.shader_library.borrow_mut().load_shader_silent(
            self.asset_root.join("shaders/light_source.vert"),
            self.asset_root.join("shaders/light_source.frag"),
        )?;
        self.shaders.insert("light_source".into(), light_source);
        Ok(())
    }

    pub const fn default_material(&self) -> Material {
        self.default_material
    }
    pub fn mesh(&self, name: &str) -> Result<Rc<Mesh>, SceneResourceError> {
        self.meshes
            .get(name)
            .cloned()
            .ok_or_else(|| SceneResourceError::MissingResource(name.into()))
    }
    pub fn texture(&self, name: &str) -> Result<Rc<Texture>, SceneResourceError> {
        self.textures
            .get(name)
            .cloned()
            .ok_or_else(|| SceneResourceError::MissingResource(name.into()))
    }
    pub fn shader(&self, name: &str) -> Result<Rc<Shader>, SceneResourceError> {
        self.shaders
            .get(name)
            .cloned()
            .ok_or_else(|| SceneResourceError::MissingResource(name.into()))
    }

    pub fn model(
        &self,
        material: Material,
        mesh: &str,
        textures: &[&str],
    ) -> Result<Model, SceneResourceError> {
        Ok(Model::new(
            self.camera.clone(),
            Some(self.light_library.clone()),
            self.shader("multilighting")?,
            material,
            self.mesh(mesh)?,
            textures
                .iter()
                .map(|name| self.texture(name))
                .collect::<Result<_, _>>()?,
        ))
    }
    pub fn model_with_shader(
        &self,
        shader: &str,
        material: Material,
        mesh: &str,
        textures: &[&str],
    ) -> Result<Model, SceneResourceError> {
        Ok(Model::new(
            self.camera.clone(),
            Some(self.light_library.clone()),
            self.shader(shader)?,
            material,
            self.mesh(mesh)?,
            textures
                .iter()
                .map(|name| self.texture(name))
                .collect::<Result<_, _>>()?,
        ))
    }

    /// Build the unlit visible mesh embedded by Java's point/spot light nodes.
    pub fn light_source_model(&self, material: Material) -> Result<Model, SceneResourceError> {
        Ok(Model::new(
            self.camera.clone(),
            None,
            self.shader("light_source")?,
            material,
            self.mesh("sphere")?,
            Vec::new(),
        ))
    }

    pub fn create_daylight(&self) -> Rc<RefCell<crate::graphics::lighting::DirectionalLightNode>> {
        self.light_library.borrow_mut().create_directional_light(
            "Daylight",
            Material::new(Vec3::default(), Vec3::default(), Vec3::default(), 16.0),
        )
    }

    pub unsafe fn create_skybox(&self) -> Result<Skybox, SceneResourceError> {
        let names = [
            "right_flipped.jpg",
            "left_flipped.jpg",
            "top_flipped.jpg",
            "bottom_flipped.jpg",
            "front_flipped.jpg",
            "back_flipped.jpg",
        ];
        let paths: Vec<_> = names
            .iter()
            .map(|name| self.asset_root.join("textures/skybox").join(name))
            .collect();
        let texture = self
            .texture_library
            .borrow_mut()
            .load_cubemap_with_parameters(
                &paths,
                gl::CLAMP_TO_EDGE as i32,
                gl::CLAMP_TO_EDGE as i32,
                gl::LINEAR as i32,
                gl::LINEAR as i32,
            )?;
        Ok(Skybox::new(
            self.camera.clone(),
            &mut self.shader_library.borrow_mut(),
            self.default_material,
            &mut self.mesh_library.borrow_mut(),
            texture,
        )?)
    }

    pub unsafe fn create_skysphere(&self) -> Result<Skysphere, SceneResourceError> {
        let texture = self.texture_library.borrow_mut().load_texture(
            self.asset_root
                .join("textures/SkyhighFluffycloudField4k.jpg"),
        )?;
        Ok(Skysphere::new(
            self.camera.clone(),
            &mut self.shader_library.borrow_mut(),
            self.default_material,
            &mut self.mesh_library.borrow_mut(),
            texture,
        )?)
    }

    pub fn create_room(&self) -> Result<Room, SceneResourceError> {
        Room::new(self)
    }
    pub fn create_robot(&self) -> Result<Robot, SceneResourceError> {
        Robot::new(self)
    }
    pub fn create_smartphone(&self) -> Result<Smartphone, SceneResourceError> {
        Smartphone::new(self)
    }
    pub fn create_swinging_spotlight(&self) -> Result<SwingingSpotlight, SceneResourceError> {
        SwingingSpotlight::new(self)
    }
    pub fn create_egg(&self) -> Result<Egg, SceneResourceError> {
        Egg::new(self)
    }
}
