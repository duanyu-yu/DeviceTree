use alloc::{
	string::{String, ToString},
	rc::Rc
};
use core::convert::From;

use crate::{
	DeviceTree,
	DeviceTreeBlob,
	utils,
	fdt::blob::Token,
	tree::{
		node::{
			DeviceTreeNode,
			AddChild,
		},
		prop::{
			DeviceTreeProperty, 
			StatusValue, 
			Pairs, 
			Triplets
		}
	},
};

#[test]
fn set_parent() {
	let parent = DeviceTreeNode::new_wrap();
	let child = DeviceTreeNode::new_wrap();

	child.borrow_mut().set_parent(Rc::clone(&parent));

	assert_eq!(child.borrow().parent(), Some(&Rc::clone(&parent)));
}

#[test]
fn add_child() {
	let parent = DeviceTreeNode::new_wrap();

	parent.borrow_mut().set_label("parent");

	let child = DeviceTreeNode::new_wrap();

	child.borrow_mut().set_label("child");

	parent.add_child("child", Rc::clone(&child));

	let parent_of_child = Rc::clone(&child.borrow().parent().as_ref().unwrap());

	assert_eq!(parent_of_child.borrow().label(), Some(&"parent".to_string()));
	assert_eq!(parent.borrow().find_child("child").unwrap().borrow().label(), Some(&"child".to_string()));
}

#[test]
fn add_prop() {
	let mut node = DeviceTreeNode::new();

	assert_eq!(node.add_prop("name", DeviceTreeProperty::String("old".to_string())), None);
	assert_eq!(node.prop_exists("name"), true);

	assert_eq!(node.add_prop("name", DeviceTreeProperty::String("new".to_string())), Some(DeviceTreeProperty::String("old".to_string())));

	assert_eq!(node.prop("name"), Some(&DeviceTreeProperty::String("new".to_string())));
}

#[test]
fn delete_prop() {
	let mut node = DeviceTreeNode::new();

	node.add_prop("name", DeviceTreeProperty::Empty);

	assert_eq!(node.prop_exists("name"), true);

	node.remove_prop("name");

	assert_eq!(node.prop_exists("name"), false);
}

#[test]
fn prop_value() {
	let string_list = vec![String::from("string1"), String::from("string2")];
	let comp = DeviceTreeProperty::StringList(string_list);

	assert_eq!(comp.to_stringfmt(), String::from("'string1', 'string2'"));


	let string = DeviceTreeProperty::String(String::from("string"));

	assert_eq!(string.to_stringfmt(), String::from("string"));


	let u32 = DeviceTreeProperty::U32(16_u32);

	assert_eq!(u32.to_stringfmt(), String::from("0x10"));


	let status = DeviceTreeProperty::Status(StatusValue::Okay);

	assert_eq!(status.to_stringfmt(), String::from("okay"));


	let reg_value = Pairs(vec![(vec![222_u32, 1_u32], vec![16_u32, 204_u32]), (vec![256_u32], vec![172_u32])]);
	let reg = DeviceTreeProperty::Pairs(reg_value);

	assert_eq!(reg.to_stringfmt(), String::from("0xDE 0x1 0x10 0xCC 0x100 0xAC"));


	let ranges_value = Some(Triplets(vec![(vec![0xDE_u32], vec![0xAC_u32, 0x10_u32], vec![0x100_u32])]));
	let ranges = DeviceTreeProperty::Triplets(ranges_value);

	assert_eq!(ranges.to_stringfmt(), String::from("0xDE 0xAC 0x10 0x100"));


	let empty = DeviceTreeProperty::Empty;

	assert_eq!(empty.to_stringfmt().is_empty(), true);
}

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

#[test]
fn devicetreeblob() {
    let mut dtb: &[u8] = include_bytes!("../../dtb/test1.dtb");

    let blob = DeviceTreeBlob::from_bytes(&mut dtb);

    assert!(blob.is_ok());
}

#[test]
fn pop_slice() {
	let mut bytes: &[u8] = b"yesokyesok";

	let first = utils::pop_slice(&mut bytes, 4).unwrap();

	assert_eq!(first, b"yeso");

	assert_eq!(bytes, b"kyesok");
}

#[test]
fn cstr_from_utf8() {
	use core::ffi::CStr;

	let bytes = b"hello\0world";

	let cstr = CStr::from_bytes_until_nul(bytes).unwrap();

	assert_eq!(cstr.to_str(), Ok("hello"));
}

#[test]
fn take_utf8_until_nul_aligned() {
	let mut bytes: &[u8] = b"hello\0world";

	let str = utils::take_utf8_until_nul_aligned(&mut bytes, 0);

	assert_eq!(str, Some("hello"));

	assert_eq!(bytes, b"world".as_slice());
}

#[test]
fn parsing() {
	let mut dtb: &[u8] = include_bytes!("../../dtb/test1.dtb");

    let blob = DeviceTreeBlob::from_bytes(&mut dtb).unwrap();

	let structure_block = blob.structure_block();

	let mut block_bytes = structure_block.bytes();

	let cursor = Token::from_bytes(&mut block_bytes).unwrap();

	assert_eq!(cursor, Token::TokenBeginNode);
}

#[test]
fn strings_block() {
	let mut dtb: &[u8] = include_bytes!("../../dtb/test1.dtb");

    let blob = DeviceTreeBlob::from_bytes(&mut dtb).unwrap();

	let strings_block = blob.strings_block();

	assert_eq!(strings_block.find(0), Ok("#address-cells"));

	assert_eq!(strings_block.find(15), Ok("#size-cells"));

	assert_eq!(strings_block.find(27), Ok("compatible"));
}

#[test]
fn blob_to_tree() {
    let mut dtb: &[u8] = include_bytes!("../../dtb/test1.dtb");

    let mut blob = DeviceTreeBlob::from_bytes(&mut dtb).unwrap();

    let tree = blob.to_tree().unwrap();

    assert_eq!(tree.num_cpus(), 4);
}
