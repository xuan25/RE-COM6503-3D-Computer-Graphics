//! GLFW rendering-state counterpart of `legacy/MuseumGL.java`.

use crate::{
    gmaths::mat4_transform,
    graphics::{
        camera::{Camera, Movement},
        interfaces::Renderable,
        lighting::DirectionalLightNode,
        model::Model,
        node::{BasicNode, Node, NodeLink},
        offscreen::{HdrOffscreenBuffer, MsaaOffscreenBuffer, OffscreenBuffer},
    },
    scene::{
        animator::{
            EnvironmentAnimator, RobotAnimator, SwingingSpotlightAnimator, interfaces::Animator,
        },
        component::{
            SceneBuilder,
            interfaces::{Component, ComponentLink},
        },
    },
};
use std::{cell::RefCell, rc::Rc};

/// The multisample count used by the Java `MSAAOffScreenBuffer`.
pub const MSAA_RATE: i32 = 16;

/// Direct counterpart of the nested `MuseumGL.SkyType` enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkyType {
    A,
    B,
}

/// Retained key state, deliberately separate from GLFW event delivery just as
/// `MuseumGL.CameraMovementState` is separate from the Swing key listeners.
#[derive(Default)]
pub struct CameraMovementState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

pub struct MuseumGL {
    pub camera_movement_state: CameraMovementState,
    pub sky_type: SkyType,
    pub camera: Rc<RefCell<Camera>>,
    pub root: BasicNode,
    pub skybox: Rc<RefCell<crate::graphics::model::Skybox>>,
    pub skysphere: Rc<RefCell<crate::graphics::model::Skysphere>>,
    pub room: Rc<RefCell<crate::scene::component::Room>>,
    /// `MuseumGL.daylightNode` in the Java project.  Retaining it here keeps
    /// the scene component addressable after its node is attached to `root`.
    pub daylight_node: Rc<RefCell<DirectionalLightNode>>,
    pub robot: Rc<RefCell<crate::scene::component::Robot>>,
    /// Direct counterpart of Java's `MuseumGL.smartphone` field.
    pub smartphone: Rc<RefCell<crate::scene::component::Smartphone>>,
    pub spotlight: Rc<RefCell<crate::scene::component::SwingingSpotlight>>,
    /// Direct counterpart of Java's `MuseumGL.egg` field.
    pub egg: Rc<RefCell<crate::scene::component::Egg>>,
    pub robot_animator: RobotAnimator,
    pub spotlight_animator: SwingingSpotlightAnimator,
    pub environment_animator: EnvironmentAnimator,
    pub msaa: MsaaOffscreenBuffer,
    pub hdr: HdrOffscreenBuffer,
    render_width: i32,
    render_height: i32,
}

impl MuseumGL {
    /// Equivalent to `MuseumGL.initScene`, including all retained scene state.
    pub unsafe fn initialize(
        camera: Rc<RefCell<Camera>>,
        asset_root: &str,
        width: i32,
        height: i32,
    ) -> Result<Self, String> {
        camera
            .borrow_mut()
            .set_perspective_matrix(mat4_transform::perspective(
                45.,
                width as f32 / height as f32,
            ));
        let mut builder = SceneBuilder::new(camera.clone());
        unsafe {
            builder
                .initialize(asset_root)
                .map_err(|error| format!("{error:?}"))?
        };
        let skybox = Rc::new(RefCell::new(unsafe {
            builder
                .create_skybox()
                .map_err(|error| format!("{error:?}"))?
        }));
        let skysphere = Rc::new(RefCell::new(unsafe {
            builder
                .create_skysphere()
                .map_err(|error| format!("{error:?}"))?
        }));
        let room = Rc::new(RefCell::new(
            builder
                .create_room()
                .map_err(|error| format!("{error:?}"))?,
        ));
        let robot = Rc::new(RefCell::new(
            builder
                .create_robot()
                .map_err(|error| format!("{error:?}"))?,
        ));
        let phone = Rc::new(RefCell::new(
            builder
                .create_smartphone()
                .map_err(|error| format!("{error:?}"))?,
        ));
        let spotlight = Rc::new(RefCell::new(
            builder
                .create_swinging_spotlight()
                .map_err(|error| format!("{error:?}"))?,
        ));
        let egg = Rc::new(RefCell::new(
            builder.create_egg().map_err(|error| format!("{error:?}"))?,
        ));
        let daylight = builder.create_daylight();
        robot
            .borrow_mut()
            .node_mut()
            .set_center_translation(-10., 0., -10.);
        phone
            .borrow_mut()
            .node_mut()
            .set_center_translation(10., 0., -10.);
        spotlight
            .borrow_mut()
            .node_mut()
            .set_center_translation(13., 0., 5.);
        let mut root = BasicNode::new("Scene root");
        root.add_child(Box::new(ComponentLink::new(room.clone())));
        root.add_child(Box::new(ComponentLink::new(robot.clone())));
        root.add_child(Box::new(ComponentLink::new(phone.clone())));
        root.add_child(Box::new(ComponentLink::new(spotlight.clone())));
        root.add_child(Box::new(ComponentLink::new(egg.clone())));
        root.add_child(Box::new(NodeLink::new(daylight.clone())));
        root.update();
        let robot_animator = RobotAnimator::new(robot.clone());
        let spotlight_animator = SwingingSpotlightAnimator::new(spotlight.clone());
        let environment_animator = EnvironmentAnimator::new(
            room.borrow().window_view_node(),
            skybox.clone(),
            skysphere.clone(),
            daylight.clone(),
        );
        let mut msaa = MsaaOffscreenBuffer::new(MSAA_RATE);
        let mut hdr = unsafe { HdrOffscreenBuffer::new().map_err(|error| error.to_string())? };
        unsafe {
            msaa.reshape(width, height)?;
            hdr.reshape(width, height)?;
        }
        Ok(Self {
            camera_movement_state: CameraMovementState::default(),
            sky_type: SkyType::B,
            camera,
            root,
            skybox,
            skysphere,
            room,
            daylight_node: daylight,
            robot,
            smartphone: phone,
            spotlight,
            egg,
            robot_animator,
            spotlight_animator,
            environment_animator,
            msaa,
            hdr,
            render_width: width,
            render_height: height,
        })
    }

    /// Exact Rust equivalent of `MuseumGL.updateCamera(deltaTime)`.
    pub fn update_camera(&self, camera: &Rc<RefCell<Camera>>, delta_time: f32) {
        let mut camera = camera.borrow_mut();
        let state = &self.camera_movement_state;
        if state.left {
            camera.move_camera(Movement::Left, delta_time);
        }
        if state.right {
            camera.move_camera(Movement::Right, delta_time);
        }
        if state.up {
            camera.move_camera(Movement::Up, delta_time);
        }
        if state.down {
            camera.move_camera(Movement::Down, delta_time);
        }
        if state.forward {
            camera.move_camera(Movement::Forward, delta_time);
        }
        if state.backward {
            camera.move_camera(Movement::Backward, delta_time);
        }
    }

    pub fn advance(&mut self, seconds: f32) {
        self.update_camera(&self.camera, seconds);
        self.environment_animator.forward(seconds as f64);
        self.spotlight_animator.forward(seconds as f64);
        self.robot_animator.forward(seconds as f64);
    }

    /// Textual scene graph used by the original control panel's
    /// "Debug: Print scene hierarchy" button.
    pub fn scene_hierarchy(&self) -> String {
        self.root.hierarchy_string()
    }

    pub unsafe fn reshape(&mut self, width: i32, height: i32) -> Result<(), String> {
        self.camera
            .borrow_mut()
            .set_perspective_matrix(mat4_transform::perspective(
                45.,
                width as f32 / height as f32,
            ));
        unsafe {
            self.msaa.reshape(width, height)?;
            self.hdr.reshape(width, height)?;
        }
        self.render_width = width;
        self.render_height = height;
        Ok(())
    }

    /// Render the GL canvas, then composite it into the non-control-panel part
    /// of the application window.  Swing's `BorderLayout.WEST` similarly
    /// reserves the left side before the JOGL canvas is painted.
    pub unsafe fn render(&self, output_x: i32) {
        unsafe {
            // Dear ImGui changes OpenGL state while drawing the left control
            // panel.  The JOGL application had no second renderer between
            // frames, so restore its canonical scene state explicitly before
            // rendering the next off-screen scene.  In particular, a stale
            // disabled depth test lets the sky sphere overwrite the museum.
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::FrontFace(gl::CCW);
            self.msaa.use_buffer();
            gl::Viewport(0, 0, self.render_width, self.render_height);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            if Model::debug_wireframe() {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                gl::Disable(gl::CULL_FACE);
            }
            self.root.render();
            // Java restores these two states after every scene-root render,
            // not only after a wireframe pass.
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl::Enable(gl::CULL_FACE);
            if !Model::debug_wireframe() && self.sky_type == SkyType::A {
                self.skybox.borrow().render();
            } else if !Model::debug_wireframe() {
                self.skysphere.borrow().render();
            }
            self.msaa.render(self.hdr.framebuffer_id());
            gl::Viewport(output_x, 0, self.render_width, self.render_height);
            self.hdr.render(0);
        }
    }

    /// Counterpart of `MuseumGL.dispose(GLAutoDrawable)`.  Consuming the
    /// renderer guarantees every OpenGL resource is dropped while its GLFW
    /// context is still current.
    pub fn dispose(mut self) {
        self.root.dispose();
    }
}
