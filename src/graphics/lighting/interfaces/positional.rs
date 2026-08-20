use crate::gmaths::Vec3;

pub trait Positional {
    fn position(&self) -> Vec3;
}
