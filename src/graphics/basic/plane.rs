/// Port of `legacy/graphics/basic/Plane.java`.
pub struct Plane;
impl Plane {
    pub fn vertices() -> Vec<f32> {
        vec![
            -0.5, 0., -0.5, 0., 1., 0., 0., 1., -0.5, 0., 0.5, 0., 1., 0., 0., 0., 0.5, 0., 0.5,
            0., 1., 0., 1., 0., 0.5, 0., -0.5, 0., 1., 0., 1., 1.,
        ]
    }
    pub fn indices() -> Vec<u32> {
        vec![0, 1, 2, 0, 2, 3]
    }
}
