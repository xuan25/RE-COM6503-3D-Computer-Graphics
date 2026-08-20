//! Shared child-node adapter used where a component must retain a child node for animation.

#![allow(unsafe_op_in_unsafe_fn)]

use super::interfaces::{CenterTransformable, Node};
use crate::gmaths::{Mat4, Vec3};
use std::{cell::RefCell, rc::Rc};

pub struct NodeLink<T: Node>(pub Rc<RefCell<T>>);

impl<T: Node> NodeLink<T> {
    pub fn new(node: Rc<RefCell<T>>) -> Self {
        Self(node)
    }
}

impl<T: Node> CenterTransformable for NodeLink<T> {
    fn center_translation(&self) -> Vec3 {
        self.0.borrow().center_translation()
    }
    fn set_center_translation(&mut self, x: f32, y: f32, z: f32) {
        self.0.borrow_mut().set_center_translation(x, y, z);
    }
    fn center_rotation(&self) -> Vec3 {
        self.0.borrow().center_rotation()
    }
    fn set_center_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.0.borrow_mut().set_center_rotation(x, y, z);
    }
    fn center_scale(&self) -> Vec3 {
        self.0.borrow().center_scale()
    }
    fn set_center_scale(&mut self, x: f32, y: f32, z: f32) {
        self.0.borrow_mut().set_center_scale(x, y, z);
    }
    fn center_transform(&self) -> Mat4 {
        self.0.borrow().center_transform()
    }
}

impl<T: Node> Node for NodeLink<T> {
    fn name(&self) -> String {
        self.0.borrow().name()
    }
    fn set_name(&mut self, name: String) {
        self.0.borrow_mut().set_name(name);
    }
    fn update(&mut self) {
        self.0.borrow_mut().update();
    }
    fn update_with_parent(&mut self, parent_transform: Mat4) {
        self.0.borrow_mut().update_with_parent(parent_transform);
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.0.borrow_mut().add_child(child);
    }
    fn remove_child_at(&mut self, index: usize) -> Option<Box<dyn Node>> {
        self.0.borrow_mut().remove_child_at(index)
    }
    fn child_count(&self) -> usize {
        self.0.borrow().child_count()
    }
    unsafe fn render(&self) {
        self.0.borrow().render();
    }
    fn dispose(&mut self) {
        self.0.borrow_mut().dispose();
    }
    fn build_string(&self, output: &mut String, last_children: &mut Vec<bool>) {
        self.0.borrow().build_string(output, last_children);
    }
    fn kind(&self) -> &'static str {
        self.0.borrow().kind()
    }
}
