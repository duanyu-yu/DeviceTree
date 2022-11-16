use alloc::rc::Rc;

use crate::{
	DeviceTree,
	tree::node::{
		DeviceTreeNode,
		AddChild,
	}
};

#[test]
fn cpus() {
	let mut tree = DeviceTree::init().expect("Failed by init device-tree!");

	assert!(tree.add_cpus(4).is_ok());

	let root = tree.root();

	let current = Rc::clone(root);

	let tmp = current.borrow();

	let cpus = tmp.find_child("cpus").unwrap();

	assert!(cpus.borrow().child_exists("cpu@0"));
	assert!(cpus.borrow().child_exists("cpu@1"));
	assert!(cpus.borrow().child_exists("cpu@2"));
	assert!(cpus.borrow().child_exists("cpu@3"));
}

#[test]
fn tree() {
	let tree = DeviceTree::new_empty_root();

	let root = tree.root();

	let mut current = Rc::clone(root);

	let cpus = DeviceTreeNode::new_wrap();

	current.add_child("cpus", Rc::clone(&cpus));

	current = Rc::clone(&cpus);

	assert!(tree.has_cpus());

	let cpu_0 = DeviceTreeNode::new_wrap();

	current.add_child("cpu@0", Rc::clone(&cpu_0));

	assert_eq!(tree.num_cpus(), 1);
}
