/// Port of `legacy/graphics/basic/Sphere.java`.
pub struct Sphere;
impl Sphere {
    pub const XLONG: usize = 30;
    pub const YLAT: usize = 30;
    pub fn vertices() -> Vec<f32> {
        let mut v = Vec::with_capacity(Self::XLONG * Self::YLAT * 8);
        for j in 0..Self::YLAT {
            // `Sphere.java` performs all spherical-coordinate calculations
            // with `double` and narrows only the values written to its float
            // vertex array.
            let b = (-90.0 + 180.0 * j as f64 / (Self::YLAT - 1) as f64).to_radians();
            for i in 0..Self::XLONG {
                let a = (360.0 * i as f64 / (Self::XLONG - 1) as f64).to_radians();
                let (x, y, z) = (b.cos() * a.sin(), b.sin(), b.cos() * a.cos());
                v.extend_from_slice(&[
                    (0.5 * x) as f32,
                    (0.5 * y) as f32,
                    (0.5 * z) as f32,
                    x as f32,
                    y as f32,
                    z as f32,
                    i as f32 / (Self::XLONG - 1) as f32,
                    j as f32 / (Self::YLAT - 1) as f32,
                ])
            }
        }
        v
    }
    pub fn indices() -> Vec<u32> {
        let mut v = Vec::new();
        for j in 0..Self::YLAT - 1 {
            for i in 0..Self::XLONG - 1 {
                let n = (j * Self::XLONG + i) as u32;
                v.extend_from_slice(&[
                    n,
                    n + 1,
                    n + Self::XLONG as u32 + 1,
                    n,
                    n + Self::XLONG as u32 + 1,
                    n + Self::XLONG as u32,
                ])
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::Sphere;

    #[test]
    fn indices_match_the_legacy_two_triangle_winding() {
        let indices = Sphere::indices();
        let xlong = Sphere::XLONG as u32;
        assert_eq!(&indices[..6], &[0, 1, xlong + 1, 0, xlong + 1, xlong]);
    }
}
