use alloc::{
	string::{String, ToString},
	vec::Vec,
	rc::Rc,
	collections::{
		BTreeMap,
		btree_map::Iter
	}
};
use core::cell::RefCell;
use log::{
    info,
    debug,
    error
};

use crate::{DeviceTreeError, utils};

use super::prop::{
	DeviceTreeProperty,
	NumCells,
	Pairs
};

const INDENT_SIZE: usize = 4;

static mut INDENT: usize = 0;

/// Node of devicetree 
#[derive(Clone, Default, PartialEq, Debug)]
pub struct DeviceTreeNode {
	name: String,
	parent: Option<DeviceTreeNodeWrap>,
	children: BTreeMap<String, DeviceTreeNodeWrap>,
	/// Properties consist of a name and a value. Keys of properties are their names.
	properties: BTreeMap<String, DeviceTreeProperty>, 
	/// Required for all nodes that have children. Default: #address-cells=2 and #size-cells=1
	num_cells: NumCells, 
	label: Option<String>
}

impl DeviceTreeNode {
	pub fn new() -> Self {
		Self {
			name: String::new(),
			parent: None,
			children: BTreeMap::new(),
			properties: BTreeMap::new(),
			num_cells: NumCells::new(),
			label: None
		}
	}

	pub fn new_wrap() -> DeviceTreeNodeWrap {
		Rc::new(RefCell::new(Self::new()))
		
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn set_name(&mut self, name: &str) {
		self.name = name.to_string();
	}

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = Some(label.to_string());
    }

    pub fn parent(&self) -> Option<&DeviceTreeNodeWrap> {
        self.parent.as_ref()
    }

	pub fn set_parent(&mut self, parent: DeviceTreeNodeWrap) {
		self.parent = Some(Rc::clone(&parent));
	}

	pub fn has_parent(&self) -> bool {
		self.parent.is_some()
	}

	pub fn children_iter(&self) -> Iter<'_, String, DeviceTreeNodeWrap> {
		self.children.iter()
	}

	pub fn find_child(&self, name: &str) -> Option<&DeviceTreeNodeWrap> { 
		self.children.get(name)
	}

	pub fn child_exists(&self, name: &str) -> bool {
		self.children.contains_key(name)
	}

	pub fn num_children(&self) -> usize {
		self.children.len()
	}

	pub fn prop_value(&self, name: &str) -> Option<&DeviceTreeProperty>{
		self.properties.get(name)
	}

	pub fn prop_iter(&self) -> Iter<'_, String, DeviceTreeProperty> {
		self.properties.iter()
	}

	pub fn prop_exists(&self, name: &str) -> bool {
		self.properties.contains_key(name)
	}

	/// Add a property into property-map of the current node:
	/// 
	/// If the map did not have this key present, None is returned. 
	/// 
	/// If the map did have this key present, the value is updated, and the old value is returned.
	pub fn update_prop(&mut self, name: &str, value: DeviceTreeProperty) -> Option<DeviceTreeProperty> {
		debug!("Adding property {{ {} {} }} to node '{}'.", name, value, self.name());

		self.properties.insert(name.to_string(), value)
	}

	/// Removes a property from the property-map: 
	/// 
	/// returning the stored name and value of the property if the property was previously in the map.
	pub fn remove_prop(&mut self, name: &str) -> Option<(String, DeviceTreeProperty)> {
		self.properties.remove_entry(name)
	}

    pub fn set_numcells(&mut self, addr_cells: u32, size_cells: u32) {
        self.num_cells.set(addr_cells, size_cells);
    }

	pub fn set_addr_cells(&mut self, addr_cells: u32) {
		self.num_cells.set_addr_cells(addr_cells);
	}

	pub fn set_size_cells(&mut self, size_cells: u32) {
		self.num_cells.set_size_cells(size_cells);
	}

	/* Device-Tree specific actions */
	/// create a new /cpu node
	pub fn new_cpu(reg: u32) -> DeviceTreeNodeWrap {
		let cpu_node = DeviceTreeNode::new_wrap();

		// Required properties of a cpu node
		cpu_node.borrow_mut().update_prop("device_type", DeviceTreeProperty::String("cpu".to_string()));
        cpu_node.borrow_mut().update_prop("reg", DeviceTreeProperty::Pairs(Pairs(vec![(vec![reg], Vec::new())])));
		// TODO: further properties required: clock-frequency, timebase-frequency

		Rc::clone(&cpu_node)
	}
}

impl core::fmt::Display for DeviceTreeNode {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		unsafe {
			writeln!(f, "")?;
			
			writeln!(f, "{:indent$}{} {{", "", self.name(), indent = INDENT)?;

			INDENT += INDENT_SIZE;

			for (prop_name, prop_value) in self.prop_iter() {
				writeln!(f, "{:indent$}{} {};", "", prop_name, prop_value, indent = INDENT)?;
			} 

			for (_, child) in self.children_iter() {
				writeln!(f, "{}", child.borrow())?;
			}

			INDENT -= INDENT_SIZE;

			write!(f, "{:indent$}}};", "", indent = INDENT)
		}
	}
}

/// DeviceTreeNode wrapped in Rc<RefCell<DeviceTreeNode>> to have shared references
pub type DeviceTreeNodeWrap = Rc<RefCell<DeviceTreeNode>>;

pub trait AddChild {
	fn add_child(&self, name: &str, child: DeviceTreeNodeWrap) -> Option<DeviceTreeNodeWrap>;
}

impl AddChild for DeviceTreeNodeWrap {
	/// Add child to the current node
	/// 
	/// If the current node did not have the child present, None is returned.
	/// 
	/// If the current node did have the child present, the child is updated, and the old child is returned.
	fn add_child(&self, name: &str, child: DeviceTreeNodeWrap) -> Option<DeviceTreeNodeWrap> {
		debug!("Adding subnode '{}' to node '{}'.", name, self.borrow().name());

		child.borrow_mut().set_name(name);
		child.borrow_mut().set_parent(Rc::clone(&self));

		self.borrow_mut().children.insert(name.to_string(), Rc::clone(&child))
	}
}
