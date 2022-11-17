use alloc::rc::Rc;

use crate::{
	DeviceTree,
	tree::node::{
		DeviceTreeNode,
		AddChild,
	}
};

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

#[test]
fn from_bytes() {
	let mut dtb: &[u8] = include_bytes!("./dtb/test1.dtb");

    let tree = DeviceTree::from_bytes(&mut dtb).unwrap();

    assert_eq!(tree.num_cpus(), 4);
}
