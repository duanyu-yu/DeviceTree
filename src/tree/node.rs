use alloc::{
	string::{String, ToString},
	vec::Vec,
	rc::Rc,
	collections::BTreeMap
};
use core::{
	cell::RefCell,
	convert::From
};

use crate::utils;

/* The #address-cells and #size-cells properties may be used in any device node that has children in the devicetree
hierarchy and describes how child device nodes should be addressed. 
The #address-cells property defines the number of <u32> cells used to encode the address field in a child node’s reg property. 
The #size-cells property defines the number of <u32> cells used to encode the size field in a child node’s reg property. */
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct AddressSizeCells {
	address_cells: u32,
	size_cells: u32
}

impl AddressSizeCells {
	pub fn new() -> Self {
		AddressSizeCells { address_cells: 2, size_cells: 1 } // Default value of #address-cells and #size-cells
	}

	pub fn set(&mut self, address_cells: u32, size_cells: u32) {
		self.address_cells = address_cells;
		self.size_cells = size_cells;
	}
}

/* Type aliases */
/// DeviceTreeNode wrapped in Rc<RefCell<DeviceTreeNode>> to have shared references
pub type DeviceTreeNodeWrap = Rc<RefCell<DeviceTreeNode>>;

/* Node of devicetree */
#[derive(Clone, Default, PartialEq, Debug)]
pub struct DeviceTreeNode {
	parent: Option<DeviceTreeNodeWrap>,
	children: BTreeMap<String, DeviceTreeNodeWrap>,
	/// Properties consist of a name and a value. Keys of properties are their names.
	properties: BTreeMap<String, DeviceTreeProperty>, 
	/// Required for all nodes that have children. Default: #address-cells=2 and #size-cells=1
	addresssizecells: AddressSizeCells, 
	label: Option<String>
}

impl DeviceTreeNode {
	pub fn new() -> Self {
		DeviceTreeNode {
			parent: None,
			children: BTreeMap::new(),
			properties: BTreeMap::new(),
			addresssizecells: AddressSizeCells::new(),
			label: None
		}
	}

	pub fn new_wrap() -> DeviceTreeNodeWrap {
		Rc::new(RefCell::new(
			DeviceTreeNode {
				parent: None,
				children: BTreeMap::new(),
				properties: BTreeMap::new(),
				addresssizecells: AddressSizeCells::new(),
				label: None
			}
		))
		
	}

	pub fn wrap(&self) -> DeviceTreeNodeWrap {
		Rc::new(RefCell::new(self.clone()))
	}

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn set_label(&mut self, label:&str) {
        self.label = Some(label.to_string());
    }

    pub fn parent(&self) -> Option<&DeviceTreeNodeWrap> {
        self.parent.as_ref()
    }

	pub fn set_parent(&mut self, parent: DeviceTreeNodeWrap) {
		self.parent = Some(parent);
	}

	/// Add child
	/// 
	/// Return None, if the child not exits; return the old child, if the child already exits
	pub fn update_child(&mut self, name: &str, node: DeviceTreeNodeWrap) -> Option<DeviceTreeNodeWrap> {
		node.borrow_mut().set_parent(Rc::clone(&self.wrap()));

		self.children.insert(name.to_string(), Rc::clone(&node))
	}

	/// Add child, and return this child
	pub fn add_child(&mut self, name: &str, node: DeviceTreeNodeWrap) -> DeviceTreeNodeWrap {
		node.borrow_mut().set_parent(Rc::clone(&self.wrap()));

		self.children.insert(name.to_string(), Rc::clone(&node));

		Rc::clone(&node)
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

	pub fn prop(&self, name: &str) -> Option<&DeviceTreeProperty>{
		self.properties.get(name)
	}

	pub fn prop_exists(&self, name: &str) -> bool {
		self.properties.contains_key(name)
	}

	/* Add a property into property-map:
	If the map did not have this key present, None is returned. 
	If the map did have this key present, the value is updated, and the old value is returned.*/
	pub fn add_prop(&mut self, name: &str, value: DeviceTreeProperty) -> Option<DeviceTreeProperty> {
		self.properties.insert(name.to_string(), value)
	}

	/* Removes a property from the property-map: 
	returning the stored name and value of the property if the property was previously in the map. */
	pub fn remove_prop(&mut self, name: &str) -> Option<(String, DeviceTreeProperty)> {
		self.properties.remove_entry(name)
	}

    pub fn set_ascells(&mut self, addr_cells: u32, size_cells: u32) {
        self.addresssizecells.set(addr_cells, size_cells);
    }


	/* Device-Tree specific actions */
	// create new /cpu node
	pub fn new_cpu(reg: u32) -> DeviceTreeNodeWrap {
		let cpu_node = DeviceTreeNode::new_wrap();

		// Required properties of a cpu node
		cpu_node.borrow_mut().add_prop("device_type", DeviceTreeProperty::String("cpu".to_string()));
        cpu_node.borrow_mut().add_prop("reg", DeviceTreeProperty::Pairs(Pairs(vec![(vec![reg], Vec::new())])));
		// TODO: further properties required: clock-frequency, timebase-frequency

		Rc::clone(&cpu_node)
	}
}

#[derive(Clone, PartialEq, Debug)]
pub enum StatusValue {
	Okay,
	Disabled,
	Reserved,
	Fail,
	FailSss,
}

impl StatusValue {
	pub fn to_string(&self) -> String {
		match self {
			StatusValue::Okay => String::from("okay"),
			StatusValue::Disabled => String::from("disabled"),
			StatusValue::Reserved => String::from("reserved"),
			StatusValue::Fail => String::from("fail"),
			StatusValue::FailSss => String::from("fail-sss")
		}
	}
}

/* Property of devicetree: 
Each node in the devicetree has properties that describe the characteristics of the node. */
#[derive(Clone, PartialEq, Debug)]
pub enum DeviceTreeProperty {
	Empty,
	StringList(Vec<String>), // Specifies a list of platform architectures with which this platform is compatible. This property can be used by operating systems in selecting platform specific code. The recommended form of the property value is: "manufacturer,model"
	String(String), // Specifies a string that uniquely identifies the model of the system board. The recommended format is “manufacturer,model-number”.
	U32(u32),
	U64(u64),
	Status(StatusValue),
	Pairs(Pairs),
	Triplets(Option<Triplets>),
	Bytes(Vec<u8>)
}

impl DeviceTreeProperty {
	pub fn to_stringfmt(&self) -> String { 
		match self {
			DeviceTreeProperty::Empty => String::new(),
			DeviceTreeProperty::StringList(v) => utils::vec_strings_fmt(v),  
			DeviceTreeProperty::String(v) => v.to_string(),
			DeviceTreeProperty::U32(v) => format!("0x{:X}", v),
			DeviceTreeProperty::U64(v) => format!("0x{:X}", v),
			DeviceTreeProperty::Status(v) => v.to_string(),
			DeviceTreeProperty::Pairs(v) => v.clone().into(),
			DeviceTreeProperty::Triplets(v) => v.as_ref().unwrap().clone().into(),
			DeviceTreeProperty::Bytes(v) => String::from_utf8(v.to_vec()).unwrap()
		}
	}
}

// Vector of pairs: one of formats of prop-encoded-array
#[derive(Clone, PartialEq, Debug)]
pub struct Pairs(pub(crate) Vec<(Vec<u32>, Vec<u32>)>);

impl Pairs {
	pub fn new() -> Self {
		Pairs(Vec::new())
	}
}

impl From<Pairs> for String {
	fn from(pairs: Pairs) -> Self {
		let mut v = Vec::new();

		for (a, b) in pairs.0 {
			let v_inner = [a, b].concat();

			let v_str: Vec<String> = v_inner.iter().map(|&i| format!("0x{:X}", i)).collect();

			let str = v_str.join(" ");

			v.push(str);
		}

		v.join(" ")
	}
}

// Vector of triplets: one of formats of prop-encoded-array
#[derive(Clone, PartialEq, Debug)]
pub struct Triplets(pub(crate) Vec<(Vec<u32>, Vec<u32>, Vec<u32>)>);

impl From<Triplets> for String {
	fn from(triplets: Triplets) -> Self {
		let mut v = Vec::new();

		for (a, b, c) in triplets.0 {
			let v_inner = [a, b, c].concat();

			let v_str: Vec<String> = v_inner.iter().map(|&i| format!("0x{:X}", i)).collect();

			let str = v_str.join(" ");

			v.push(str);
		}

		v.join(" ")
	}
}
