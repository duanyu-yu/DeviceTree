use alloc::{
	string::{
		String, 
		ToString
	},
	vec::Vec
};
use core::convert::From;

use crate::{
	utils::{self, variant_eq}, 
	DeviceTreeError
};

/* Property of devicetree: 
Each node in the devicetree has properties that describe the characteristics of the node. */
#[derive(Clone, PartialEq, Debug)]
pub enum DeviceTreeProperty {
	Empty,
	StringList(Vec<String>), 
	String(String), 
	U32(u32),
	U64(u64),
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
			Self::Pairs(p) => Ok(()), // TODO: fmt for Pairs and Triplets
			Self::Triplets(t) => Ok(()),
			Self::Bytes(v) => write!(f, "= [{}]", v.iter().map(|i| format!("{:x}", i)).collect::<Vec<String>>().join(" "))
		}
	}
}

impl DeviceTreeProperty {
	pub fn from_bytes(bytes: &mut &[u8], value_type: DeviceTreePropertyType) -> Result<Self, DeviceTreeError> {
		match value_type {
			DeviceTreePropertyType::Empty => Ok(Self::Empty),
			DeviceTreePropertyType::StringList => {
				// TODO: A string-list consists of a concatenated list of null terminated strings
				// let mut vec_string: Vec<String> = Vec::new();
				// loop {
				// 	let s = utils::take_utf8_until_nul(bytes).unwrap();

				// 	if s.is_empty() {
				// 		break;
				// 	}

				// 	vec_string.push(s.to_string());
				// }
				
				// Ok(Self::StringList(vec_string))

				let s = String::from_utf8(bytes.to_vec()).unwrap();

				Ok(Self::StringList(vec![s]))
			}
			DeviceTreePropertyType::String => {
				let s = utils::take_utf8_until_nul(bytes).unwrap();
				
				if s.is_empty() {
					Err(DeviceTreeError::BadPropValue)
				} else {
					Ok(Self::String(s.to_string()))
				}
			}
			DeviceTreePropertyType::U32 => Ok(Self::U32(utils::take_be_u32(bytes).unwrap())),
			DeviceTreePropertyType::U64 => Ok(Self::U64(utils::take_be_u64(bytes).unwrap())),
			DeviceTreePropertyType::Bytes => Ok(Self::Bytes(bytes.to_vec())),
			_ => Err(DeviceTreeError::BadPropType)
		}
	}

	pub fn is_bytes(&self) -> bool {
		variant_eq(self, &Self::Bytes(Vec::new()))
	}

	pub fn to_stringfmt(&self) -> String { 
		match self {
			DeviceTreeProperty::Empty => String::new(),
			DeviceTreeProperty::StringList(v) => utils::vec_strings_fmt(v),  
			DeviceTreeProperty::String(v) => v.to_string(),
			DeviceTreeProperty::U32(v) => format!("0x{:X}", v),
			DeviceTreeProperty::U64(v) => format!("0x{:X}", v),
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
pub struct NumCells {
	address_cells: u32,
	size_cells: u32
}

impl NumCells {
	pub fn new() -> Self {
		// Default value of #address-cells and #size-cells
		NumCells { address_cells: 2, size_cells: 1 } 
	}

	pub fn set(&mut self, address_cells: u32, size_cells: u32) {
		self.address_cells = address_cells;
		self.size_cells = size_cells;
	}

	pub fn set_addr_cells(&mut self, addr_cells: u32) {
		self.address_cells = addr_cells;
	}

	pub fn set_size_cells(&mut self, size_cells: u32) {
		self.size_cells = size_cells;
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

pub enum DeviceTreePropertyType {
	Empty,
	StringList,
	String, 
	U32,
	U64,
	Pairs,
	Triplets,
	Bytes
}
