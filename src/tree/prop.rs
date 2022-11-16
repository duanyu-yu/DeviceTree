use alloc::{
	string::{String, ToString},
	vec::Vec
};
use core::convert::From;

use crate::utils;

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

impl core::fmt::Display for DeviceTreeProperty {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Empty => write!(f, ";"),
			Self::StringList(v) => write!(f, "= {}", v.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<String>>().join(", ")),
			Self::String(s) => write!(f, "= \"{}\"", s),
			Self::U32(i) => write!(f, "= <0x{:x}>", i),
			Self::U64(i) => write!(f, "= <0x{:x}>", i),
			Self::Status(s) => write!(f, "= \"{}\"", s),
			Self::Pairs(p) => Ok(()), // TODO: fmt for Pairs and Triplets
			Self::Triplets(t) => Ok(()),
			Self::Bytes(v) => write!(f, "= [{}]", v.iter().map(|i| format!("{:x}", i)).collect::<Vec<String>>().join(" "))
		}
	}
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
		// Default value of #address-cells and #size-cells
		AddressSizeCells { address_cells: 2, size_cells: 1 } 
	}

	pub fn set(&mut self, address_cells: u32, size_cells: u32) {
		self.address_cells = address_cells;
		self.size_cells = size_cells;
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

impl core::fmt::Display for StatusValue {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Okay => write!(f, "okay"),
			Self::Disabled => write!(f, "disabled"),
			Self::Reserved => write!(f, "reserved"),
			Self::Fail => write!(f, "fail"),
			Self::FailSss => write!(f, "fail-sss")
		}
	}
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
