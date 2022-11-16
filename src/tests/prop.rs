use alloc::string::String;
use core::convert::From;

use crate::tree::prop::{
	DeviceTreeProperty, 
	Pairs, 
	Triplets
};

#[test]
fn prop_value() {
	let string_list = vec![String::from("string1"), String::from("string2")];
	let comp = DeviceTreeProperty::StringList(string_list);

	assert_eq!(comp.to_stringfmt(), String::from("'string1', 'string2'"));


	let string = DeviceTreeProperty::String(String::from("string"));

	assert_eq!(string.to_stringfmt(), String::from("string"));


	let u32 = DeviceTreeProperty::U32(16_u32);

	assert_eq!(u32.to_stringfmt(), String::from("0x10"));
	

	let reg_value = Pairs(vec![(vec![222_u32, 1_u32], vec![16_u32, 204_u32]), (vec![256_u32], vec![172_u32])]);
	let reg = DeviceTreeProperty::Pairs(reg_value);

	assert_eq!(reg.to_stringfmt(), String::from("0xDE 0x1 0x10 0xCC 0x100 0xAC"));


	let ranges_value = Some(Triplets(vec![(vec![0xDE_u32], vec![0xAC_u32, 0x10_u32], vec![0x100_u32])]));
	let ranges = DeviceTreeProperty::Triplets(ranges_value);

	assert_eq!(ranges.to_stringfmt(), String::from("0xDE 0xAC 0x10 0x100"));


	let empty = DeviceTreeProperty::Empty;

	assert_eq!(empty.to_stringfmt().is_empty(), true);
}
