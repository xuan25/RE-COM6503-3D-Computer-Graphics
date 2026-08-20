pub trait Ranged {
    fn cut_off(&self) -> f32;
    fn cut_off_coefficient(&self) -> f32;
    fn set_cut_off(&mut self, degree: f32);
    fn outer_cut_off(&self) -> f32;
    fn outer_cut_off_coefficient(&self) -> f32;
    fn set_outer_cut_off(&mut self, degree: f32);
}
