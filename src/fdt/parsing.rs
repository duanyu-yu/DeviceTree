use core::ffi::CStr;
use alloc::{
    rc::Rc,
    vec::Vec
};
use libc_print::std_name::println;

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
    FdtStringsBlock,
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
        println!("[BLOB] Parsing dtb; dtb located at {:#x}", bytes as *const _ as usize);

        let header = FdtHeader::from_bytes(bytes)?;

        let mut memory_reservation_vec: Vec<FdtReserveEntry> = Vec::new();

        while let Some(entry) = FdtReserveEntry::from_bytes(bytes) {
            if !entry.end_of_list() {
                println!("[BLOB] Adding reserved memory entry.");
                memory_reservation_vec.push(entry);
            } else {
                println!("[BLOB] End of adding reserved memory entry.");
                break;
            }
        }

        let structure_block_size = header.size_dt_struct();
        let string_block_size = header.size_dt_strings();

        Ok( Self {
            header: header,
            memory_reservation_block: memory_reservation_vec,
            structure_block: FdtStructBlock::from_bytes(bytes.take(..structure_block_size).unwrap()),
            strings_block: FdtStringsBlock::from_bytes(bytes.take(..string_block_size).unwrap()) 
        })
    }

    pub fn to_tree(&mut self) -> Result<DeviceTree, DeviceTreeError> {
        self.structure_block.parsing(&self.strings_block)
    }

    pub fn structure_block(&self) -> &FdtStructBlock {
        &self.structure_block
    }

    pub fn strings_block(&self) -> &FdtStringsBlock {
        &self.strings_block
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
        println!("[BLOB] Before read prop describe: {:?}", bytes);

        let len = utils::take_be_u32(bytes)?;
        let name_off = utils::take_be_u32(bytes)?;

        println!("[BLOB] Prop: len = {}, name_off = {}", len, name_off);

        println!("[BLOB] After read prop describe: {:?}", bytes);

        Some( Self {
            len: len,
            name_off: name_off 
        })
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn name_off(&self) -> usize {
        self.name_off as usize
    }
}

impl core::fmt::Display for FdtPropDescribe {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "len: {}, name_offset: {}", self.len, self.name_off)
    }
}

impl<'a> FdtStructBlock<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }

    pub fn bytes(&self) -> &[u8] {
        self.0
    }

    pub fn parsing(&mut self, strings_block: &FdtStringsBlock) -> Result<DeviceTree, DeviceTreeError> {
        println!("[BLOB] Converting dtb to tree structure.");

        let tree = DeviceTree::new();
        let root = tree.root();

        let mut current = Rc::clone(root);
        let mut last = DeviceTreeNode::new_wrap();

        let mut token = Block::TokenBeginNode;

        while let Some(cursor) = Block::from_bytes(&mut self.0) {
            if cursor.is_end() {
                break;
            }

            match cursor {
                Block::Data(mut bytes) => { 
                    match token {
                        Block::TokenBeginNode => { 
                            let name = utils::take_utf8_until_nul_aligned(&mut bytes, 4).unwrap();

                            self.0 = bytes;

                            if name == "" {
                                println!("[BLOB] Adding root node.");
                                continue;
                            }

                            let next = DeviceTreeNode::new_wrap();
                            current.borrow_mut().update_child(name, Rc::clone(&next));
                            last = Rc::clone(&current);
                            current = Rc::clone(&next);
                            
                        }
                        Block::TokenProp => {
                            let prop_describe = FdtPropDescribe::from_bytes(&mut bytes).unwrap();

                            println!("[BLOB] Property describe data: {}", prop_describe);

                            let name = strings_block.find(prop_describe.name_off()).unwrap();

                            println!("[BLOB] Before take value: {:?}", bytes);

                            let value = utils::take_aligned(&mut bytes, prop_describe.len(), 4).unwrap();

                            println!("[BLOB] After take value: {:?}", bytes);

                            println!("[BLOB] Property: {} = {}", name, CStr::from_bytes_until_nul(value).unwrap().to_str().unwrap());

                            self.0 = bytes;

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

        Ok(tree)

    }
}

impl<'a> FdtStringsBlock<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }

    pub fn find(&self, offset: usize) -> Result<&str, DeviceTreeError> {
        if offset > self.0.len() {
            return Err(DeviceTreeError::BadStringsBlockOffset);
        }

        println!("[BLOB] Searching for property name at offset {}", offset);

        let find = self.0.get(offset..).unwrap();

        let name = CStr::from_bytes_until_nul(find).unwrap().to_str().unwrap();

        println!("[BLOB] Find name '{}' at offset {}", name, offset);

        Ok(name)
    }
}

impl<'a> Block<'a> {
    pub fn from_bytes(bytes: &mut &'a [u8]) -> Option<Self> {
        let mut output = Self::Data(bytes);

        match utils::read_first_be_u32(bytes)? {
            FDT_BEGIN_NODE => output = Self::TokenBeginNode,
            FDT_END_NODE => output = Self::TokenEndNode,
            FDT_PROP => output = Self::TokenProp, 
            FDT_NOP => output = Self::TokenNop,
            FDT_END => output = Self::TokenEnd,
            _ => ()
        }

        if !output.is_data() {
            utils::pop_slice(bytes, 4)?;
        }

        Some(output)
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

    pub fn data(&self) -> Option<&[u8]> {
        match self {
            Self::Data(bytes) => Some(bytes),
            _ => None
        }
    } 
}

impl<'a> core::fmt::Display for Block<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Data(bytes) => write!(f, "{:x?}", bytes),
            Self::TokenBeginNode => write!(f, "TOKEN_BEGIN_NODE"),
            Self::TokenEndNode => write!(f, "TOKEN_END_NODE"),
            Self::TokenProp => write!(f, "TOKEN_PROP"),
            Self::TokenNop => write!(f, "TOKEN_NOP"),
            Self::TokenEnd => write!(f, "TOKEN_END"),
        }
    }
}
