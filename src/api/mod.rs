//! Safe API for Apple Hypervisor
use crate::ffi::types::*;
use crate::ffi::*;

use core::ffi::c_void;

use alloc::alloc::Layout;
use alloc::vec::Vec;

/// An Hypervisor Result.
pub type Result<T> = core::result::Result<T, HypervisorError>;

/// Represent an error returned by the Hypervisor.
#[derive(Debug)]
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
#[derive(Copy, Clone, Debug)]
pub struct AllocationHandle(pub u64);

/// Represent an handle to a mapping.
#[derive(Copy, Clone, Debug)]
pub struct MappingHandle(pub u64);

/// An utility to manipulate counters.
#[derive(Debug)]
struct Counter(u64);

impl Counter {
    /// Get the next value on the counter
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
            // TODO: PartialEq on AllocationHandle.
            if entry.handle.0 == handle.0 {
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
            // TODO: PartialEq on MappingHandle.
            if entry.mapping_handle.0 == handle.0 {
                return Ok((index, entry));
            }
        }

        Err(HypervisorError::InvalidHandle)
    }

    /// Check if the given allocation handle is mapped.
    fn is_allocation_mapped(&self, handle: AllocationHandle) -> bool {
        for (_, entry) in self.mapping_list.iter().enumerate() {
            // TODO: PartialEq on MappingHandle.
            if entry.allocation_handle.0 == handle.0 {
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

    /// Get a slice to an allocation with its handle.
    pub fn get_allocation_slice(&self, allocation_handle: AllocationHandle) -> Result<&[u8]> {
        let (_, allocation) = self.find_allocation_by_handle(allocation_handle)?;

        let slice = unsafe {
            core::slice::from_raw_parts(allocation.base_address, allocation.layout.size())
        };

        Ok(slice)
    }

    /// Get a mutable slice to an allocation with its handle.
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

    /// Get the information about a mapping from its handle.
    pub fn get_mapping_info(&self, mapping_handle: MappingHandle) -> Result<VirtualMachineMapping> {
        self.find_mapping_by_handle(mapping_handle)
            .map(|(_, value)| *value)
    }
}

impl Drop for VirtualMachine {
    fn drop(&mut self) {
        unsafe {
            // TODO: ummap everything

            // TODO: should we assert here on error? maybe only on debug?
            hv_vm_destroy();
        }
    }
}
