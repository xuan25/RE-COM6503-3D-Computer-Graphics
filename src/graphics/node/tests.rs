use super::{BasicNode, CenterTransformable, Node};
#[test]
fn hierarchy_contains_children_in_order() {
    let mut root = BasicNode::new("Root");
    root.add_child(Box::new(BasicNode::new("First")));
    root.add_child(Box::new(BasicNode::new("Second")));
    let text = root.hierarchy_string();
    assert!(text.contains("[Root - BasicNode]"));
    assert!(text.find("First").unwrap() < text.find("Second").unwrap());
}
#[test]
fn update_applies_center_translation() {
    let mut node = BasicNode::new("Node");
    node.set_center_translation(2.0, 3.0, 4.0);
    node.update();
    assert_eq!(node.center_transform().values[12], 2.0);
    assert_eq!(node.center_transform().values[13], 3.0);
    assert_eq!(node.center_transform().values[14], 4.0);
}
