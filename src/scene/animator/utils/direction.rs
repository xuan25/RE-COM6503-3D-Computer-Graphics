//! Port of `legacy/scene/animator/utils/Direction.java`.

pub fn simplify_rotation(reference: f32, mut target: f32) -> f32 {
    while target - reference > 180.0 {
        target -= 360.0;
    }
    while target - reference < -180.0 {
        target += 360.0;
    }
    target
}

pub fn travel_direction(x1: f32, z1: f32, x2: f32, z2: f32) -> f32 {
    (x2 - x1).atan2(z2 - z1).to_degrees()
}
