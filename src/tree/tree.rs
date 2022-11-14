use alloc::{
	string::ToString,
	rc::Rc
};

use crate::{
	DeviceTree, 
	DeviceTreeError
};
use crate::tree::node::{
	DeviceTreeNodeWrap, 
	DeviceTreeNode,
	DeviceTreeProperty
};

use super::CPU_MAX_NUM;

impl DeviceTree {
	pub fn new() -> Self {
		DeviceTree {
			root: DeviceTreeNode::new_wrap()
		}
	}

	pub fn init() -> Result<Self, DeviceTreeError> {
        let mut tree = DeviceTree::new();

        tree.add_cpus(1)?;
        tree.add_memory();

		Ok(tree)
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

	// create new node /cpus
	pub fn add_cpus(&mut self, num: u32) -> Result<DeviceTreeNodeWrap, DeviceTreeError>{
		let root = &mut self.root;
		let cpus_node = DeviceTreeNode::new_wrap();

        // In the cpus node, #address-cells is set to 1, and #size-cells is set to 0. 
        // This means that child reg values are a single uint32 that represent the address with no size field.
        cpus_node.borrow_mut().set_ascells(1, 0);

        let mut result = Err(DeviceTreeError::CpuNumInvalid);

        match num {
            1..=CPU_MAX_NUM => for reg in 0..num 
            { 
                cpus_node.borrow_mut().update_child(&format!("cpu@{}", reg), DeviceTreeNode::new_cpu(reg));
                result = Ok(Rc::clone(&cpus_node));
            },
            _ => ()
        }

		root.borrow_mut().update_child("cpus", Rc::clone(&cpus_node));

        return result;
	}

    // create new node /memory
    pub fn add_memory(&mut self) { // TODO: add arguments to pass address and size information
        let root = &mut self.root;
        let memory_node = DeviceTreeNode::new_wrap();

		// Required properties of a memory node
		memory_node.borrow_mut().add_prop("device_type", DeviceTreeProperty::String("memory".to_string()));
		// TODO: further properties required: reg

		root.borrow_mut().update_child("memory", Rc::clone(&memory_node));
    }

	pub fn add_net(&mut self, mac: [u8; 6]) {
		let root = &mut self.root;
		let net_node = DeviceTreeNode::new_wrap();

		// Set local-mac-address property 
		net_node.borrow_mut().add_prop("local-mac-address", DeviceTreeProperty::Bytes(mac.to_vec()));

		root.borrow_mut().update_child("net", Rc::clone(&net_node));
	}
}
