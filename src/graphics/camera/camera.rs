//! Port of `legacy/graphics/camera/Camera.java`.

use crate::gmaths::{Mat4, Vec3, mat4_transform};

#[derive(Clone, Copy)]
pub enum CameraType {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy)]
pub enum Movement {
    NoMovement,
    Left,
    Right,
    Up,
    Down,
    Forward,
    Backward,
}

pub struct Camera {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    world_up: Vec3,
    front: Vec3,
    right: Vec3,
    yaw: f32,
    pitch: f32,
    perspective: Mat4,
}

impl Camera {
    pub const YAW: f32 = -90.0;
    pub const PITCH: f32 = 0.0;
    pub const MOVEMENT_SPEED: f32 = 12.0;
    pub const ROTATION_SENSITIVITY: f32 = 0.001;
    pub const PITCH_RANGE: f32 = 89.9999_f32 / 180.0 * std::f32::consts::PI;

    pub fn new(position: Vec3, target: Vec3, up: Vec3) -> Self {
        let mut result = Self {
            position,
            target,
            up,
            world_up: up,
            front: Vec3::default(),
            right: Vec3::default(),
            yaw: 0.0,
            pitch: 0.0,
            perspective: Mat4::identity(),
        };
        result.set_camera(position, target, up);
        result
    }

    pub fn set_camera(&mut self, position: Vec3, target: Vec3, up: Vec3) {
        self.position = position;
        self.target = target;
        self.up = up.normalized();
        self.world_up = self.up;
        self.front = (target - position).normalized();
        self.yaw = (self.front.z as f64).atan2(self.front.x as f64) as f32;
        self.pitch = (self.front.y as f64).asin() as f32;
        self.update_vectors();
    }

    pub fn set_preset(&mut self, camera_type: CameraType) {
        match camera_type {
            CameraType::X => self.set_camera(
                Vec3::new(0.0, 0.0, 25.0),
                Vec3::default(),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            CameraType::Y => self.set_camera(
                Vec3::new(0.0, 25.0, 0.0001),
                Vec3::default(),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            CameraType::Z => self.set_camera(
                Vec3::new(25.0, 0.0, 0.0),
                Vec3::default(),
                Vec3::new(0.0, 1.0, 0.0),
            ),
        }
    }

    pub const fn position(&self) -> Vec3 {
        self.position
    }
    pub const fn target(&self) -> Vec3 {
        self.target
    }
    pub const fn up(&self) -> Vec3 {
        self.up
    }
    pub fn view_matrix(&mut self) -> Mat4 {
        self.target = self.position + self.front;
        mat4_transform::look_at(self.position, self.target, self.up)
    }
    pub fn set_perspective_matrix(&mut self, matrix: Mat4) {
        self.perspective = matrix;
    }
    pub const fn perspective_matrix(&self) -> Mat4 {
        self.perspective
    }

    pub fn move_camera(&mut self, movement: Movement, delta_time: f32) {
        let distance = Self::MOVEMENT_SPEED * delta_time;
        match movement {
            Movement::NoMovement => {}
            Movement::Left => self.position += self.right * -distance,
            Movement::Right => self.position += self.right * distance,
            Movement::Up => self.position += self.up * distance,
            Movement::Down => self.position += self.up * -distance,
            Movement::Forward => self.position += self.front * distance,
            Movement::Backward => self.position += self.front * -distance,
        }
    }

    pub fn rotate_camera(&mut self, delta_x: f32, delta_y: f32) {
        self.update_yaw_pitch(
            delta_x * Self::ROTATION_SENSITIVITY,
            delta_y * Self::ROTATION_SENSITIVITY,
        );
    }

    /// Exact public counterpart of Java `Camera.updateYawPitch`.
    pub fn update_yaw_pitch(&mut self, yaw: f32, pitch: f32) {
        self.yaw += yaw;
        self.pitch = (self.pitch + pitch).clamp(-Self::PITCH_RANGE, Self::PITCH_RANGE);
        self.update_front();
        self.update_vectors();
    }

    fn update_front(&mut self) {
        // Java uses `Math.sin/cos` (double) and casts each component to
        // float before normalizing the vector.
        let yaw = self.yaw as f64;
        let pitch = self.pitch as f64;
        self.front = Vec3::new(
            (yaw.cos() * pitch.cos()) as f32,
            pitch.sin() as f32,
            (yaw.sin() * pitch.cos()) as f32,
        )
        .normalized();
        self.target = self.position + self.front;
    }

    fn update_vectors(&mut self) {
        self.right = self.front.cross(self.world_up).normalized();
        self.up = self.right.cross(self.front).normalized();
    }
}

#[cfg(test)]
mod tests {
    use super::Camera;
    use crate::gmaths::Vec3;

    #[test]
    fn rotating_camera_updates_target_to_rotated_unit_front() {
        let mut camera = Camera::new(
            Vec3::new(0., 0., 5.),
            Vec3::default(),
            Vec3::new(0., 2., 0.),
        );
        camera.update_yaw_pitch(0.5, 0.25);
        let offset = camera.target() - camera.position();
        assert!((offset.length() - 1.).abs() < 1e-5);
        assert!((camera.up().length() - 1.).abs() < 1e-5);
    }
}
