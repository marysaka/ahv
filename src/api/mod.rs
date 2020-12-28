//! Safe API for Apple Hypervisor
use crate::ffi::types::*;
use crate::ffi::*;

use core::ffi::c_void;
use core::marker::PhantomData;

use alloc::alloc::Layout;
use alloc::vec::Vec;

/// An Hypervisor Result.
pub type Result<T> = core::result::Result<T, HypervisorError>;

/// Represent an error returned by the Hypervisor.
#[derive(Copy, Clone, Debug)]
pub enum HypervisorError {
    /// A generic error was returned by the Hypervisor.
    Error,

    /// The Hypervisor is busy.
    Busy,

    /// A bad argument was received.
    BadArgument,

    /// The guest is in an illegal state.
    IllegalGuestState,

    /// No resources availaible.
    NoResources,

    /// No device availaible.
    NoDevice,

    /// Access was denied.
    Denied,

    /// Operation unsupported.
    Unsupported,

    /// Invalid handle sent.
    InvalidHandle,

    /// The given allocation handle is still mapped.
    AllocationStillMapped,

    /// An unknown error was returned.
    Unknown(u32),
}

/// Util used to convert a hv_return_t into a Result
fn convert_hv_return(value: hv_return_t) -> Result<()> {
    if value == HV_SUCCESS {
        Ok(())
    } else {
        Err(HypervisorError::from(value))
    }
}

impl From<hv_return_t> for HypervisorError {
    fn from(value: hv_return_t) -> HypervisorError {
        match value {
            HV_SUCCESS => panic!("HV_SUCCESS was not catch beforehand for Result, this is a bug!"),
            HV_ERROR => HypervisorError::Error,
            HV_BUSY => HypervisorError::Busy,
            HV_BAD_ARGUMENT => HypervisorError::BadArgument,
            HV_ILLEGAL_GUEST_STATE => HypervisorError::IllegalGuestState,
            HV_NO_RESOURCES => HypervisorError::NoResources,
            HV_NO_DEVICE => HypervisorError::NoDevice,
            HV_DENIED => HypervisorError::Denied,
            HV_UNSUPPORTED => HypervisorError::Unsupported,
            _ => HypervisorError::Unknown(value),
        }
    }
}

/// Represent the configuration of a Virtual Machine.
#[derive(Debug)]
pub struct VirtualMachineConfiguration {
    /// The inner configuration opaque type.
    pub handle: hv_vm_config_t,
}

impl VirtualMachineConfiguration {
    /// Create a new Virtual Machine configuration instance.
    pub fn new() -> Result<Self> {
        Ok(VirtualMachineConfiguration {
            // NOTE: no configuration APIs are availaible for the VM at the time of writting, as such this is set to null.
            handle: core::ptr::null_mut(),
        })
    }
}

/// Represent the permission of a memory region.
#[derive(Copy, Clone, Debug)]
pub struct MemoryPermission {
    /// Read.
    read: bool,

    /// Write.
    write: bool,

    /// Execute.
    execute: bool,
}

impl MemoryPermission {
    /// Create a new memory permission instance.
    pub const fn new(read: bool, write: bool, execute: bool) -> Self {
        MemoryPermission {
            read,
            write,
            execute,
        }
    }

    /// Read-only.
    pub const READ: MemoryPermission = MemoryPermission::new(true, false, false);

    /// Write-only.
    pub const WRITE: MemoryPermission = MemoryPermission::new(false, true, false);

    /// Execute-only.
    pub const EXECUTE: MemoryPermission = MemoryPermission::new(false, false, true);

    /// Read Write.
    pub const READ_WRITE: MemoryPermission = MemoryPermission::new(true, true, false);

    /// Read Execute.
    pub const READ_EXECUTE: MemoryPermission = MemoryPermission::new(true, false, true);

    /// Write Execute.
    pub const WRITE_EXECUTE: MemoryPermission = MemoryPermission::new(false, true, true);

    /// Read Write Execute.
    pub const READ_WRITE_EXECUTE: MemoryPermission = MemoryPermission::new(true, true, true);
}

impl From<MemoryPermission> for hv_memory_flags_t {
    fn from(value: MemoryPermission) -> hv_memory_flags_t {
        let mut result = 0;

        if value.read {
            result |= HV_MEMORY_READ;
        }

        if value.write {
            result |= HV_MEMORY_WRITE;
        }

        if value.execute {
            result |= HV_MEMORY_EXEC;
        }

        result
    }
}

/// Represent a memory mapping of a Virtual Machine.
#[derive(Copy, Clone, Debug)]
pub struct VirtualMachineMapping {
    /// The allcation handle associated to this mapping.
    pub allocation_handle: AllocationHandle,

    /// The handle associated to this mapping.
    pub mapping_handle: MappingHandle,

    /// The guess address of the region.
    pub address: hv_ipa_t,

    /// The size of the region.
    pub size: usize,

    /// The memory permission associated with the region.
    pub permission: MemoryPermission,
}

/// Represent an handle to an allocation.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AllocationHandle(pub u64);

/// Represent an handle to a mapping.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MappingHandle(pub u64);

/// An utility to manipulate counters.
#[derive(Debug)]
struct Counter(u64);

impl Counter {
    /// Gets the next value on the counter
    pub fn get_next_value(&mut self) -> u64 {
        self.0 += 1;

        self.0
    }
}

impl Default for Counter {
    fn default() -> Counter {
        Counter(0)
    }
}

/// Represent a Virtual Machine allocation.
#[derive(Debug)]
struct VirtualMachineAllocation {
    /// The allocation base address.
    base_address: *mut u8,

    /// The layout of the allocation.
    layout: Layout,

    /// Associated handle.
    handle: AllocationHandle,
}

impl Drop for VirtualMachineAllocation {
    fn drop(&mut self) {
        unsafe {
            alloc::alloc::dealloc(self.base_address, self.layout);
        }
    }
}

/// The size of a page.
pub const PAGE_SIZE: usize = 0x10000;

impl VirtualMachineAllocation {
    /// Create a new allocation to use by the VirtualMachine.
    pub fn new(size: usize) -> Self {
        unsafe {
            let layout = Layout::from_size_align(size, PAGE_SIZE)
                .unwrap()
                .pad_to_align();

            VirtualMachineAllocation {
                base_address: alloc::alloc::alloc_zeroed(layout),
                layout,
                handle: AllocationHandle(0),
            }
        }
    }
}

/// Represent the instance of a Virtual Machine.
#[derive(Debug)]
pub struct VirtualMachine {
    /// Counter used for allocation identifier.
    allocation_counter: Counter,

    /// Counter used for mapping identifier.
    mapping_counter: Counter,

    /// List of all allocations.
    allocation_list: Vec<VirtualMachineAllocation>,

    /// List of all mappings.
    mapping_list: Vec<VirtualMachineMapping>,
}

impl VirtualMachine {
    /// Create a new Virtual Machine instance
    ///
    /// **There should be only one instance living in the same process.**
    pub fn new(config: Option<VirtualMachineConfiguration>) -> Result<Self> {
        let handle: hv_vm_config_t = config
            .map(|value| value.handle)
            .unwrap_or(core::ptr::null_mut());

        let ret = unsafe { hv_vm_create(handle) };

        convert_hv_return(ret).map(|_| VirtualMachine {
            allocation_counter: Counter::default(),
            mapping_counter: Counter::default(),
            allocation_list: Vec::new(),
            mapping_list: Vec::new(),
        })
    }

    /// Create a new allocation that can be used in the Virtual Machine.
    pub fn allocate(&mut self, size: usize) -> Result<AllocationHandle> {
        let mut allocation = VirtualMachineAllocation::new(size);

        let handle = AllocationHandle(self.allocation_counter.get_next_value());

        allocation.handle = handle;

        self.allocation_list.push(allocation);

        Ok(handle)
    }

    /// Create a new allocation from data that can be used in the Virtual Machine.
    pub fn allocate_from(&mut self, source: &[u8]) -> Result<AllocationHandle> {
        let allocation_handle = self.allocate(source.len())?;

        if let Ok(destination) = self.get_allocation_slice_mut(allocation_handle) {
            let destination = &mut destination[..source.len()];
            destination.copy_from_slice(source);

            Ok(allocation_handle)
        } else {
            Err(HypervisorError::NoResources)
        }
    }

    /// Find an allocation by handle.
    fn find_allocation_by_handle(
        &self,
        handle: AllocationHandle,
    ) -> Result<(usize, &VirtualMachineAllocation)> {
        for (index, entry) in self.allocation_list.iter().enumerate() {
            if entry.handle == handle {
                return Ok((index, entry));
            }
        }

        Err(HypervisorError::InvalidHandle)
    }

    /// Find an allocation by handle.
    fn find_mapping_by_handle(
        &self,
        handle: MappingHandle,
    ) -> Result<(usize, &VirtualMachineMapping)> {
        for (index, entry) in self.mapping_list.iter().enumerate() {
            if entry.mapping_handle == handle {
                return Ok((index, entry));
            }
        }

        Err(HypervisorError::InvalidHandle)
    }

    /// Check if the given allocation handle is mapped.
    fn is_allocation_mapped(&self, handle: AllocationHandle) -> bool {
        for (_, entry) in self.mapping_list.iter().enumerate() {
            if entry.allocation_handle == handle {
                return true;
            }
        }

        false
    }

    /// Destroy an allocation from the Virtual Machine.
    ///
    /// **All references to this allocation should be unmapped first**
    pub fn deallocate(&mut self, allocation_handle: AllocationHandle) -> Result<()> {
        let (index, _) = self.find_allocation_by_handle(allocation_handle)?;

        // Ensure it's not in use.
        if self.is_allocation_mapped(allocation_handle) {
            return Err(HypervisorError::AllocationStillMapped);
        }

        self.allocation_list.remove(index);

        Ok(())
    }

    /// Gets a slice to an allocation with its handle.
    pub fn get_allocation_slice(&self, allocation_handle: AllocationHandle) -> Result<&[u8]> {
        let (_, allocation) = self.find_allocation_by_handle(allocation_handle)?;

        let slice = unsafe {
            core::slice::from_raw_parts(allocation.base_address, allocation.layout.size())
        };

        Ok(slice)
    }

    /// Gets a mutable slice to an allocation with its handle.
    pub fn get_allocation_slice_mut(
        &mut self,
        allocation_handle: AllocationHandle,
    ) -> Result<&mut [u8]> {
        let (_, allocation) = self.find_allocation_by_handle(allocation_handle)?;

        let slice = unsafe {
            core::slice::from_raw_parts_mut(allocation.base_address, allocation.layout.size())
        };

        Ok(slice)
    }

    /// Map an allocation in the Virtual Machine.
    pub fn map(
        &mut self,
        allocation_handle: AllocationHandle,
        guest_address: hv_ipa_t,
        permission: MemoryPermission,
    ) -> Result<MappingHandle> {
        let (_, allocation) = self.find_allocation_by_handle(allocation_handle)?;

        let allocation_size = allocation.layout.size();

        let ret = unsafe {
            hv_vm_map(
                allocation.base_address as *mut c_void,
                guest_address,
                allocation_size,
                hv_memory_flags_t::from(permission),
            )
        };

        // Ensure no error got reported
        convert_hv_return(ret)?;

        let mapping_handle = MappingHandle(self.mapping_counter.get_next_value());

        let virtual_mapping = VirtualMachineMapping {
            allocation_handle,
            mapping_handle,
            address: guest_address,
            size: allocation_size,
            permission,
        };

        self.mapping_list.push(virtual_mapping);

        Ok(mapping_handle)
    }

    /// Unmap a given mapping in the Virtual Machine.
    pub fn unmap(&mut self, mapping_handle: MappingHandle) -> Result<()> {
        let (index, mapping) = self.find_mapping_by_handle(mapping_handle)?;

        let ret = unsafe { hv_vm_unmap(mapping.address, mapping.size) };

        // Ensure no error got reported
        convert_hv_return(ret)?;

        self.mapping_list.remove(index);

        Ok(())
    }

    /// Change memory permissions of a given mapping in the Virtual Machine.
    pub fn reprotect(
        &mut self,
        mapping_handle: MappingHandle,
        permission: MemoryPermission,
    ) -> Result<()> {
        let (index, mapping) = self.find_mapping_by_handle(mapping_handle)?;

        let ret = unsafe {
            hv_vm_protect(
                mapping.address,
                mapping.size,
                hv_memory_flags_t::from(permission),
            )
        };

        // Ensure no error got reported
        convert_hv_return(ret)?;

        let mut mapping = self
            .mapping_list
            .get_mut(index)
            .expect("Mapping disapeared in between! (TOUTOC????)");

        mapping.permission = permission;

        Ok(())
    }

    /// Create a new vCPU configuration.
    pub fn create_vcpu_configuration(&self) -> VirtualCpuConfiguration {
        VirtualCpuConfiguration::new()
    }

    /// Create a new vCPU.
    ///
    /// **This should be called in the thread that will run the vCPU as it's resident inside it.**
    pub fn create_vcpu(
        &mut self,
        config: Option<&mut VirtualCpuConfiguration>,
    ) -> Result<VirtualCpu> {
        let handle: hv_vcpu_config_t = config
            .map(|value| value.handle)
            .unwrap_or(core::ptr::null_mut());

        let mut vcpu_handle: hv_vcpu_t = 0;
        let mut vcpu_exit: *const hv_vcpu_exit_t = core::ptr::null_mut();

        let ret = unsafe { hv_vcpu_create(&mut vcpu_handle, &mut vcpu_exit, &handle) };

        convert_hv_return(ret).map(|_| VirtualCpu {
            _not_send_marker: PhantomData,
            handle: vcpu_handle,
            vcpu_exit,
        })
    }

    /// Exits given vCPUs.
    pub fn exit_vcpus(&mut self, vcpus: &[hv_vcpu_t]) -> Result<()> {
        let ret = unsafe { hv_vcpus_exit(vcpus.as_ptr(), vcpus.len() as u32) };

        convert_hv_return(ret)
    }

    /// Gets the information about a mapping from its handle.
    pub fn get_mapping_info(&self, mapping_handle: MappingHandle) -> Result<VirtualMachineMapping> {
        self.find_mapping_by_handle(mapping_handle)
            .map(|(_, value)| *value)
    }

    /// Get a list of all mapping informations.
    pub fn get_all_mapping_infos(&self) -> Vec<VirtualMachineMapping> {
        self.mapping_list.clone()
    }
}

impl Drop for VirtualMachine {
    fn drop(&mut self) {
        for mapping in self.get_all_mapping_infos() {
            self.unmap(mapping.mapping_handle)
                .expect("Cannot unmap memory on VM drop!");
        }

        let ret = unsafe { hv_vm_destroy() };

        convert_hv_return(ret).expect("Cannot destroy VM on drop!");
    }
}

/// Cache type.
#[derive(Copy, Clone, Debug)]
pub enum CacheType {
    /// Data cache.
    Data,

    /// Instruction cache.
    Instruction,
}

impl From<CacheType> for hv_cache_type_t {
    fn from(value: CacheType) -> hv_cache_type_t {
        match value {
            CacheType::Data => HV_CACHE_TYPE_DATA,
            CacheType::Instruction => HV_CACHE_TYPE_INSTRUCTION,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
/// Feature register.
pub enum FeatureRegister {
    /// ID_AA64DFR0_EL1 register.
    ID_AA64DFR0_EL1,

    /// ID_AA64DFR1_EL1 register.
    ID_AA64DFR1_EL1,

    /// ID_AA64ISAR0_EL1 register.
    ID_AA64ISAR0_EL1,

    /// ID_AA64ISAR1_EL1 register.
    ID_AA64ISAR1_EL1,

    /// ID_AA64MMFR0_EL1 register.
    ID_AA64MMFR0_EL1,

    /// ID_AA64MMFR1_EL1 register.
    ID_AA64MMFR1_EL1,

    /// ID_AA64MMFR2_EL1 register.
    ID_AA64MMFR2_EL1,

    /// ID_AA64PFR0_EL1 register.
    ID_AA64PFR0_EL1,

    /// ID_AA64PFR1_EL1 register.
    ID_AA64PFR1_EL1,

    /// CTR_EL0 register.
    CTR_EL0,

    /// CLIDR_EL1 register.
    CLIDR_EL1,

    /// DCZID_EL0 register.
    DCZID_EL0,
}

impl From<FeatureRegister> for hv_feature_reg_t {
    fn from(value: FeatureRegister) -> hv_feature_reg_t {
        match value {
            FeatureRegister::ID_AA64DFR0_EL1 => HV_FEATURE_REG_ID_AA64DFR0_EL1,
            FeatureRegister::ID_AA64DFR1_EL1 => HV_FEATURE_REG_ID_AA64DFR1_EL1,
            FeatureRegister::ID_AA64ISAR0_EL1 => HV_FEATURE_REG_ID_AA64ISAR0_EL1,
            FeatureRegister::ID_AA64ISAR1_EL1 => HV_FEATURE_REG_ID_AA64ISAR1_EL1,
            FeatureRegister::ID_AA64MMFR0_EL1 => HV_FEATURE_REG_ID_AA64MMFR0_EL1,
            FeatureRegister::ID_AA64MMFR1_EL1 => HV_FEATURE_REG_ID_AA64MMFR1_EL1,
            FeatureRegister::ID_AA64MMFR2_EL1 => HV_FEATURE_REG_ID_AA64MMFR2_EL1,
            FeatureRegister::ID_AA64PFR0_EL1 => HV_FEATURE_REG_ID_AA64PFR0_EL1,
            FeatureRegister::ID_AA64PFR1_EL1 => HV_FEATURE_REG_ID_AA64PFR1_EL1,
            FeatureRegister::CTR_EL0 => HV_FEATURE_REG_CTR_EL0,
            FeatureRegister::CLIDR_EL1 => HV_FEATURE_REG_CLIDR_EL1,
            FeatureRegister::DCZID_EL0 => HV_FEATURE_REG_DCZID_EL0,
        }
    }
}

/// vCPU configuration for a Virtual Machine.
#[derive(Debug)]
pub struct VirtualCpuConfiguration {
    /// Handle of the vCPU configuration.
    handle: hv_vcpu_config_t,
}

impl VirtualCpuConfiguration {
    /// Create a new vCPU configuration.
    fn new() -> Self {
        VirtualCpuConfiguration {
            handle: unsafe { hv_vcpu_config_create() },
        }
    }

    /// Return value of a feature register.
    pub fn get_feature_register(&self, feature_register: FeatureRegister) -> Result<u64> {
        let mut result = 0;

        let ret = unsafe {
            hv_vcpu_config_get_feature_reg(
                self.handle,
                hv_feature_reg_t::from(feature_register),
                &mut result as *mut u64,
            )
        };

        // Ensure no error got reported
        convert_hv_return(ret)?;

        Ok(result)
    }

    /// Return values of CCSIDR_EL1 for a given cache type.
    pub fn get_ccsidr_el1_sys_register_values(&self, cache_type: CacheType) -> Result<[u64; 8]> {
        let mut result = [0x0; 8];

        let ret = unsafe {
            hv_vcpu_config_get_ccsidr_el1_sys_reg_values(
                self.handle,
                hv_cache_type_t::from(cache_type),
                &mut result[0] as *mut u64,
            )
        };

        // Ensure no error got reported
        convert_hv_return(ret)?;

        Ok(result)
    }
}

extern "C" {
    fn os_release(object: *mut c_void);
}

impl Drop for VirtualCpuConfiguration {
    fn drop(&mut self) {
        unsafe {
            os_release(self.handle);
        }
    }
}

/// ARM register.
#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum Register {
    /// X0 register.
    X0,

    /// X1 register.
    X1,

    /// X2 register.
    X2,

    /// X3 register.
    X3,

    /// X4 register.
    X4,

    /// X5 register.
    X5,

    /// X6 register.
    X6,

    /// X7 register.
    X7,

    /// X8 register.
    X8,

    /// X9 register.
    X9,

    /// X10 register.
    X10,

    /// X11 register.
    X11,

    /// X12 register.
    X12,

    /// X13 register.
    X13,

    /// X14 register.
    X14,

    /// X15 register.
    X15,

    /// X16 register.
    X16,

    /// X17 register.
    X17,

    /// X18 register.
    X18,

    /// X19 register.
    X19,

    /// X20 register.
    X20,

    /// X21 register.
    X21,

    /// X22 register.
    X22,

    /// X23 register.
    X23,

    /// X24 register.
    X24,

    /// X25 register.
    X25,

    /// X26 register.
    X26,

    /// X27 register.
    X27,

    /// X28 register.
    X28,

    /// X29 register.
    X29,

    /// FP register.
    FP,

    /// X30 register.
    X30,

    /// LR register.
    LR,

    /// PC register.
    PC,

    /// FPCR register.
    FPCR,

    /// FPSR register.
    FPSR,

    /// CPSR register.
    CPSR,
}

impl From<Register> for hv_reg_t {
    fn from(value: Register) -> hv_reg_t {
        match value {
            Register::X0 => HV_REG_X0,
            Register::X1 => HV_REG_X1,
            Register::X2 => HV_REG_X2,
            Register::X3 => HV_REG_X3,
            Register::X4 => HV_REG_X4,
            Register::X5 => HV_REG_X5,
            Register::X6 => HV_REG_X6,
            Register::X7 => HV_REG_X7,
            Register::X8 => HV_REG_X8,
            Register::X9 => HV_REG_X9,
            Register::X10 => HV_REG_X10,
            Register::X11 => HV_REG_X11,
            Register::X12 => HV_REG_X12,
            Register::X13 => HV_REG_X13,
            Register::X14 => HV_REG_X14,
            Register::X15 => HV_REG_X15,
            Register::X16 => HV_REG_X16,
            Register::X17 => HV_REG_X17,
            Register::X18 => HV_REG_X18,
            Register::X19 => HV_REG_X19,
            Register::X20 => HV_REG_X20,
            Register::X21 => HV_REG_X21,
            Register::X22 => HV_REG_X22,
            Register::X23 => HV_REG_X23,
            Register::X24 => HV_REG_X24,
            Register::X25 => HV_REG_X25,
            Register::X26 => HV_REG_X26,
            Register::X27 => HV_REG_X27,
            Register::X28 => HV_REG_X28,
            Register::X29 => HV_REG_X29,
            Register::X30 => HV_REG_X30,
            Register::FP => HV_REG_FP,
            Register::LR => HV_REG_LR,
            Register::PC => HV_REG_PC,
            Register::FPCR => HV_REG_FPCR,
            Register::FPSR => HV_REG_FPSR,
            Register::CPSR => HV_REG_CPSR,
        }
    }
}

/// ARM system register.
#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum SystemRegister {
    /// DBGBVR0_EL1 register.
    DBGBVR0_EL1,

    /// DBGBCR0_EL1 register.
    DBGBCR0_EL1,

    /// DBGWVR0_EL1 register.
    DBGWVR0_EL1,

    /// DBGWCR0_EL1 register.
    DBGWCR0_EL1,

    /// DBGBVR1_EL1 register.
    DBGBVR1_EL1,

    /// DBGBCR1_EL1 register.
    DBGBCR1_EL1,

    /// DBGWVR1_EL1 register.
    DBGWVR1_EL1,

    /// DBGWCR1_EL1 register.
    DBGWCR1_EL1,

    /// MDCCINT_EL1 register.
    MDCCINT_EL1,

    /// MDSCR_EL1 register.
    MDSCR_EL1,

    /// DBGBVR2_EL1 register.
    DBGBVR2_EL1,

    /// DBGBCR2_EL1 register.
    DBGBCR2_EL1,

    /// DBGWVR2_EL1 register.
    DBGWVR2_EL1,

    /// DBGWCR2_EL1 register.
    DBGWCR2_EL1,

    /// DBGBVR3_EL1 register.
    DBGBVR3_EL1,

    /// DBGBCR3_EL1 register.
    DBGBCR3_EL1,

    /// DBGWVR3_EL1 register.
    DBGWVR3_EL1,

    /// DBGWCR3_EL1 register.
    DBGWCR3_EL1,

    /// DBGBVR4_EL1 register.
    DBGBVR4_EL1,

    /// DBGBCR4_EL1 register.
    DBGBCR4_EL1,

    /// DBGWVR4_EL1 register.
    DBGWVR4_EL1,

    /// DBGWCR4_EL1 register.
    DBGWCR4_EL1,

    /// DBGBVR5_EL1 register.
    DBGBVR5_EL1,

    /// DBGBCR5_EL1 register.
    DBGBCR5_EL1,

    /// DBGWVR5_EL1 register.
    DBGWVR5_EL1,

    /// DBGWCR5_EL1 register.
    DBGWCR5_EL1,

    /// DBGBVR6_EL1 register.
    DBGBVR6_EL1,

    /// DBGBCR6_EL1 register.
    DBGBCR6_EL1,

    /// DBGWVR6_EL1 register.
    DBGWVR6_EL1,

    /// DBGWCR6_EL1 register.
    DBGWCR6_EL1,

    /// DBGBVR7_EL1 register.
    DBGBVR7_EL1,

    /// DBGBCR7_EL1 register.
    DBGBCR7_EL1,

    /// DBGWVR7_EL1 register.
    DBGWVR7_EL1,

    /// DBGWCR7_EL1 register.
    DBGWCR7_EL1,

    /// DBGBVR8_EL1 register.
    DBGBVR8_EL1,

    /// DBGBCR8_EL1 register.
    DBGBCR8_EL1,

    /// DBGWVR8_EL1 register.
    DBGWVR8_EL1,

    /// DBGWCR8_EL1 register.
    DBGWCR8_EL1,

    /// DBGBVR9_EL1 register.
    DBGBVR9_EL1,

    /// DBGBCR9_EL1 register.
    DBGBCR9_EL1,

    /// DBGWVR9_EL1 register.
    DBGWVR9_EL1,

    /// DBGWCR9_EL1 register.
    DBGWCR9_EL1,

    /// DBGBVR10_EL1 register.
    DBGBVR10_EL1,

    /// DBGBCR10_EL1 register.
    DBGBCR10_EL1,

    /// DBGWVR10_EL1 register.
    DBGWVR10_EL1,

    /// DBGWCR10_EL1 register.
    DBGWCR10_EL1,

    /// DBGBVR11_EL1 register.
    DBGBVR11_EL1,

    /// DBGBCR11_EL1 register.
    DBGBCR11_EL1,

    /// DBGWVR11_EL1 register.
    DBGWVR11_EL1,

    /// DBGWCR11_EL1 register.
    DBGWCR11_EL1,

    /// DBGBVR12_EL1 register.
    DBGBVR12_EL1,

    /// DBGBCR12_EL1 register.
    DBGBCR12_EL1,

    /// DBGWVR12_EL1 register.
    DBGWVR12_EL1,

    /// DBGWCR12_EL1 register.
    DBGWCR12_EL1,

    /// DBGBVR13_EL1 register.
    DBGBVR13_EL1,

    /// DBGBCR13_EL1 register.
    DBGBCR13_EL1,

    /// DBGWVR13_EL1 register.
    DBGWVR13_EL1,

    /// DBGWCR13_EL1 register.
    DBGWCR13_EL1,

    /// DBGBVR14_EL1 register.
    DBGBVR14_EL1,

    /// DBGBCR14_EL1 register.
    DBGBCR14_EL1,

    /// DBGWVR14_EL1 register.
    DBGWVR14_EL1,

    /// DBGWCR14_EL1 register.
    DBGWCR14_EL1,

    /// DBGBVR15_EL1 register.
    DBGBVR15_EL1,

    /// DBGBCR15_EL1 register.
    DBGBCR15_EL1,

    /// DBGWVR15_EL1 register.
    DBGWVR15_EL1,

    /// DBGWCR15_EL1 register.
    DBGWCR15_EL1,

    /// MIDR_EL1 register.
    MIDR_EL1,

    /// MPIDR_EL1 register.
    MPIDR_EL1,

    /// ID_AA64PFR0_EL1 register.
    ID_AA64PFR0_EL1,

    /// ID_AA64PFR1_EL1 register.
    ID_AA64PFR1_EL1,

    /// ID_AA64DFR0_EL1 register.
    ID_AA64DFR0_EL1,

    /// ID_AA64DFR1_EL1 register.
    ID_AA64DFR1_EL1,

    /// ID_AA64ISAR0_EL1 register.
    ID_AA64ISAR0_EL1,

    /// ID_AA64ISAR1_EL1 register.
    ID_AA64ISAR1_EL1,

    /// AA64MMFR0_EL1 register.
    ID_AA64MMFR0_EL1,

    /// ID_AA64MMFR1_EL1 register.
    ID_AA64MMFR1_EL1,

    /// AA64MMFR2_EL1 register.
    ID_AA64MMFR2_EL1,

    /// SCTLR_EL1 register.
    SCTLR_EL1,

    /// CPACR_EL1 register.
    CPACR_EL1,

    /// TTBR0_EL1 register.
    TTBR0_EL1,

    /// TTBR1_EL1 register.
    TTBR1_EL1,

    /// TCR_EL1 register.
    TCR_EL1,

    /// APIAKEYLO_EL1 register.
    APIAKEYLO_EL1,

    /// APIAKEYHI_EL1 register.
    APIAKEYHI_EL1,

    /// APIBKEYLO_EL1 register.
    APIBKEYLO_EL1,

    /// APIBKEYHI_EL1 register.
    APIBKEYHI_EL1,

    /// APDAKEYLO_EL1 register.
    APDAKEYLO_EL1,

    /// APDAKEYHI_EL1 register.
    APDAKEYHI_EL1,

    /// APDBKEYLO_EL1 register.
    APDBKEYLO_EL1,

    /// APDBKEYHI_EL1 register.
    APDBKEYHI_EL1,

    /// APGAKEYLO_EL1 register.
    APGAKEYLO_EL1,

    /// APGAKEYHI_EL1 register.
    APGAKEYHI_EL1,

    /// SPSR_EL1 register.
    SPSR_EL1,

    /// ELR_EL1 register.
    ELR_EL1,

    /// SP_EL0 register.
    SP_EL0,

    /// AFSR0_EL1 register.
    AFSR0_EL1,

    /// AFSR1_EL1 register.
    AFSR1_EL1,

    /// ESR_EL1 register.
    ESR_EL1,

    /// FAR_EL1 register.
    FAR_EL1,

    /// PAR_EL1 register.
    PAR_EL1,

    /// MAIR_EL1 register.
    MAIR_EL1,

    /// AMAIR_EL1 register.
    AMAIR_EL1,

    /// VBAR_EL1 register.
    VBAR_EL1,

    /// CONTEXTIDR_EL1 register.
    CONTEXTIDR_EL1,

    /// TPIDR_EL1 register.
    TPIDR_EL1,

    /// CNTKCTL_EL1 register.
    CNTKCTL_EL1,

    /// CSSELR_EL1 register.
    CSSELR_EL1,

    /// TPIDR_EL0 register.
    TPIDR_EL0,

    /// TPIDRRO_EL0 register.
    TPIDRRO_EL0,

    /// CNTV_CTL_EL0 register.
    CNTV_CTL_EL0,

    /// CNTV_CVAL_EL0 register.
    CNTV_CVAL_EL0,

    /// SP_EL1 register.
    SP_EL1,
}

impl From<SystemRegister> for hv_sys_reg_t {
    fn from(value: SystemRegister) -> hv_sys_reg_t {
        match value {
            SystemRegister::DBGBVR0_EL1 => HV_SYS_REG_DBGBVR0_EL1,
            SystemRegister::DBGBCR0_EL1 => HV_SYS_REG_DBGBCR0_EL1,
            SystemRegister::DBGWVR0_EL1 => HV_SYS_REG_DBGWVR0_EL1,
            SystemRegister::DBGWCR0_EL1 => HV_SYS_REG_DBGWCR0_EL1,
            SystemRegister::DBGBVR1_EL1 => HV_SYS_REG_DBGBVR1_EL1,
            SystemRegister::DBGBCR1_EL1 => HV_SYS_REG_DBGBCR1_EL1,
            SystemRegister::DBGWVR1_EL1 => HV_SYS_REG_DBGWVR1_EL1,
            SystemRegister::DBGWCR1_EL1 => HV_SYS_REG_DBGWCR1_EL1,
            SystemRegister::MDCCINT_EL1 => HV_SYS_REG_MDCCINT_EL1,
            SystemRegister::MDSCR_EL1 => HV_SYS_REG_MDSCR_EL1,
            SystemRegister::DBGBVR2_EL1 => HV_SYS_REG_DBGBVR2_EL1,
            SystemRegister::DBGBCR2_EL1 => HV_SYS_REG_DBGBCR2_EL1,
            SystemRegister::DBGWVR2_EL1 => HV_SYS_REG_DBGWVR2_EL1,
            SystemRegister::DBGWCR2_EL1 => HV_SYS_REG_DBGWCR2_EL1,
            SystemRegister::DBGBVR3_EL1 => HV_SYS_REG_DBGBVR3_EL1,
            SystemRegister::DBGBCR3_EL1 => HV_SYS_REG_DBGBCR3_EL1,
            SystemRegister::DBGWVR3_EL1 => HV_SYS_REG_DBGWVR3_EL1,
            SystemRegister::DBGWCR3_EL1 => HV_SYS_REG_DBGWCR3_EL1,
            SystemRegister::DBGBVR4_EL1 => HV_SYS_REG_DBGBVR4_EL1,
            SystemRegister::DBGBCR4_EL1 => HV_SYS_REG_DBGBCR4_EL1,
            SystemRegister::DBGWVR4_EL1 => HV_SYS_REG_DBGWVR4_EL1,
            SystemRegister::DBGWCR4_EL1 => HV_SYS_REG_DBGWCR4_EL1,
            SystemRegister::DBGBVR5_EL1 => HV_SYS_REG_DBGBVR5_EL1,
            SystemRegister::DBGBCR5_EL1 => HV_SYS_REG_DBGBCR5_EL1,
            SystemRegister::DBGWVR5_EL1 => HV_SYS_REG_DBGWVR5_EL1,
            SystemRegister::DBGWCR5_EL1 => HV_SYS_REG_DBGWCR5_EL1,
            SystemRegister::DBGBVR6_EL1 => HV_SYS_REG_DBGBVR6_EL1,
            SystemRegister::DBGBCR6_EL1 => HV_SYS_REG_DBGBCR6_EL1,
            SystemRegister::DBGWVR6_EL1 => HV_SYS_REG_DBGWVR6_EL1,
            SystemRegister::DBGWCR6_EL1 => HV_SYS_REG_DBGWCR6_EL1,
            SystemRegister::DBGBVR7_EL1 => HV_SYS_REG_DBGBVR7_EL1,
            SystemRegister::DBGBCR7_EL1 => HV_SYS_REG_DBGBCR7_EL1,
            SystemRegister::DBGWVR7_EL1 => HV_SYS_REG_DBGWVR7_EL1,
            SystemRegister::DBGWCR7_EL1 => HV_SYS_REG_DBGWCR7_EL1,
            SystemRegister::DBGBVR8_EL1 => HV_SYS_REG_DBGBVR8_EL1,
            SystemRegister::DBGBCR8_EL1 => HV_SYS_REG_DBGBCR8_EL1,
            SystemRegister::DBGWVR8_EL1 => HV_SYS_REG_DBGWVR8_EL1,
            SystemRegister::DBGWCR8_EL1 => HV_SYS_REG_DBGWCR8_EL1,
            SystemRegister::DBGBVR9_EL1 => HV_SYS_REG_DBGBVR9_EL1,
            SystemRegister::DBGBCR9_EL1 => HV_SYS_REG_DBGBCR9_EL1,
            SystemRegister::DBGWVR9_EL1 => HV_SYS_REG_DBGWVR9_EL1,
            SystemRegister::DBGWCR9_EL1 => HV_SYS_REG_DBGWCR9_EL1,
            SystemRegister::DBGBVR10_EL1 => HV_SYS_REG_DBGBVR10_EL1,
            SystemRegister::DBGBCR10_EL1 => HV_SYS_REG_DBGBCR10_EL1,
            SystemRegister::DBGWVR10_EL1 => HV_SYS_REG_DBGWVR10_EL1,
            SystemRegister::DBGWCR10_EL1 => HV_SYS_REG_DBGWCR10_EL1,
            SystemRegister::DBGBVR11_EL1 => HV_SYS_REG_DBGBVR11_EL1,
            SystemRegister::DBGBCR11_EL1 => HV_SYS_REG_DBGBCR11_EL1,
            SystemRegister::DBGWVR11_EL1 => HV_SYS_REG_DBGWVR11_EL1,
            SystemRegister::DBGWCR11_EL1 => HV_SYS_REG_DBGWCR11_EL1,
            SystemRegister::DBGBVR12_EL1 => HV_SYS_REG_DBGBVR12_EL1,
            SystemRegister::DBGBCR12_EL1 => HV_SYS_REG_DBGBCR12_EL1,
            SystemRegister::DBGWVR12_EL1 => HV_SYS_REG_DBGWVR12_EL1,
            SystemRegister::DBGWCR12_EL1 => HV_SYS_REG_DBGWCR12_EL1,
            SystemRegister::DBGBVR13_EL1 => HV_SYS_REG_DBGBVR13_EL1,
            SystemRegister::DBGBCR13_EL1 => HV_SYS_REG_DBGBCR13_EL1,
            SystemRegister::DBGWVR13_EL1 => HV_SYS_REG_DBGWVR13_EL1,
            SystemRegister::DBGWCR13_EL1 => HV_SYS_REG_DBGWCR13_EL1,
            SystemRegister::DBGBVR14_EL1 => HV_SYS_REG_DBGBVR14_EL1,
            SystemRegister::DBGBCR14_EL1 => HV_SYS_REG_DBGBCR14_EL1,
            SystemRegister::DBGWVR14_EL1 => HV_SYS_REG_DBGWVR14_EL1,
            SystemRegister::DBGWCR14_EL1 => HV_SYS_REG_DBGWCR14_EL1,
            SystemRegister::DBGBVR15_EL1 => HV_SYS_REG_DBGBVR15_EL1,
            SystemRegister::DBGBCR15_EL1 => HV_SYS_REG_DBGBCR15_EL1,
            SystemRegister::DBGWVR15_EL1 => HV_SYS_REG_DBGWVR15_EL1,
            SystemRegister::DBGWCR15_EL1 => HV_SYS_REG_DBGWCR15_EL1,
            SystemRegister::MIDR_EL1 => HV_SYS_REG_MIDR_EL1,
            SystemRegister::MPIDR_EL1 => HV_SYS_REG_MPIDR_EL1,
            SystemRegister::ID_AA64PFR0_EL1 => HV_SYS_REG_ID_AA64PFR0_EL1,
            SystemRegister::ID_AA64PFR1_EL1 => HV_SYS_REG_ID_AA64PFR1_EL1,
            SystemRegister::ID_AA64DFR0_EL1 => HV_SYS_REG_ID_AA64DFR0_EL1,
            SystemRegister::ID_AA64DFR1_EL1 => HV_SYS_REG_ID_AA64DFR1_EL1,
            SystemRegister::ID_AA64ISAR0_EL1 => HV_SYS_REG_ID_AA64ISAR0_EL1,
            SystemRegister::ID_AA64ISAR1_EL1 => HV_SYS_REG_ID_AA64ISAR1_EL1,
            SystemRegister::ID_AA64MMFR0_EL1 => HV_SYS_REG_ID_AA64MMFR0_EL1,
            SystemRegister::ID_AA64MMFR1_EL1 => HV_SYS_REG_ID_AA64MMFR1_EL1,
            SystemRegister::ID_AA64MMFR2_EL1 => HV_SYS_REG_ID_AA64MMFR2_EL1,
            SystemRegister::SCTLR_EL1 => HV_SYS_REG_SCTLR_EL1,
            SystemRegister::CPACR_EL1 => HV_SYS_REG_CPACR_EL1,
            SystemRegister::TTBR0_EL1 => HV_SYS_REG_TTBR0_EL1,
            SystemRegister::TTBR1_EL1 => HV_SYS_REG_TTBR1_EL1,
            SystemRegister::TCR_EL1 => HV_SYS_REG_TCR_EL1,
            SystemRegister::APIAKEYLO_EL1 => HV_SYS_REG_APIAKEYLO_EL1,
            SystemRegister::APIAKEYHI_EL1 => HV_SYS_REG_APIAKEYHI_EL1,
            SystemRegister::APIBKEYLO_EL1 => HV_SYS_REG_APIBKEYLO_EL1,
            SystemRegister::APIBKEYHI_EL1 => HV_SYS_REG_APIBKEYHI_EL1,
            SystemRegister::APDAKEYLO_EL1 => HV_SYS_REG_APDAKEYLO_EL1,
            SystemRegister::APDAKEYHI_EL1 => HV_SYS_REG_APDAKEYHI_EL1,
            SystemRegister::APDBKEYLO_EL1 => HV_SYS_REG_APDBKEYLO_EL1,
            SystemRegister::APDBKEYHI_EL1 => HV_SYS_REG_APDBKEYHI_EL1,
            SystemRegister::APGAKEYLO_EL1 => HV_SYS_REG_APGAKEYLO_EL1,
            SystemRegister::APGAKEYHI_EL1 => HV_SYS_REG_APGAKEYHI_EL1,
            SystemRegister::SPSR_EL1 => HV_SYS_REG_SPSR_EL1,
            SystemRegister::ELR_EL1 => HV_SYS_REG_ELR_EL1,
            SystemRegister::SP_EL0 => HV_SYS_REG_SP_EL0,
            SystemRegister::AFSR0_EL1 => HV_SYS_REG_AFSR0_EL1,
            SystemRegister::AFSR1_EL1 => HV_SYS_REG_AFSR1_EL1,
            SystemRegister::ESR_EL1 => HV_SYS_REG_ESR_EL1,
            SystemRegister::FAR_EL1 => HV_SYS_REG_FAR_EL1,
            SystemRegister::PAR_EL1 => HV_SYS_REG_PAR_EL1,
            SystemRegister::MAIR_EL1 => HV_SYS_REG_MAIR_EL1,
            SystemRegister::AMAIR_EL1 => HV_SYS_REG_AMAIR_EL1,
            SystemRegister::VBAR_EL1 => HV_SYS_REG_VBAR_EL1,
            SystemRegister::CONTEXTIDR_EL1 => HV_SYS_REG_CONTEXTIDR_EL1,
            SystemRegister::TPIDR_EL1 => HV_SYS_REG_TPIDR_EL1,
            SystemRegister::CNTKCTL_EL1 => HV_SYS_REG_CNTKCTL_EL1,
            SystemRegister::CSSELR_EL1 => HV_SYS_REG_CSSELR_EL1,
            SystemRegister::TPIDR_EL0 => HV_SYS_REG_TPIDR_EL0,
            SystemRegister::TPIDRRO_EL0 => HV_SYS_REG_TPIDRRO_EL0,
            SystemRegister::CNTV_CTL_EL0 => HV_SYS_REG_CNTV_CTL_EL0,
            SystemRegister::CNTV_CVAL_EL0 => HV_SYS_REG_CNTV_CVAL_EL0,
            SystemRegister::SP_EL1 => HV_SYS_REG_SP_EL1,
        }
    }
}

/// ARM interrupt type.
#[derive(Copy, Clone, Debug)]
pub enum InterruptType {
    /// ARM IRQ.
    IRQ,

    /// ARM FIQ.
    FIQ,
}

impl From<InterruptType> for hv_interrupt_type_t {
    fn from(value: InterruptType) -> hv_interrupt_type_t {
        match value {
            InterruptType::IRQ => HV_INTERRUPT_TYPE_IRQ,
            InterruptType::FIQ => HV_INTERRUPT_TYPE_FIQ,
        }
    }
}

#[derive(Copy, Clone, Debug)]
/// Exit reason of a vCPU.
pub enum VirtualCpuExitReason {
    /// Asynchronous exit.
    Cancelled,

    /// Guest exception.
    Exception {
        /// The informations about the guest exception.
        exception: hv_vcpu_exit_exception_t,
    },

    /// Virtual Timer enters the pending state.
    VTimerActivated,

    /// Unexpected exit.
    Unknown,
}

impl From<hv_vcpu_exit_t> for VirtualCpuExitReason {
    fn from(value: hv_vcpu_exit_t) -> VirtualCpuExitReason {
        match value.reason {
            HV_EXIT_REASON_CANCELED => VirtualCpuExitReason::Cancelled,
            HV_EXIT_REASON_EXCEPTION => VirtualCpuExitReason::Exception {
                exception: value.exception,
            },
            HV_EXIT_REASON_VTIMER_ACTIVATED => VirtualCpuExitReason::VTimerActivated,
            HV_EXIT_REASON_UNKNOWN => VirtualCpuExitReason::Unknown,

            // Unexpected unknown
            _ => VirtualCpuExitReason::Unknown,
        }
    }
}

/// vCPU for a Virtual Machine.
#[derive(Debug)]
pub struct VirtualCpu {
    /// A vCPU is resident to a specific thread. Therefore, it shouldn't be Send.
    _not_send_marker: PhantomData<*const u8>,

    /// Handle of the vCPU configuration.
    handle: hv_vcpu_t,

    /// vCPU exit informations.
    vcpu_exit: *const hv_vcpu_exit_t,
}

impl Drop for VirtualCpu {
    fn drop(&mut self) {
        self.exit().expect("Cannot exit vCPU on drop!");

        let ret = unsafe { hv_vcpu_destroy(self.handle) };

        convert_hv_return(ret).expect("Cannot destroy vCPU on drop!")
    }
}

impl VirtualCpu {
    /// Gets vCPU handle.
    pub fn get_handle(&self) -> hv_vcpu_t {
        self.handle
    }

    /// Gets a register value.
    ///
    /// **This should be called in the thread that will run the vCPU as it's resident inside it.**
    pub fn get_register(&mut self, register: Register) -> Result<u64> {
        let mut result = 0;

        let ret = unsafe {
            hv_vcpu_get_reg(
                self.handle,
                hv_reg_t::from(register),
                &mut result as *mut u64,
            )
        };

        // Ensure no error got reported
        convert_hv_return(ret)?;

        Ok(result)
    }

    /// Sets a register value.
    ///
    /// **This should be called in the thread that will run the vCPU as it's resident inside it.**
    pub fn set_register(&mut self, register: Register, value: u64) -> Result<()> {
        let ret = unsafe { hv_vcpu_set_reg(self.handle, hv_reg_t::from(register), value) };

        convert_hv_return(ret)
    }

    // TODO: SIMD APIs

    /// Gets a system register value.
    ///
    /// **This should be called in the thread that will run the vCPU as it's resident inside it.**
    pub fn get_system_register(&mut self, register: SystemRegister) -> Result<u64> {
        let mut result = 0;

        let ret = unsafe {
            hv_vcpu_get_sys_reg(
                self.handle,
                hv_sys_reg_t::from(register),
                &mut result as *mut u64,
            )
        };

        // Ensure no error got reported
        convert_hv_return(ret)?;

        Ok(result)
    }

    /// Sets a system register value.
    ///
    /// **This should be called in the thread that will run the vCPU as it's resident inside it.**
    pub fn set_system_register(&mut self, register: SystemRegister, value: u64) -> Result<()> {
        let ret = unsafe { hv_vcpu_set_sys_reg(self.handle, hv_sys_reg_t::from(register), value) };

        convert_hv_return(ret)
    }

    /// Gets pending interrupts.
    ///
    /// **This should be called in the thread that will run the vCPU as it's resident inside it.**
    pub fn get_pending_interrupt(&mut self, interrupt_type: InterruptType) -> Result<bool> {
        let mut result = false;

        let ret = unsafe {
            hv_vcpu_get_pending_interrupt(
                self.handle,
                hv_interrupt_type_t::from(interrupt_type),
                &mut result,
            )
        };

        convert_hv_return(ret)?;

        Ok(result)
    }

    /// Sets pending interrupts.
    ///
    /// **This should be called in the thread that will run the vCPU as it's resident inside it.**
    /// **Pending interrupts automatically get cleared after vCPU run and must be resetup before every call to run.**
    pub fn set_pending_interrupt(
        &mut self,
        interrupt_type: InterruptType,
        value: bool,
    ) -> Result<()> {
        let ret = unsafe {
            hv_vcpu_set_pending_interrupt(
                self.handle,
                hv_interrupt_type_t::from(interrupt_type),
                value,
            )
        };

        convert_hv_return(ret)
    }

    /// Gets whether debug exceptions exit the vCPU.
    ///
    /// **This should be called in the thread that will run the vCPU as it's resident inside it.**
    pub fn get_trap_debug_exceptions(&mut self) -> Result<bool> {
        let mut result = false;

        let ret = unsafe { hv_vcpu_get_trap_debug_exceptions(self.handle, &mut result) };

        convert_hv_return(ret)?;

        Ok(result)
    }

    /// Gets whether debug exceptions exit the vCPU.
    ///
    /// **This should be called in the thread that will run the vCPU as it's resident inside it.**
    pub fn set_trap_debug_exceptions(&mut self, value: bool) -> Result<()> {
        let ret = unsafe { hv_vcpu_set_trap_debug_exceptions(self.handle, value) };

        convert_hv_return(ret)
    }

    /// Gets whether debug-register accesses exit the vCPU.
    ///
    /// **This should be called in the thread that will run the vCPU as it's resident inside it.**
    pub fn get_trap_debug_reg_accesses(&mut self) -> Result<bool> {
        let mut result = false;

        let ret = unsafe { hv_vcpu_get_trap_debug_reg_accesses(self.handle, &mut result) };

        convert_hv_return(ret)?;

        Ok(result)
    }

    /// Gets whether debug-register accesses exit the vCPU.
    ///
    /// **This should be called in the thread that will run the vCPU as it's resident inside it.**
    pub fn set_trap_debug_reg_accesses(&mut self, value: bool) -> Result<()> {
        let ret = unsafe { hv_vcpu_set_trap_debug_reg_accesses(self.handle, value) };

        convert_hv_return(ret)
    }

    /// Runs the vCPU.
    ///
    /// **This should be called in the thread that will run the vCPU as it's resident inside it.**
    pub fn run(&mut self) -> Result<VirtualCpuExitReason> {
        let ret = unsafe { hv_vcpu_run(self.handle) };

        convert_hv_return(ret)?;

        Ok(VirtualCpuExitReason::from(unsafe { *self.vcpu_exit }))
    }

    /// Forces exit the vCPU.
    pub fn exit(&mut self) -> Result<()> {
        let ret = unsafe { hv_vcpus_exit(&self.handle, 1) };

        convert_hv_return(ret)
    }

    /// Gets cumulative execution time of a vCPU in mach_absolute_time().
    ///
    /// **This should be called in the thread that will run the vCPU as it's resident inside it.**
    pub fn get_exec_time(&mut self) -> Result<u64> {
        let mut result = 0;

        let ret = unsafe { hv_vcpu_get_exec_time(self.handle, &mut result) };

        convert_hv_return(ret)?;

        Ok(result)
    }

    /// Gets Virtual Timer mask.
    pub fn get_vtimer_mask(&mut self) -> Result<bool> {
        let mut result = false;

        let ret = unsafe { hv_vcpu_get_vtimer_mask(self.handle, &mut result) };

        convert_hv_return(ret)?;

        Ok(result)
    }

    /// Sets Virtual Timer mask.
    pub fn set_vtimer_mask(&mut self, value: bool) -> Result<()> {
        let ret = unsafe { hv_vcpu_set_vtimer_mask(self.handle, value) };

        convert_hv_return(ret)
    }

    /// Gets Virtual Timer offset (CNTVOFF_EL2).
    pub fn get_vtimer_offset(&mut self) -> Result<u64> {
        let mut result = 0;

        let ret = unsafe { hv_vcpu_get_vtimer_offset(self.handle, &mut result) };

        convert_hv_return(ret)?;

        Ok(result)
    }

    /// Sets Virtual Timer offset (CNTVOFF_EL2).
    pub fn set_vtimer_offset(&mut self, value: u64) -> Result<()> {
        let ret = unsafe { hv_vcpu_set_vtimer_offset(self.handle, value) };

        convert_hv_return(ret)
    }
}
