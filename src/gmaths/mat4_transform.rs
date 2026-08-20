use super::{Mat4, Vec3};
pub fn translate(v: Vec3) -> Mat4 {
    Mat4::translation(v)
}
pub fn scale(v: Vec3) -> Mat4 {
    Mat4 {
        values: [
            v.x, 0., 0., 0., 0., v.y, 0., 0., 0., 0., v.z, 0., 0., 0., 0., 1.,
        ],
    }
}
pub fn rotate_x(degrees: f32) -> Mat4 {
    let radians = degrees as f64 * std::f64::consts::PI / 180.0;
    let s = radians.sin() as f32;
    let c = radians.cos() as f32;
    Mat4 {
        values: [1., 0., 0., 0., 0., c, s, 0., 0., -s, c, 0., 0., 0., 0., 1.],
    }
}
pub fn rotate_y(degrees: f32) -> Mat4 {
    let radians = degrees as f64 * std::f64::consts::PI / 180.0;
    let s = radians.sin() as f32;
    let c = radians.cos() as f32;
    Mat4 {
        values: [c, 0., -s, 0., 0., 1., 0., 0., s, 0., c, 0., 0., 0., 0., 1.],
    }
}
pub fn rotate_z(degrees: f32) -> Mat4 {
    let radians = degrees as f64 * std::f64::consts::PI / 180.0;
    let s = radians.sin() as f32;
    let c = radians.cos() as f32;
    Mat4 {
        values: [c, s, 0., 0., -s, c, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.],
    }
}
pub fn perspective(fov: f32, aspect: f32) -> Mat4 {
    // `Mat4Transform.java` uses its default near/far planes of 0.1 and
    // 1000.0.  Keep the calculation explicit so the depth mapping remains
    // identical to the JOGL projection matrix.
    const NEAR_CLIP: f32 = 0.1;
    const FAR_CLIP: f32 = 1000.0;
    let f = (1.0 / ((fov as f64 * std::f64::consts::PI / 180.0) * 0.5).tan()) as f32;
    let depth_scale = -(FAR_CLIP + NEAR_CLIP) / (FAR_CLIP - NEAR_CLIP);
    let depth_offset = -(2. * FAR_CLIP * NEAR_CLIP) / (FAR_CLIP - NEAR_CLIP);
    Mat4 {
        values: [
            f / aspect,
            0.,
            0.,
            0.,
            0.,
            f,
            0.,
            0.,
            0.,
            0.,
            depth_scale,
            -1.,
            0.,
            0.,
            depth_offset,
            0.,
        ],
    }
}
pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
    let f = (target - eye).normalized();
    let s = f.cross(up).normalized();
    // Java `Mat4Transform.lookAt` explicitly normalizes this second cross
    // product before constructing the view matrix.
    let u = s.cross(f).normalized();
    Mat4 {
        values: [
            s.x,
            u.x,
            -f.x,
            0.,
            s.y,
            u.y,
            -f.y,
            0.,
            s.z,
            u.z,
            -f.z,
            0.,
            -s.dot(eye),
            -u.dot(eye),
            f.dot(eye),
            1.,
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::perspective;

    #[test]
    fn perspective_uses_the_legacy_default_far_clip_of_1000() {
        let matrix = perspective(45., 1.);
        assert!((matrix.values[10] + 1.0002).abs() < 0.00001);
        assert!((matrix.values[14] + 0.20002).abs() < 0.00001);
    }
}
