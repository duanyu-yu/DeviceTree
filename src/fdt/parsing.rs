use core::ffi::CStr;
use alloc::{
    rc::Rc,
    vec::Vec,
};
use log::{
    info,
    debug,
};

use super::header::FdtHeader;
use crate::{
    utils,
    DeviceTree, 
    DeviceTreeError, 
    DeviceTreeBlob,
    tree::{
        node::{
            DeviceTreeNode, 
            AddChild
        }, 
        prop::DeviceTreeProperty,
    }
};
use super::blob::{
    FdtReserveEntry,
    FdtPropDescribe,
    FdtStructBlock,
    FdtStringsBlock,
    Token
};

/* FDT Token */
const FDT_BEGIN_NODE: u32 = 0x00000001;
const FDT_END_NODE: u32 = 0x00000002;
const FDT_PROP: u32 = 0x00000003;
const FDT_NOP: u32 = 0x00000004;
const FDT_END: u32 = 0x00000009;

impl DeviceTree {
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, DeviceTreeError> {
        let mut dtb = DeviceTreeBlob::from_bytes(bytes)?;

        dtb.to_tree()
    }
}

impl<'a> DeviceTreeBlob<'a> {
    pub fn from_bytes(bytes: &mut &'a [u8]) -> Result<Self, DeviceTreeError> {
        info!("Device-Tree-Blob located at {:#x}", bytes as *const _ as usize);

        let header = FdtHeader::from_bytes(bytes)?;

        let mut memory_reservation_vec: Vec<FdtReserveEntry> = Vec::new();

        while let Some(entry) = FdtReserveEntry::from_bytes(bytes) {
            if !entry.end_of_list() {
                debug!("Adding reserved memory entry.");
                memory_reservation_vec.push(entry);
            } else {
                debug!("End of adding reserved memory entry.");
                break;
            }
        }

        let structure_block_size = header.size_dt_struct();
        let string_block_size = header.size_dt_strings();

        let struct_buf = &bytes[..structure_block_size];
        *bytes = &bytes[structure_block_size..];

        let string_buf = &bytes[..string_block_size];
        *bytes = &bytes[string_block_size..];

        Ok( Self {
            header: header,
            memory_reservation_block: memory_reservation_vec,
            structure_block: FdtStructBlock::from_bytes(struct_buf),
            strings_block: FdtStringsBlock::from_bytes(string_buf), 
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
        debug!("Converting dtb to tree structure.");

        let mut current = DeviceTreeNode::new_wrap();

        current.borrow_mut().set_name("root");

        let mut bytes = self.0;

        loop {
            let token = Token::from_bytes(&mut bytes)?;

            match token {
                Token::TokenBeginNode => { 
                    let name = utils::take_utf8_until_nul_aligned(&mut bytes, 4).unwrap();
    
                    if name.is_empty() {
                        debug!("Adding root node.");
                        continue;
                    }
    
                    let next = DeviceTreeNode::new_wrap();

                    current.add_child(name, Rc::clone(&next));

                    current = Rc::clone(&next);
                }
                Token::TokenProp => {
                    let prop_describe = FdtPropDescribe::from_bytes(&mut bytes).unwrap();
        
                    let name = strings_block.find(prop_describe.name_off()).unwrap();

                    let mut raw_value = utils::take_aligned(&mut bytes, prop_describe.len(), 4).unwrap();
        
                    let prop = DeviceTreeProperty::from_bytes(name, &mut raw_value);

                    current.borrow_mut().add_prop(prop);
                }
                Token::TokenEndNode => {
                    debug!("End of node '{}'.", current.borrow().name());

                    if !current.borrow().has_parent() {
                        break;
                    }

                    let parent = Rc::clone(&current.borrow().parent().unwrap());

                    current = Rc::clone(&parent);
                }
                Token::TokenEnd => {
                    break;
                }
                _ => ()
            }
        }

        debug!("End of parsing.");

        Ok(DeviceTree::new(current))
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

        let find = self.0.get(offset..).unwrap();

        let name = CStr::from_bytes_until_nul(find).unwrap().to_str().unwrap();

        Ok(name)
    }
}

impl Token {
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, DeviceTreeError> {
        match utils::take_be_u32(bytes).unwrap() {
            FDT_BEGIN_NODE => Ok(Self::TokenBeginNode),
            FDT_END_NODE => Ok(Self::TokenEndNode),
            FDT_PROP => Ok(Self::TokenProp), 
            FDT_NOP => Ok(Self::TokenNop),
            FDT_END => Ok(Self::TokenEnd),
            _ => Err(DeviceTreeError::NotAToken)
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
}

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TokenBeginNode => write!(f, "TOKEN_BEGIN_NODE"),
            Self::TokenEndNode => write!(f, "TOKEN_END_NODE"),
            Self::TokenProp => write!(f, "TOKEN_PROP"),
            Self::TokenNop => write!(f, "TOKEN_NOP"),
            Self::TokenEnd => write!(f, "TOKEN_END"),
        }
    }
}
