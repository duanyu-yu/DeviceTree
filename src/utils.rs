use core::{
    mem::discriminant,
    ffi::CStr
};
use alloc::{
    vec::Vec,
    string::String
};

/// Pop the first n-bytes from input, and return it
/// 
/// Returns None and does not modify the slice if the given length is out of bounds.
pub(crate) fn pop_slice<'a>(input: &mut &'a [u8], len: usize) -> Option<&'a [u8]> {
    if len < input.len() {
        let out = Some(&input[..len]);
        *input = &input[len..];
        out
    } else {
        None
    }
}

/// Read from a slice as a u32 in big endian
/// 
/// Note: After read, the input will point to unread position
pub(crate) fn take_be_u32(input: &mut &[u8]) -> Option<u32> {
    Some(u32::from_be_bytes(pop_slice(input, 4)?.try_into().unwrap()))
}

/// Read from a slice as a u64 in big endian
/// 
/// Note: After read, the input will point to unread position
pub(crate) fn take_be_u64(input: &mut &[u8]) -> Option<u64> {
    Some(u64::from_be_bytes(pop_slice(input, 8)?.try_into().unwrap()))
}

/// Read first 4 bytes from a slice as a u32 in big endian
pub(crate) fn read_first_be_u32(input: &[u8]) -> Option<u32> {
    Some(u32::from_be_bytes(input.get(..4)?.try_into().unwrap()))
}

/// Read first 8 bytes from a slice as a u64 in big endian
pub(crate) fn read_first_be_u64(input: &[u8]) -> Option<u64> {
    Some(u64::from_be_bytes(input.get(..8)?.try_into().unwrap()))
}

pub(crate) fn take_utf8_until_nul_aligned<'a>(input: &mut &'a [u8], align: usize) -> Option<&'a str> {
    let c_str = CStr::from_bytes_until_nul(input).unwrap();

    let str = c_str.to_str().unwrap();

    let len = c_str.to_bytes_with_nul().len();

    if align != 0 {
        pop_slice(input, len + (align - (len % align)) % align)?;
    } else {
        pop_slice(input, len)?;
    }

    Some(str)
}

pub(crate) fn take_utf8_until_nul<'a>(input: &mut &'a [u8]) -> Option<&'a str> {
    let c_str = CStr::from_bytes_until_nul(input).unwrap();

    let str = c_str.to_str().unwrap();

    let len = c_str.to_bytes_with_nul().len();

    pop_slice(input, len)?;

    Some(str)
}

pub(crate) fn take_aligned<'a>(input: &mut &'a [u8], len: usize, align: usize) -> Option<&'a [u8]> {
    pop_slice(input, len + (align - (len % align)) % align)?.get(..len)
}

/// A function that compares two enums by their variant
/// 
/// Returns true if both enums are same variant
pub fn variant_eq<T>(a: &T, b: &T) -> bool {
	discriminant(a) == discriminant(b)
}

/// A function that print vector of strings in the form '<String1>', '<String2>'
pub fn vec_strings_fmt(v: &Vec<String>) -> String {
	let v_fmt: Vec<String> = v.iter().map(|i| format!("'{}'", i)).collect();

	v_fmt.join(", ")
}
