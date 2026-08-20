//! Port of `legacy/scene/component/interfaces/Component.java`.

use crate::graphics::node::Node;

pub trait Component {
    fn node(&self) -> &dyn Node;
    fn node_mut(&mut self) -> &mut dyn Node;
}
