use crate::utils;

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
