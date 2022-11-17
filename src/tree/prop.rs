use alloc::{
	string::{
		String, 
		ToString
	},
	vec::Vec
};
use core::convert::From;

use crate::{
	utils, 
	DeviceTreeError
};

/* Property of devicetree: 
Each node in the devicetree has properties that describe the characteristics of the node. */
#[derive(PartialEq, Debug)]
pub struct  DeviceTreeProperty {
	name: String,
	raw_value: Vec<u8>,
	value_type: DeviceTreePropertyType
}

impl core::fmt::Display for DeviceTreeProperty {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self.value_type {
			DeviceTreePropertyType::Empty => write!(f, "{};", self.name),
			DeviceTreePropertyType::String => write!(f, "{} = \"{}\"", self.name, String::from_utf8(self.raw_value.to_vec()).unwrap()),
			DeviceTreePropertyType::StringList => write!(f, "{} = \"{}\"", self.name, String::from_utf8(self.raw_value.to_vec()).unwrap()),
			DeviceTreePropertyType::U32 => write!(f, "{} = <{:#x}>", self.name, utils::read_first_be_u32(&mut self.raw_value.as_slice()).unwrap()),
			DeviceTreePropertyType::U64 => write!(f, "{} = <{:#x}>", self.name, utils::read_first_be_u64(&mut self.raw_value.as_slice()).unwrap()),
			DeviceTreePropertyType::Bytes => write!(f, "{} = [{}]", self.name, self.raw_value.iter().map(|i| format!("{:02x}", i)).collect::<Vec<String>>().join(" ")),
			DeviceTreePropertyType::Raw => write!(f, "{} = (raw) [{}]", self.name, self.raw_value.iter().map(|i| format!("{:x}", i)).collect::<Vec<String>>().join(" "))
		}
	}
}

impl DeviceTreeProperty {
	pub fn from_bytes(name: &str, bytes: &[u8]) -> Self {
		Self { 
			name: name.to_string(), 
			raw_value: bytes.to_vec(), 
			value_type: DeviceTreePropertyType::Raw 
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn update_type(&mut self) {
		self.value_type = match self.name.as_str() {
			"#address-cells" => DeviceTreePropertyType::U32,
			"#size-cells" => DeviceTreePropertyType::U32,
			"#interrupt-cells" => DeviceTreePropertyType::U32,
			"compatible" => DeviceTreePropertyType::StringList,
			"model" => DeviceTreePropertyType::String,
			"phandle" => DeviceTreePropertyType::U32,
			"status" => DeviceTreePropertyType::String,
			"virtual-reg" => DeviceTreePropertyType::U32,
			"dma-coherent" => DeviceTreePropertyType::Empty,
			"name" => DeviceTreePropertyType::String,
			"device_type" => DeviceTreePropertyType::String,
			"timebase-frequency" => DeviceTreePropertyType::U32,
			"clock-frequency" => DeviceTreePropertyType::U32,
			"local-mac-address" => DeviceTreePropertyType::Bytes,
			_ => DeviceTreePropertyType::Raw
		};
	}

	pub fn u32(&self) -> Result<u32, DeviceTreeError> {
		if self.value_type != DeviceTreePropertyType::U32 {
			return Err(DeviceTreeError::BadPropType);
		}

		Ok(utils::read_first_be_u32(&mut self.raw_value.as_slice()).unwrap())
	}

    pub fn u64(&self) -> Result<u64, DeviceTreeError> {
		if self.value_type != DeviceTreePropertyType::U64 {
			return Err(DeviceTreeError::BadPropType);
		}

		Ok(utils::read_first_be_u64(&mut self.raw_value.as_slice()).unwrap())
	}

    pub fn string(&self) -> Result<String, DeviceTreeError> {
		if self.value_type != DeviceTreePropertyType::String {
			return Err(DeviceTreeError::BadPropType);
		}

		Ok(String::from_utf8(self.raw_value.to_vec()).unwrap())
	}

    pub fn stringlist(&self) -> Result<Vec<String>, DeviceTreeError> {
		if self.value_type != DeviceTreePropertyType::StringList {
			return Err(DeviceTreeError::BadPropType);
		}

		let mut vec_string: Vec<String> = Vec::new();

		loop {
			let s = utils::take_utf8_until_nul(&mut self.raw_value.as_slice().clone()).unwrap();

			if s.is_empty() {
				break;
			}

			vec_string.push(s.to_string());
		}

		Ok(vec_string)
	}

    pub fn bytes(&self) -> Result<Vec<u8>, DeviceTreeError> {
		if self.value_type != DeviceTreePropertyType::StringList {
			return Err(DeviceTreeError::BadPropType);
		}

		Ok(self.raw_value.clone())
	}
}

#[derive(PartialEq, Debug)]
pub enum DeviceTreePropertyType {
	Empty,
	StringList,
	String, 
	U32,
	U64,
	Bytes,
	Raw
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
