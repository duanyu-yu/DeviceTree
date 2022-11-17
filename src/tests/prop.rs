use crate::tree::prop::{
	DeviceTreeProperty, 
};

#[test]
fn prop_value() {
	let string_list = "string1\0string2\0";
	let mut prop_stringlist = DeviceTreeProperty::from_bytes("compatible", string_list.as_bytes());
	prop_stringlist.update_type();

	assert_eq!(format!("{};", prop_stringlist), "compatible = \"string1\0string2\0\";");

	let string = "string";
	let mut prop_string = DeviceTreeProperty::from_bytes("model", string.as_bytes());
	prop_string.update_type();

	assert_eq!(format!("{};", prop_string), "model = \"string\";");

	let u32 = 16_u32;
	let mut prop_u32 = DeviceTreeProperty::from_bytes("clock-frequency", &u32.to_be_bytes());
	prop_u32.update_type();
 
	assert_eq!(format!("{};", prop_u32), "clock-frequency = <0x10>;");  

	let empty: &[u8] = &[];
	let mut prop_empty = DeviceTreeProperty::from_bytes("dma-coherent", empty);
	prop_empty.update_type();

	assert_eq!(format!("{};", prop_empty), "dma-coherent;");
}
