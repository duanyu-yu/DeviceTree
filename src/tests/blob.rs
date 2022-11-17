use crate::{
	DeviceTreeBlob,
	fdt::blob::Token
};

#[test]
fn dtb() {
    let mut dtb: &[u8] = include_bytes!("./dtb/test1.dtb");

    let blob = DeviceTreeBlob::from_bytes(&mut dtb);

    assert!(blob.is_ok());
}

#[test]
fn parsing() {
	let mut dtb: &[u8] = include_bytes!("./dtb/test1.dtb");

    let blob = DeviceTreeBlob::from_bytes(&mut dtb).unwrap();

	let structure_block = blob.structure_block();

	let mut block_bytes = structure_block.bytes();

	let cursor = Token::from_bytes(&mut block_bytes).unwrap();

	assert_eq!(cursor, Token::TokenBeginNode);
}

#[test]
fn strings_block() {
	let mut dtb: &[u8] = include_bytes!("./dtb/test1.dtb");

    let blob = DeviceTreeBlob::from_bytes(&mut dtb).unwrap();

	let strings_block = blob.strings_block();

	assert_eq!(strings_block.find(0), Ok("#address-cells"));

	assert_eq!(strings_block.find(15), Ok("#size-cells"));

	assert_eq!(strings_block.find(27), Ok("compatible"));
}

#[test]
fn blob_to_tree() {
    let mut dtb: &[u8] = include_bytes!("./dtb/test1.dtb");

    let mut blob = DeviceTreeBlob::from_bytes(&mut dtb).unwrap();

    let tree = blob.to_tree().unwrap();

    assert_eq!(tree.num_cpus(), 4);
}
