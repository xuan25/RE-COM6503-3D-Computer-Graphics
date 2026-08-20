use super::CenterTransformable;
use crate::gmaths::Mat4;

pub trait Node: CenterTransformable {
    fn name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn update(&mut self);
    fn update_with_parent(&mut self, parent_transform: Mat4);
    fn add_child(&mut self, child: Box<dyn Node>);
    fn remove_child_at(&mut self, index: usize) -> Option<Box<dyn Node>>;
    fn child_count(&self) -> usize;
    unsafe fn render(&self);
    fn dispose(&mut self);
    fn build_string(&self, output: &mut String, last_children: &mut Vec<bool>);
    fn kind(&self) -> &'static str;
}
