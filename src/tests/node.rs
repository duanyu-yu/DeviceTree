use alloc::{
	string::ToString,
	rc::Rc
};

use crate::{
	tree::{
		node::{
			DeviceTreeNode,
			AddChild,
		},
		prop::DeviceTreeProperty
	},
};

#[test]
fn set_parent() {
	let parent = DeviceTreeNode::new_wrap();
	let child = DeviceTreeNode::new_wrap();

	child.borrow_mut().set_parent(Rc::clone(&parent));

	assert_eq!(child.borrow().parent(), Some(&Rc::clone(&parent)));
}

#[test]
fn add_child() {
	let parent = DeviceTreeNode::new_wrap();

	parent.borrow_mut().set_label("parent");

	let child = DeviceTreeNode::new_wrap();

	child.borrow_mut().set_label("child");

	parent.add_child("child", Rc::clone(&child));

	let parent_of_child = Rc::clone(&child.borrow().parent().as_ref().unwrap());

	assert_eq!(parent_of_child.borrow().label(), Some(&"parent".to_string()));
	assert_eq!(parent.borrow().find_child("child").unwrap().borrow().label(), Some(&"child".to_string()));
}

#[test]
fn add_prop() {
	let mut node = DeviceTreeNode::new();

	assert_eq!(node.add_prop(DeviceTreeProperty::from_bytes("name", "value".as_bytes())), None);
	assert_eq!(node.prop_exists("name"), true);
}

#[test]
fn delete_prop() {
	let mut node = DeviceTreeNode::new();

	node.add_prop(DeviceTreeProperty::from_bytes("name", "value".as_bytes()));

	assert_eq!(node.prop_exists("name"), true);

	node.remove_prop("name");

	assert_eq!(node.prop_exists("name"), false);
}
