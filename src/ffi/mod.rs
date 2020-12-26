//! Bindings to the Hypervisor Framework for arm64 targets.

use std::ffi::c_void;

pub mod types;

use types::*;

extern "C" {

    // VM APIs

    /// Creates a VM instance for the current process.
    pub fn hv_vm_create(config: hv_vm_config_t) -> hv_return_t;

    /// Destroys the VM instance associated with the current process.
    pub fn hv_vm_destroy() -> hv_return_t;

    /// Maps a region in the virtual address space of the current process into the guest physical address space of the VM.
    pub fn hv_vm_map(address: *mut c_void, ipa: hv_ipa_t, size: usize, flags: hv_memory_flags_t) -> hv_return_t;

    /// Unmaps a region in the guest physical address space of the VM.
    pub fn hv_vm_unmap(ipa: hv_ipa_t, size: usize) -> hv_return_t;

    /// Modifies the permissions of a region in the guest physical address space of the VM.
    pub fn hv_vm_protect(ipa: hv_ipa_t, size: usize, flags: hv_memory_flags_t) -> hv_return_t;

    // vCPU configuration APIs

    /// Creates a vCPU configuration.
    pub fn hv_vcpu_config_create() -> hv_vcpu_config_t;

    /// Gets the value of a feature register.
    pub fn hv_vcpu_config_get_feature_reg(config: hv_vcpu_config_t, feature_register: hv_feature_reg_t, value: *mut u64) -> hv_return_t;

    /// Return the given CCSIDR_EL1 for the given cache type.
    pub fn hv_vcpu_config_get_ccsidr_el1_sys_reg_values(config: hv_vcpu_config_t, cache_type: hv_cache_type_t, values: *mut u64) -> hv_return_t;

    // vCPU APIs

    /// Creates a vCPU instance for the current thread.
    pub fn hv_vcpu_create(vcpu: *mut hv_vcpu_t, exit: *mut *mut hv_vcpu_exit_t, config: *const hv_vcpu_config_t) -> hv_return_t;

    /// Destroys the vCPU instance associated with the current thread.
    pub fn hv_vcpu_destroy(vcpu: hv_vcpu_t) -> hv_return_t;

    /// Gets the current value of a vCPU register.
    pub fn hv_vcpu_get_reg(vcpu: hv_vcpu_t, reg: hv_reg_t, value: *mut u64) -> hv_return_t;

    /// Sets the value of a vCPU register.
    pub fn hv_vcpu_set_reg(vcpu: hv_vcpu_t, reg: hv_reg_t, value: u64) -> hv_return_t;

    //pub fn hv_vcpu_get_simd_fp_reg(vcpu: hv_vcpu_t, reg: hv_simd_fp_reg_t, value: *mut hv_simd_fp_uchar16_t) -> hv_return_t;
    //pub fn hv_vcpu_set_simd_fp_reg(vcpu: hv_vcpu_t, reg: hv_simd_fp_reg_t, value: hv_simd_fp_uchar16_t) -> hv_return_t;

    /// Gets the current value of a vCPU system register.
    pub fn hv_vcpu_get_sys_reg(vcpu: hv_vcpu_t, reg: hv_sys_reg_t, value: *mut u64) -> hv_return_t;

    /// Sets the value of a vCPU system register.
    pub fn hv_vcpu_set_sys_reg(vcpu: hv_vcpu_t, reg: hv_sys_reg_t, value: u64) -> hv_return_t;

    /// Gets pending interrupts for a vCPU.
    pub fn hv_vcpu_get_pending_interrupt(vcpu: hv_vcpu_t, interrupt: hv_interrupt_type_t, pending: *mut bool) -> hv_return_t;

    /// Sets pending interrupts for a vCPU.
    pub fn hv_vcpu_set_pending_interrupt(vcpu: hv_vcpu_t, interrupt: hv_interrupt_type_t, pending: bool) -> hv_return_t;

    /// Gets whether debug exceptions exit the guest.
    pub fn hv_vcpu_get_trap_debug_exceptions(vcpu: hv_vcpu_t, value: *mut bool) -> hv_return_t;

    /// Sets whether debug exceptions exit the guest.
    pub fn hv_vcpu_set_trap_debug_exceptions(vcpu: hv_vcpu_t, value: bool) -> hv_return_t;

    /// Gets whether debug-register accesses exit the guest.
    pub fn hv_vcpu_get_trap_debug_reg_accesses(vcpu: hv_vcpu_t, value: *mut bool) -> hv_return_t;

    /// Sets whether debug-register accesses exit the guest.
    pub fn hv_vcpu_set_trap_debug_reg_accesses(vcpu: hv_vcpu_t, value: bool) -> hv_return_t;

    /// Starts the execution of a vCPU.
    pub fn hv_vcpu_run(vcpu: hv_vcpu_t) -> hv_return_t;

    /// Forces an immediate exit of a set of vCPUs of the VM.
    pub fn hv_vcpus_exit(vcpus: *const hv_vcpu_t, vcpu_count: u32) -> hv_return_t;

    /// Returns, the cumulative execution time of a vCPU in nanoseconds.
    pub fn hv_vcpu_get_exec_time(vcpu: hv_vcpu_t, time: *mut u64) -> hv_return_t;

    /// Gets the virtual timer mask.
    pub fn hv_vcpu_get_vtimer_mask(vcpu: hv_vcpu_t, vtimer_is_masked: *mut bool) -> hv_return_t;

    /// Sets the virtual timer mask.
    pub fn hv_vcpu_set_vtimer_mask(vcpu: hv_vcpu_t, vtimer_is_masked: bool) -> hv_return_t;

    /// Gets the virtual timer offset.
    pub fn hv_vcpu_get_vtimer_offset(vcpu: hv_vcpu_t, vtimer_offset: *mut u64) -> hv_return_t;

    /// Sets the virtual timer offset.
    pub fn hv_vcpu_set_vtimer_offset(vcpu: hv_vcpu_t, vtimer_offset: u64) -> hv_return_t;
}