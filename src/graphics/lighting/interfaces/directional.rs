use crate::gmaths::Vec3;

pub trait Directional {
    fn direction(&self) -> Vec3;
}
