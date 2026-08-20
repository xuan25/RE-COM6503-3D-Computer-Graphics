//! GLFW + Dear ImGui counterpart of `legacy/MuseumControlPanel.java`.

use crate::{
    gmaths::Vec3,
    graphics::{camera::Camera, offscreen::HdrOffscreenBuffer},
    museum_gl::{MuseumGL, SkyType},
    scene::{
        animator::{EnvironmentAnimator, RobotAnimator},
        component::{Room, SwingingSpotlight, interfaces::Component},
    },
};
use imgui::{Condition, Ui};
use std::{cell::RefCell, rc::Rc};

/// Width reserved by the original `BorderLayout.WEST` control panel.
pub const CONTROL_PANEL_WIDTH: i32 = 275;

#[derive(Clone, Copy)]
pub enum CameraPreset {
    Reset,
    Closer,
    Window,
    Smartphone,
    Spotlight,
    Egg,
    Robot,
    X,
    Y,
    Z,
}

/// Selection maintained by the two camera-keymap buttons in the original
/// `MuseumControlPanel`.  Keeping it in the control-panel model makes the
/// enabled/disabled button state explicit instead of scattering it across the
/// GLFW event loop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CameraKeymap {
    My,
    Steve,
}

pub struct MuseumControlPanel {
    pub environment_intensity: f32,
    pub room_intensity: f32,
    pub spotlight_intensity: f32,
    pub exposure: f32,
    pub room_light_rotation: f32,
    pub debug_shader: bool,
    pub wireframe: bool,
    pub sky_type: SkyType,
    pub camera_keymap: CameraKeymap,
}
impl Default for MuseumControlPanel {
    fn default() -> Self {
        Self {
            environment_intensity: 1.,
            room_intensity: 1.,
            spotlight_intensity: 1.,
            exposure: 2.5,
            room_light_rotation: 0.,
            debug_shader: false,
            wireframe: false,
            // Java starts with SkyType.B and therefore disables the B button.
            sky_type: SkyType::B,
            // Java attaches Steve's Arrows/AZ listener at startup.
            camera_keymap: CameraKeymap::Steve,
        }
    }
}
impl MuseumControlPanel {
    /// Draw the GLFW/OpenGL counterpart of the original Swing side panel.
    ///
    /// The mutable scene owner is passed in deliberately: it is the Rust
    /// equivalent of the Java panel retaining `glEventListener`, while leaving
    /// the control state and every action in this dedicated type.
    pub fn draw(&mut self, ui: &Ui, scene: &mut MuseumGL, should_close: &mut bool) {
        ui.window("Museum controls")
            // The original panel is a fixed BorderLayout.WEST sidebar.
            .position([0., 0.], Condition::Always)
            .size(
                [CONTROL_PANEL_WIDTH as f32, ui.io().display_size[1]],
                Condition::Always,
            )
            .movable(false)
            .resizable(false)
            .collapsible(false)
            .always_vertical_scrollbar(true)
            .build(|| {
                ui.separator();
                ui.text("Camera");
                {
                    // Swing placed each group in its own JPanel.  Dear ImGui
                    // uses one ID namespace for this window, so scope every
                    // group explicitly: both Camera and Lighting have a
                    // visible "Spotlight" control.
                    let _camera_ids = ui.push_id("camera-controls");
                    if ui.button("Reset camera") {
                        self.set_camera(
                            &scene.camera,
                            CameraPreset::Reset,
                            scene.robot.borrow().node().center_translation(),
                        );
                    }
                    if ui.button("Closer") {
                        self.set_camera(
                            &scene.camera,
                            CameraPreset::Closer,
                            scene.robot.borrow().node().center_translation(),
                        );
                    }
                    for (label, preset) in [
                        ("Window", CameraPreset::Window),
                        ("Smartphone", CameraPreset::Smartphone),
                        ("Spotlight", CameraPreset::Spotlight),
                        ("Egg", CameraPreset::Egg),
                        ("Robot", CameraPreset::Robot),
                        ("Camera X", CameraPreset::X),
                        ("Camera Y", CameraPreset::Y),
                        ("Camera Z", CameraPreset::Z),
                    ] {
                        if ui.button(label) {
                            self.set_camera(
                                &scene.camera,
                                preset,
                                scene.robot.borrow().node().center_translation(),
                            );
                        }
                    }
                }

                ui.separator();
                ui.text("Robot pose");
                {
                    let _robot_pose_ids = ui.push_id("robot-pose-controls");
                    for pose in 1..=5 {
                        if ui.button(format!("Pose {pose}")) {
                            self.pose(&mut scene.robot_animator, pose);
                        }
                    }
                }

                ui.separator();
                ui.text("Lighting");
                {
                    let _lighting_ids = ui.push_id("lighting-controls");
                    let mut environment = self.environment_intensity;
                    if ui.slider("Environment", 0., 1., &mut environment) {
                        self.set_environment_intensity(
                            &mut scene.environment_animator,
                            environment,
                        );
                    }
                    let mut room = self.room_intensity;
                    if ui.slider("Room", 0., 1., &mut room) {
                        self.set_room_intensity(&mut scene.room.borrow_mut(), room);
                    }
                    let mut spotlight = self.spotlight_intensity;
                    if ui.slider("Spotlight", 0., 1., &mut spotlight) {
                        self.set_spotlight_intensity(&mut scene.spotlight.borrow_mut(), spotlight);
                    }
                    let mut exposure = self.exposure / 5.;
                    if ui.slider("Render Exposure", 0., 1., &mut exposure) {
                        self.set_exposure(&mut scene.hdr, exposure);
                    }
                    let mut rotation = self.room_light_rotation;
                    if ui.slider("Room Light Rotation", 0., 1., &mut rotation) {
                        self.set_room_light_rotation(&mut scene.room.borrow_mut(), rotation);
                    }
                }

                ui.separator();
                ui.text("Sky");
                {
                    let _sky_ids = ui.push_id("sky-controls");
                    // Swing disables the currently selected sky button rather
                    // than presenting a radio-button group.
                    ui.disabled(self.sky_type == SkyType::A, || {
                        if ui.button("A: Box") {
                            scene.sky_type = SkyType::A;
                            self.set_sky_type(SkyType::A);
                        }
                    });
                    ui.disabled(self.sky_type == SkyType::B, || {
                        if ui.button("B: Sphere") {
                            scene.sky_type = SkyType::B;
                            self.set_sky_type(SkyType::B);
                        }
                    });
                }

                ui.separator();
                ui.text("Misc");
                {
                    let _misc_ids = ui.push_id("misc-controls");
                    // Match Java's mutually exclusive enabled/disabled pair.
                    ui.disabled(self.camera_keymap == CameraKeymap::My, || {
                        if ui.button("Camera keymap: WASD/EQ") {
                            self.set_camera_keymap(CameraKeymap::My);
                        }
                    });
                    ui.disabled(self.camera_keymap == CameraKeymap::Steve, || {
                        if ui.button("Camera keymap: Arrows/AZ") {
                            self.set_camera_keymap(CameraKeymap::Steve);
                        }
                    });
                    if ui.button("Debug: Print scene hierarchy") {
                        // Matches the Swing control panel's diagnostic output.
                        println!("******** Scene Hierarchy ********");
                        println!("{}", scene.scene_hierarchy());
                    }
                    let mut debug_shader = self.debug_shader;
                    if ui.checkbox("Debug: Shader", &mut debug_shader) {
                        self.set_debug_shader(debug_shader);
                    }
                    let mut wireframe = self.wireframe;
                    if ui.checkbox("Debug: Wireframe", &mut wireframe) {
                        self.set_wireframe(wireframe);
                    }
                    if ui.button("Quit") {
                        *should_close = true;
                    }
                }
            });
    }

    pub fn set_environment_intensity(
        &mut self,
        animator: &mut EnvironmentAnimator,
        intensity: f32,
    ) {
        self.environment_intensity = intensity.clamp(0., 1.);
        animator.set_custom_lighting_intensity(self.environment_intensity);
    }
    pub fn set_room_intensity(&mut self, room: &mut Room, intensity: f32) {
        self.room_intensity = intensity.clamp(0., 1.);
        room.set_custom_lighting_intensity(self.room_intensity);
    }
    pub fn set_spotlight_intensity(&mut self, spotlight: &mut SwingingSpotlight, intensity: f32) {
        self.spotlight_intensity = intensity.clamp(0., 1.);
        spotlight.set_custom_lighting_intensity(self.spotlight_intensity);
    }
    pub fn set_exposure(&mut self, hdr: &mut HdrOffscreenBuffer, ratio: f32) {
        self.exposure = ratio.clamp(0., 1.) * 5.;
        hdr.set_exposure(self.exposure);
    }
    pub fn set_room_light_rotation(&mut self, room: &mut Room, ratio: f32) {
        self.room_light_rotation = ratio.clamp(0., 1.);
        room.set_light_group_rotation(self.room_light_rotation * 90.);
    }
    pub fn set_wireframe(&mut self, enabled: bool) {
        self.wireframe = enabled;
        crate::graphics::model::Model::set_debug_wireframe(enabled);
    }
    pub fn set_debug_shader(&mut self, enabled: bool) {
        self.debug_shader = enabled;
        crate::graphics::model::Model::set_debug_shader(enabled);
    }
    /// Equivalent to clicking either `A: Box` or `B: Sphere`.  A repeated
    /// click is harmless, exactly as clicking the disabled Swing button was.
    pub fn set_sky_type(&mut self, sky_type: SkyType) -> bool {
        let changed = self.sky_type != sky_type;
        self.sky_type = sky_type;
        changed
    }
    /// Equivalent to clicking either camera-keymap button.
    pub fn set_camera_keymap(&mut self, keymap: CameraKeymap) -> bool {
        let changed = self.camera_keymap != keymap;
        self.camera_keymap = keymap;
        changed
    }
    pub fn set_camera(
        &self,
        camera: &Rc<RefCell<Camera>>,
        preset: CameraPreset,
        robot_position: Vec3,
    ) {
        let mut c = camera.borrow_mut();
        match preset {
            CameraPreset::Reset => c.set_camera(
                Vec3::new(40., 32., 50.),
                Vec3::new(-3., 2., 0.),
                Vec3::new(0., 1., 0.),
            ),
            CameraPreset::Closer => c.set_camera(
                Vec3::new(8., 24., 36.),
                Vec3::new(-3., 2., 0.),
                Vec3::new(0., 1., 0.),
            ),
            CameraPreset::Window => c.set_camera(
                Vec3::new(-1., 10., -3.),
                Vec3::new(-20., 7., 0.),
                Vec3::new(0., 1., 0.),
            ),
            CameraPreset::Smartphone => c.set_camera(
                Vec3::new(6., 7.5, -1.),
                Vec3::new(10., 3., -10.),
                Vec3::new(0., 1., 0.),
            ),
            CameraPreset::Spotlight => c.set_camera(
                Vec3::new(-1., 9., 10.5),
                Vec3::new(13., 6., 5.),
                Vec3::new(0., 1., 0.),
            ),
            CameraPreset::Egg => c.set_camera(
                Vec3::new(-1.5, 10., 11.5),
                Vec3::new(0., 4., 0.),
                Vec3::new(0., 1., 0.),
            ),
            CameraPreset::Robot => c.set_camera(
                Vec3::new(robot_position.x + 5., 6., robot_position.z + 10.),
                Vec3::new(robot_position.x, 3., robot_position.z),
                Vec3::new(0., 1., 0.),
            ),
            CameraPreset::X => c.set_preset(crate::graphics::camera::CameraType::X),
            CameraPreset::Y => c.set_preset(crate::graphics::camera::CameraType::Y),
            CameraPreset::Z => c.set_preset(crate::graphics::camera::CameraType::Z),
        }
    }
    pub fn pose(&self, animator: &mut RobotAnimator, index: u8) -> bool {
        match index {
            1 => animator.pose1(),
            2 => animator.pose2(),
            3 => animator.pose3(),
            4 => animator.pose4(),
            5 => animator.pose5(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CameraKeymap, MuseumControlPanel};
    use crate::museum_gl::SkyType;

    #[test]
    fn default_selection_matches_swing_panel() {
        let panel = MuseumControlPanel::default();
        assert_eq!(panel.sky_type, SkyType::B);
        assert_eq!(panel.camera_keymap, CameraKeymap::Steve);
    }

    #[test]
    fn selection_changes_track_the_original_button_pairs() {
        let mut panel = MuseumControlPanel::default();
        assert!(!panel.set_sky_type(SkyType::B));
        assert!(panel.set_sky_type(SkyType::A));
        assert!(!panel.set_camera_keymap(CameraKeymap::Steve));
        assert!(panel.set_camera_keymap(CameraKeymap::My));
    }
}
