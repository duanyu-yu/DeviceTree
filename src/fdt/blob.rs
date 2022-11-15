#[derive(Clone, Copy)]
pub struct FdtReserveEntry {
    pub(crate) address: u64,
    pub(crate) size: u64
}

pub struct FdtStructBlock<'a>(pub(crate) &'a [u8]);

pub struct FdtStringsBlock<'a>(pub(crate) &'a [u8]);

pub struct FdtPropDescribe {
    pub(crate) len: u32,
    pub(crate) name_off: u32
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Token {
    TokenBeginNode, 
    TokenEndNode,
    TokenProp, 
    TokenNop,
    TokenEnd
}
