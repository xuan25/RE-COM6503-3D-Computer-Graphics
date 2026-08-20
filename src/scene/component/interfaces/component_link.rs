//! Shared scene-graph adapter for a retained component.
use super::Component;
use crate::{
    gmaths::{Mat4, Vec3},
    graphics::node::{CenterTransformable, Node},
};
use std::{cell::RefCell, rc::Rc};
pub struct ComponentLink<T: Component>(pub Rc<RefCell<T>>);
impl<T: Component> ComponentLink<T> {
    pub fn new(value: Rc<RefCell<T>>) -> Self {
        Self(value)
    }
}
impl<T: Component> CenterTransformable for ComponentLink<T> {
    fn center_translation(&self) -> Vec3 {
        self.0.borrow().node().center_translation()
    }
    fn set_center_translation(&mut self, x: f32, y: f32, z: f32) {
        self.0
            .borrow_mut()
            .node_mut()
            .set_center_translation(x, y, z)
    }
    fn center_rotation(&self) -> Vec3 {
        self.0.borrow().node().center_rotation()
    }
    fn set_center_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.0.borrow_mut().node_mut().set_center_rotation(x, y, z)
    }
    fn center_scale(&self) -> Vec3 {
        self.0.borrow().node().center_scale()
    }
    fn set_center_scale(&mut self, x: f32, y: f32, z: f32) {
        self.0.borrow_mut().node_mut().set_center_scale(x, y, z)
    }
    fn center_transform(&self) -> Mat4 {
        self.0.borrow().node().center_transform()
    }
}
impl<T: Component> Node for ComponentLink<T> {
    fn name(&self) -> String {
        self.0.borrow().node().name()
    }
    fn set_name(&mut self, name: String) {
        self.0.borrow_mut().node_mut().set_name(name)
    }
    fn update(&mut self) {
        self.0.borrow_mut().node_mut().update()
    }
    fn update_with_parent(&mut self, p: Mat4) {
        self.0.borrow_mut().node_mut().update_with_parent(p)
    }
    fn add_child(&mut self, c: Box<dyn Node>) {
        self.0.borrow_mut().node_mut().add_child(c)
    }
    fn remove_child_at(&mut self, i: usize) -> Option<Box<dyn Node>> {
        self.0.borrow_mut().node_mut().remove_child_at(i)
    }
    fn child_count(&self) -> usize {
        self.0.borrow().node().child_count()
    }
    unsafe fn render(&self) {
        unsafe { self.0.borrow().node().render() }
    }
    fn dispose(&mut self) {
        self.0.borrow_mut().node_mut().dispose()
    }
    fn build_string(&self, o: &mut String, l: &mut Vec<bool>) {
        self.0.borrow().node().build_string(o, l)
    }
    fn kind(&self) -> &'static str {
        self.0.borrow().node().kind()
    }
}
