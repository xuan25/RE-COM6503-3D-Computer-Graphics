//! GLFW equivalent of `legacy/Museum.java` and `MuseumGL.java`.

use crate::{
    gmaths::Vec3,
    graphics::camera::Camera,
    museum_control_panel::{CONTROL_PANEL_WIDTH, CameraKeymap, MuseumControlPanel},
    museum_gl::{CameraMovementState, MuseumGL},
};
use glfw::{Action, Context, Key, WindowEvent};
use imgui_opengl_renderer::Renderer as ImguiRenderer;
use std::{cell::RefCell, rc::Rc, time::Instant};

pub const WIDTH: u32 = 1024;
pub const HEIGHT: u32 = 768;

fn canvas_width(window_width: i32) -> i32 {
    (window_width - CONTROL_PANEL_WIDTH).max(1)
}

fn update_camera_movement(
    keymap: CameraKeymap,
    state: &mut CameraMovementState,
    key: Key,
    down: bool,
) {
    match keymap {
        CameraKeymap::My => match key {
            Key::A => state.left = down,
            Key::D => state.right = down,
            Key::E => state.up = down,
            Key::Q => state.down = down,
            Key::W => state.forward = down,
            Key::S => state.backward = down,
            _ => {}
        },
        CameraKeymap::Steve => match key {
            Key::Left => state.left = down,
            Key::Right => state.right = down,
            Key::Up => state.up = down,
            Key::Down => state.down = down,
            Key::A => state.forward = down,
            Key::Z => state.backward = down,
            _ => {}
        },
    }
}

/// Application-window counterpart of `legacy/Museum.java`.
pub struct Museum {
    title: String,
}

impl Museum {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }

    pub fn run(&self) -> Result<(), String> {
        let mut glfw = glfw::init(glfw::fail_on_errors).map_err(|e| e.to_string())?;
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        let (mut window, events) = glfw
            .create_window(
                WIDTH,
                HEIGHT,
                self.title.as_str(),
                glfw::WindowMode::Windowed,
            )
            .ok_or("unable to create GLFW window")?;
        window.make_current();
        window.set_key_polling(true);
        // `WindowEvent::Size` is delivered only when this callback is
        // enabled. It drives the existing framebuffer and projection resize
        // path below.
        window.set_size_polling(true);
        // GLFW only sends mouse-button events through its event channel when
        // this callback is enabled.  Without it Dear ImGui receives cursor
        // movement but never a click, so every sidebar control is inert.
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        gl::load_with(|name| {
            window
                .get_proc_address(name)
                .map_or(std::ptr::null(), |p| p as *const _)
        });
        unsafe {
            gl::ClearColor(0.005, 0.005, 0.005, 1.);
            gl::ClearDepth(1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
            gl::FrontFace(gl::CCW);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
        }
        let camera = Rc::new(RefCell::new(Camera::new(
            Vec3::new(40., 32., 50.),
            Vec3::new(-3., 2., 0.),
            Vec3::new(0., 1., 0.),
        )));
        let mut gl_event_listener = unsafe {
            MuseumGL::initialize(camera, ".", canvas_width(WIDTH as i32), HEIGHT as i32)?
        };
        let mut controls = MuseumControlPanel::default();
        // Mirror the initial Java slider positions (all lighting at 100%, HDR at 50%).
        controls.set_environment_intensity(&mut gl_event_listener.environment_animator, 1.);
        controls.set_room_intensity(&mut gl_event_listener.room.borrow_mut(), 1.);
        controls.set_spotlight_intensity(&mut gl_event_listener.spotlight.borrow_mut(), 1.);
        controls.set_exposure(&mut gl_event_listener.hdr, 0.5);
        controls.set_room_light_rotation(&mut gl_event_listener.room.borrow_mut(), 0.);
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        imgui.set_platform_name(Some("glfw".to_owned()));
        imgui.io_mut().display_size = [WIDTH as f32, HEIGHT as f32];
        let imgui_renderer = ImguiRenderer::new(&mut imgui, |name| {
            window
                .get_proc_address(name)
                .map_or(std::ptr::null(), |p| p as *const _)
        });
        let mut last = Instant::now();
        let mut dragging = false;
        let mut last_cursor = (0., 0.);
        while !window.should_close() {
            let now = Instant::now();
            let delta = now.duration_since(last).as_secs_f32();
            last = now;
            {
                let io = imgui.io_mut();
                io.delta_time = delta.max(f32::EPSILON);
                io.mouse_wheel = 0.;
            }
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    WindowEvent::Close => window.set_should_close(true),
                    WindowEvent::Key(key, _, action, _) => {
                        let down = action != Action::Release;
                        update_camera_movement(
                            controls.camera_keymap,
                            &mut gl_event_listener.camera_movement_state,
                            key,
                            down,
                        );
                    }
                    WindowEvent::MouseButton(glfw::MouseButton::Button1, action, _) => {
                        imgui.io_mut().mouse_down[0] = action != Action::Release;
                        if action == Action::Release {
                            // A release must always end a camera drag, including when it occurs
                            // above the control panel after the pointer crossed into it.
                            dragging = false;
                        } else if last_cursor.0 >= CONTROL_PANEL_WIDTH as f64
                            && !imgui.io().want_capture_mouse
                        {
                            dragging = true;
                        }
                    }
                    WindowEvent::CursorPos(x, y) => {
                        imgui.io_mut().mouse_pos = [x as f32, y as f32];
                        if dragging && !imgui.io().want_capture_mouse {
                            gl_event_listener.camera.borrow_mut().rotate_camera(
                                (x - last_cursor.0) as f32,
                                -(y - last_cursor.1) as f32,
                            )
                        }
                        last_cursor = (x, y)
                    }
                    WindowEvent::Scroll(_, y) => imgui.io_mut().mouse_wheel += y as f32,
                    WindowEvent::Size(w, h) if h > 0 => {
                        imgui.io_mut().display_size = [w as f32, h as f32];
                        unsafe {
                            gl_event_listener.reshape(canvas_width(w), h)?;
                        }
                    }
                    _ => {}
                }
            }
            // Swing dispatches a control action before the next GL animator
            // display.  Build and process the ImGui panel at the equivalent
            // point so a click changes this frame rather than the next one.
            let mut close_from_controls = false;
            {
                let ui = imgui.frame();
                controls.draw(ui, &mut gl_event_listener, &mut close_from_controls);
            }
            if close_from_controls {
                window.set_should_close(true);
                break;
            }
            gl_event_listener.advance(delta);
            unsafe {
                gl_event_listener.render(CONTROL_PANEL_WIDTH);
            }
            imgui_renderer.render(&mut imgui);
            window.swap_buffers();
        }
        gl_event_listener.dispose();
        Ok(())
    }
}

/// Kept as the crate entry-point API used by `main.rs`.
pub fn run() -> Result<(), String> {
    Museum::new("Museum").run()
}

#[cfg(test)]
mod tests {
    use super::{CONTROL_PANEL_WIDTH, WIDTH, canvas_width};

    #[test]
    fn gl_canvas_width_excludes_the_west_control_panel() {
        assert_eq!(
            canvas_width(WIDTH as i32),
            WIDTH as i32 - CONTROL_PANEL_WIDTH
        );
        assert_eq!(canvas_width(CONTROL_PANEL_WIDTH), 1);
    }
}
