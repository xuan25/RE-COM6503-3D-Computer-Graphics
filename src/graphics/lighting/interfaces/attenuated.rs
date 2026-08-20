pub trait Attenuated {
    fn attenuation_constant(&self) -> f32;
    fn set_attenuation_constant(&mut self, value: f32);
    fn attenuation_linear(&self) -> f32;
    fn set_attenuation_linear(&mut self, value: f32);
    fn attenuation_quadratic(&self) -> f32;
    fn set_attenuation_quadratic(&mut self, value: f32);
}
