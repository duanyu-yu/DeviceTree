use alloc::rc::Rc;

use crate::DeviceTree;
use crate::tree::node::{
	DeviceTreeNodeWrap, 
	DeviceTreeNode
};

impl DeviceTree {
	pub fn new_empty_root() -> Self {
		DeviceTree {
			root: DeviceTreeNode::new_wrap()
		}
	}

	pub fn new(root: DeviceTreeNodeWrap) -> Self {
		root.borrow_mut().set_name("/");

		Self { root: Rc::clone(&root) }
	}

	pub fn root(&self) -> &DeviceTreeNodeWrap {
		&self.root
	}

	pub fn num_cpus(&self) -> usize {
		let root = &self.root;

		if let Some(cpus) = root.borrow().find_child("cpus") {
			return cpus.borrow().num_children();
		}

		return 0;
	}

	pub fn has_cpus(&self) -> bool {
		let root = self.root();

		if root.borrow().find_child("cpus").is_some() {
			return true;
		} else {
			return false;
		}
	}
}

impl core::fmt::Display for DeviceTree {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		writeln!(f, "Device-Tree: ")?;

		writeln!(f, "{}", self.root().borrow())
	}
}
