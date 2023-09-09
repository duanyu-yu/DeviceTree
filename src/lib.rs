#![no_std]

#[cfg_attr(test, macro_use)]
pub mod tree;
pub mod fdt;
pub mod utils;

#[cfg(test)]
mod tests;

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

use crate::tree::node::DeviceTreeNodeWrap;
use crate::fdt::{
	header::FdtHeader,
	blob::{
		FdtReserveEntry, 
		FdtStructBlock, 
		FdtStringsBlock
	},
};

#[derive(Debug, PartialEq)]
pub enum DeviceTreeError {
	/* Device Tree parsing error */
	BadMagic(u32),
    BadVersion(u32),
	BadToken,
	BadStringsBlockOffset,
	NotAToken,
	BadPropValue,
	BadPropType,
	PropAlreadyParsed,
    /* Device Tree processing error */
	CpuNumInvalid,
}

pub struct DeviceTree {
	root: DeviceTreeNodeWrap
}

pub struct DeviceTreeBlob<'a> {
	header: FdtHeader,
    memory_reservation_block: Vec<FdtReserveEntry>,
    structure_block: FdtStructBlock<'a>,
    strings_block: FdtStringsBlock<'a>
}

impl<'a> DeviceTreeBlob<'a> {
    /// Gets a reference to the [FdtHeader].
    pub const fn header(&self) -> &FdtHeader {
        &self.header
    }

    /// Gets a reference to the list of [FdtReserveEntry].
    pub fn memory_reservation_block(&self) -> &[FdtReserveEntry] {
        self.memory_reservation_block.as_ref()
    }
}
