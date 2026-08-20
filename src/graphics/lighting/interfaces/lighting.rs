use crate::gmaths::Vec3;

pub trait Lighting {
    fn ambient(&self) -> Vec3;
    fn diffuse(&self) -> Vec3;
    fn specular(&self) -> Vec3;
}
