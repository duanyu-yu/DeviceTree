use core::ffi::CStr;
use alloc::rc::Rc;
use alloc::vec::Vec;
use log::info;

use super::header::FdtHeader;
use crate::{
    utils,
    DeviceTree, 
    DeviceTreeError, 
    DeviceTreeBlob,
    tree::node::{DeviceTreeNode, DeviceTreeProperty}
};
use super::blob::{
    FdtReserveEntry,
    FdtPropDescribe,
    FdtStructBlock,
    FdtStringBlock,
    Block
};

/* FDT Token */
const FDT_BEGIN_NODE: u32 = 0x00000001;
const FDT_END_NODE: u32 = 0x00000002;
const FDT_PROP: u32 = 0x00000003;
const FDT_NOP: u32 = 0x00000004;
const FDT_END: u32 = 0x00000009;

impl<'a> DeviceTreeBlob<'a> {
    pub fn from_bytes(bytes: &mut &'a [u8]) -> Result<Self, DeviceTreeError> {
        let header = FdtHeader::from_bytes(bytes)?;

        let mut memory_reservation_vec: Vec<FdtReserveEntry> = Vec::new();

        while let Some(entry) = FdtReserveEntry::from_bytes(bytes) {
            if !entry.end_of_list() {
                memory_reservation_vec.push(entry);
            } else {
                break;
            }
        }

        let structure_block_size = header.size_dt_struct();
        let string_block_size = header.size_dt_strings();

        Ok( Self {
            header: header,
            memory_reservation_block: memory_reservation_vec,
            structure_block: FdtStructBlock::from_bytes(bytes.take(..structure_block_size).unwrap()),
            strings_block: FdtStringBlock::from_bytes(bytes.take(..string_block_size).unwrap()) 
        })
    }

    pub fn to_tree(&mut self) -> Result<DeviceTree, DeviceTreeError> {
        self.structure_block.parsing(&self.strings_block)
    }
}


impl FdtReserveEntry {
    pub fn from_bytes(bytes: &mut &[u8]) -> Option<Self> {
        Some( Self {
            address: utils::take_be_u64(bytes)?,
            size: utils::take_be_u64(bytes)?
        })
    }

    /// Return true if both address and size are equal to zero, which means that the list of reservation block is terminated here
    pub fn end_of_list(&self) -> bool {
        self.address == 0 && self.size == 0
    }
}

impl FdtPropDescribe {
    pub fn from_bytes(bytes: &mut &[u8]) -> Option<Self> {
        Some( Self {
            len: utils::take_be_u32(bytes)?,
            name_off: utils::take_be_u32(bytes)?
        })
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn name_off(&self) -> usize {
        self.name_off as usize
    }
}

impl<'a> FdtStructBlock<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }

    pub fn parsing(&mut self, strings_block: &FdtStringBlock) -> Result<DeviceTree, DeviceTreeError> {
        let tree = DeviceTree::new();
        let root = tree.root();

        let mut current = Rc::clone(root);
        let mut last = DeviceTreeNode::new_wrap();

        let mut token = Block::TokenBeginNode;

        while let Some(cursor) = Block::from_bytes(self.0) {
            while !cursor.is_end() {
                match cursor {
                    Block::Data(mut bytes) => { 
                        match token {
                            Block::TokenBeginNode => { 
                                let name = utils::take_utf8_until_nul_aligned(&mut bytes, 4).unwrap();
                                if name == "/" {
                                    continue;
                                } else {
                                    let next = DeviceTreeNode::new_wrap();
                                    current.borrow_mut().update_child(name, Rc::clone(&next));
                                    last = Rc::clone(&current);
                                    current = Rc::clone(&next);
                                }
                            }
                            Block::TokenProp => {
                                let prop_describe = FdtPropDescribe::from_bytes(&mut bytes).unwrap();

                                let name = strings_block.find(prop_describe.name_off()).unwrap();
                                let value = utils::take_aligned(&mut bytes, prop_describe.len(), 4).unwrap();

                                current.borrow_mut().add_prop(name, DeviceTreeProperty::Bytes(value.to_vec()));
                            }
                            Block::TokenEndNode => {
                                current = Rc::clone(&last);
                            }
                            _ => ()
                        }
                    }
                    Block::TokenProp => {
                        if token.is_end_node() {
                            return Err(DeviceTreeError::BadToken);
                        }

                        token = cursor;
                    }
                    Block::TokenNop => (),
                    _ => token = cursor,
                }
            }

            if token.is_begin_node() || token.is_prop() {
                return Err(DeviceTreeError::BadToken);
            }
        }

        Ok(tree)
    }
}

impl<'a> FdtStringBlock<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }

    pub fn find(&self, offset: usize) -> Option<&str> {
        let find = self.0.get(offset..)?;

        Some(CStr::from_bytes_until_nul(find).unwrap().to_str().unwrap())
    }
}

impl<'a> Block<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Option<Self> {
        match utils::read_first_be_u32(bytes)? {
            FDT_BEGIN_NODE => Some(Self::TokenBeginNode),
            FDT_END_NODE => Some(Self::TokenEndNode),
            FDT_PROP => Some(Self::TokenProp), 
            FDT_NOP => Some(Self::TokenNop),
            FDT_END => Some(Self::TokenEnd),
            _ => Some(Self::Data(bytes))
        }
    }

    pub fn is_begin_node(self) -> bool {
        self == Self::TokenBeginNode
    }

    pub fn is_end_node(self) -> bool {
        self == Self::TokenEndNode
    }

    pub fn is_prop(self) -> bool {
        self == Self::TokenProp
    }

    pub fn is_nop(self) -> bool {
        self == Self::TokenNop 
    }

    pub fn is_end(self) -> bool {
        self == Self::TokenEnd
    }

    pub fn is_data(self) -> bool {
        utils::variant_eq(&self, &Self::Data(&[]))
    }
}
