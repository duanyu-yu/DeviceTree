use log::debug;

use crate::{
    utils, 
    DeviceTreeError
};

/// FDT Header Magic Number
const FDT_MAGIC: u32 = 0xd00dfeed;

/// Version Number 
/// 
/// Note: The version is 17 if using the structure as defined in https://github.com/devicetree-org/devicetree-specification/releases/tag/v0.4-rc1
const VERSION_NUMBER: u32 = 17;

pub struct FdtHeader {
    /// The magic value, shall be 0xd00dfeed (big-endian).
	magic: u32,
    /// The total size in bytes of the devicetree data structure.
    totalsize: u32,
    /// The offset in bytes of the structure block from the beginning of the header.
    off_dt_struct: u32,
    /// The offset in bytes of the strings block from the beginning of the header.
    off_dt_strings: u32,
    /// The offset in bytes of the memory reservation block from the beginning of the header.
    off_mem_rsvmap: u32,
    /// The version of the devicetree data structure.
    version: u32,
    /// The lowest version of the devicetree data structure with which the version used is backwards compatible.
    last_comp_version: u32,
    /// The physical ID of the system’s boot CPU.
    /// It shall be identical to the physical ID given in the reg property of that CPU node within the devicetree.
    boot_cpuid_phys: u32,
    /// The length in bytes of the strings block section of the devicetree blob.
    size_dt_strings: u32,
    /// the length in bytes of the structure block section of the devicetree blob.
    size_dt_struct: u32
}

impl FdtHeader {
    /// The offset in bytes of the structure block from the beginning of the header.
    pub const fn off_dt_struct(&self) -> u32 {
        self.off_dt_struct
    }

    /// The offset in bytes of the strings block from the beginning of the header.
    pub const fn off_dt_strings(&self) -> u32 {
        self.off_dt_strings
    }

    /// The offset in bytes of the memory reservation block from the beginning of the header.
    pub const fn off_mem_rsvmap(&self) -> u32 {
        self.off_mem_rsvmap
    }

    /// The lowest version of the devicetree data structure with which the version used is backwards compatible.
    pub const fn last_comp_version(&self) -> u32 {
        self.last_comp_version
    }

    /// The physical ID of the system’s boot CPU.
    /// It shall be identical to the physical ID given in the reg property of that CPU node within the devicetree.
    pub const fn boot_cpuid_phys(&self) -> u32 {
        self.boot_cpuid_phys
    }

    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, DeviceTreeError> {
        debug!("Parsing FDT header from bytes.");

        let header = Self {
            magic: utils::take_be_u32(bytes).unwrap(), 
            totalsize: utils::take_be_u32(bytes).unwrap(), 
            off_dt_struct: utils::take_be_u32(bytes).unwrap(), 
            off_dt_strings: utils::take_be_u32(bytes).unwrap(), 
            off_mem_rsvmap: utils::take_be_u32(bytes).unwrap(), 
            version: utils::take_be_u32(bytes).unwrap(), 
            last_comp_version: utils::take_be_u32(bytes).unwrap(), 
            boot_cpuid_phys: utils::take_be_u32(bytes).unwrap(), 
            size_dt_strings: utils::take_be_u32(bytes).unwrap(), 
            size_dt_struct: utils::take_be_u32(bytes).unwrap() 
        };

        let check = header.check();

        match check {
            Ok(_) => {
                debug!("Valid header!");
                return Ok(header);
            },
            Err(error) => {
                debug!("Invalid magic number and/or version!");
                return Err(error);
            }
        }
    }

    pub fn magic_check(&self) -> Result<(), DeviceTreeError> {
        match self.magic {
            FDT_MAGIC => Ok(()),
            bad => Err(DeviceTreeError::BadMagic(bad))
        }
    }

    pub fn version_check(&self) -> Result<(), DeviceTreeError> {
        match self.version {
            VERSION_NUMBER => Ok(()), 
            bad => Err(DeviceTreeError::BadVersion(bad))
        }
    }

    pub fn check(&self) -> Result<(), DeviceTreeError> {
        self.magic_check()?;
        self.version_check()?;
        Ok(())
    }

    pub fn totalsize(&self) -> usize {
        self.totalsize as usize
    }

    pub fn size_dt_struct(&self) -> usize {
        self.size_dt_struct as usize
    }

    pub fn size_dt_strings(&self) -> usize {
        self.size_dt_strings as usize
    }
}
