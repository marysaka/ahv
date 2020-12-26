//! Types used by the Hypervisor Framework for arm64 targets.

#![allow(non_camel_case_types)]

use std::ffi::c_void;

/// vCPU configuration.
pub type hv_vcpu_config_t = *mut c_void;

/// VM configuration.
pub type hv_vm_config_t = *mut c_void;

/// An opaque value that represents a vCPU instance.
pub type hv_vcpu_t = u64;

/// Events that can trigger a guest exit to the VM.
pub type hv_exit_reason_t = u32;

/// Type of a vCPU exception syndrome (Corresponds to ESR_EL2).
pub type hv_exception_syndrome_t = u64;

/// Type of a vCPU exception virtual address. (Corresponds to FAR_EL2).
pub type hv_exception_address_t = u64;

/// The type of an intermediate physical address (which is a guest physical address space of the VM).
pub type hv_ipa_t = u64;

/// Type of an ARM register.
pub type hv_reg_t = u32;

/// Type of an ARM SIMD & FP register.
pub type hv_simd_fp_reg_t = u32;

/// Type of an ARM system register.
pub type hv_sys_reg_t = u16;

/// Interrupt type.
pub type hv_interrupt_type_t = u32;

/// Cache type.
pub type hv_cache_type_t = u32;

//pub type hv_simd_fp_uchar16_t = core::arch::aarch64::uint8x16_t;

/// Memory region permissions.
pub type hv_memory_flags_t = u64;

/// Hypervisor error codes.
pub type hv_return_t = u32;

/// Type of ARM feature register.
pub type hv_feature_reg_t = u32;

/// Contains details of a vCPU exception.
#[repr(C)]
pub struct hv_vcpu_exit_exception_t
{
    /// The vCPU exception syndrome (Corresponds to ESR_EL2).
    pub syndrome: hv_exception_syndrome_t,

    /// The vCPU exception virtual address. (Corresponds to FAR_EL2).
    pub virtual_address: hv_exception_address_t,

    /// The vCPU exception physical address (host address).
    pub physical_address: hv_ipa_t
}

/// Information about an exit from the vCPU to the host.
#[repr(C)]
pub struct hv_vcpu_exit_t
{
    /// The exit reason.
    pub reason: hv_exit_reason_t,

    /// The exit exception informations.
    pub exception: hv_vcpu_exit_exception_t
}

/// The value that identifies exits requested by exit handler on the host.
pub const HV_EXIT_REASON_CANCELED: hv_exit_reason_t = 0;

/// The value that identifies traps caused by the guest operations.
pub const HV_EXIT_REASON_EXCEPTION: hv_exit_reason_t = 1;

/// The value that identifies when the virtual timer enters the pending state.
pub const HV_EXIT_REASON_VTIMER_ACTIVATED: hv_exit_reason_t = 2;

/// The value that identifies unexpected exits.
pub const HV_EXIT_REASON_UNKNOWN: hv_exit_reason_t = 3;

/// The value that identifies register X0.
pub const HV_REG_X0: hv_reg_t = 0;

/// The value that identifies register X1.
pub const HV_REG_X1: hv_reg_t = 1;

/// The value that identifies register X2.
pub const HV_REG_X2: hv_reg_t = 2;

/// The value that identifies register X3.
pub const HV_REG_X3: hv_reg_t = 3;

/// The value that identifies register X4.
pub const HV_REG_X4: hv_reg_t = 4;

/// The value that identifies register X5.
pub const HV_REG_X5: hv_reg_t = 5;

/// The value that identifies register X6.
pub const HV_REG_X6: hv_reg_t = 6;

/// The value that identifies register X7.
pub const HV_REG_X7: hv_reg_t = 7;

/// The value that identifies register X8.
pub const HV_REG_X8: hv_reg_t = 8;

/// The value that identifies register X9.
pub const HV_REG_X9: hv_reg_t = 9;

/// The value that identifies register X10.
pub const HV_REG_X10: hv_reg_t = 10;

/// The value that identifies register X11.
pub const HV_REG_X11: hv_reg_t = 11;

/// The value that identifies register X12.
pub const HV_REG_X12: hv_reg_t = 12;

/// The value that identifies register X13.
pub const HV_REG_X13: hv_reg_t = 13;

/// The value that identifies register X14.
pub const HV_REG_X14: hv_reg_t = 14;

/// The value that identifies register X15.
pub const HV_REG_X15: hv_reg_t = 15;

/// The value that identifies register X16.
pub const HV_REG_X16: hv_reg_t = 16;

/// The value that identifies register X17.
pub const HV_REG_X17: hv_reg_t = 17;

/// The value that identifies register X18.
pub const HV_REG_X18: hv_reg_t = 18;

/// The value that identifies register X19.
pub const HV_REG_X19: hv_reg_t = 19;

/// The value that identifies register X20.
pub const HV_REG_X20: hv_reg_t = 20;

/// The value that identifies register X21.
pub const HV_REG_X21: hv_reg_t = 21;

/// The value that identifies register X22.
pub const HV_REG_X22: hv_reg_t = 22;

/// The value that identifies register X23.
pub const HV_REG_X23: hv_reg_t = 23;

/// The value that identifies register X24.
pub const HV_REG_X24: hv_reg_t = 24;

/// The value that identifies register X25.
pub const HV_REG_X25: hv_reg_t = 25;

/// The value that identifies register X26.
pub const HV_REG_X26: hv_reg_t = 26;

/// The value that identifies register X27.
pub const HV_REG_X27: hv_reg_t = 27;

/// The value that identifies register X28.
pub const HV_REG_X28: hv_reg_t = 28;

/// The value that identifies register X29.
pub const HV_REG_X29: hv_reg_t = 29;

/// The value that identifies register FP.
pub const HV_REG_FP: hv_reg_t = HV_REG_X29;

/// The value that identifies register X30.
pub const HV_REG_X30: hv_reg_t = 30;

/// The value that identifies register LR.
pub const HV_REG_LR: hv_reg_t = HV_REG_X30;

/// The value that identifies register PC.
pub const HV_REG_PC: hv_reg_t = 31;

/// The value that identifies register FPCR.
pub const HV_REG_FPCR: hv_reg_t = 32;

/// The value that identifies register FPSR.
pub const HV_REG_FPSR: hv_reg_t = 33;

/// The value that identifies register CPSR.
pub const HV_REG_CPSR: hv_reg_t = 34;

/// The value that identifies register Q0.
pub const HV_SIMD_FP_REG_Q0: hv_simd_fp_reg_t = 0;

/// The value that identifies register Q1.
pub const HV_SIMD_FP_REG_Q1: hv_simd_fp_reg_t = 1;

/// The value that identifies register Q2.
pub const HV_SIMD_FP_REG_Q2: hv_simd_fp_reg_t = 2;

/// The value that identifies register Q3.
pub const HV_SIMD_FP_REG_Q3: hv_simd_fp_reg_t = 3;

/// The value that identifies register Q4.
pub const HV_SIMD_FP_REG_Q4: hv_simd_fp_reg_t = 4;

/// The value that identifies register Q5.
pub const HV_SIMD_FP_REG_Q5: hv_simd_fp_reg_t = 5;

/// The value that identifies register Q6.
pub const HV_SIMD_FP_REG_Q6: hv_simd_fp_reg_t = 6;

/// The value that identifies register Q7.
pub const HV_SIMD_FP_REG_Q7: hv_simd_fp_reg_t = 7;

/// The value that identifies register Q8.
pub const HV_SIMD_FP_REG_Q8: hv_simd_fp_reg_t = 8;

/// The value that identifies register Q9.
pub const HV_SIMD_FP_REG_Q9: hv_simd_fp_reg_t = 9;

/// The value that identifies register Q10.
pub const HV_SIMD_FP_REG_Q10: hv_simd_fp_reg_t = 10;

/// The value that identifies register Q11.
pub const HV_SIMD_FP_REG_Q11: hv_simd_fp_reg_t = 11;

/// The value that identifies register Q12.
pub const HV_SIMD_FP_REG_Q12: hv_simd_fp_reg_t = 12;

/// The value that identifies register Q13.
pub const HV_SIMD_FP_REG_Q13: hv_simd_fp_reg_t = 13;

/// The value that identifies register Q14.
pub const HV_SIMD_FP_REG_Q14: hv_simd_fp_reg_t = 14;

/// The value that identifies register Q15.
pub const HV_SIMD_FP_REG_Q15: hv_simd_fp_reg_t = 15;

/// The value that identifies register Q16.
pub const HV_SIMD_FP_REG_Q16: hv_simd_fp_reg_t = 16;

/// The value that identifies register Q17.
pub const HV_SIMD_FP_REG_Q17: hv_simd_fp_reg_t = 17;

/// The value that identifies register Q18.
pub const HV_SIMD_FP_REG_Q18: hv_simd_fp_reg_t = 18;

/// The value that identifies register Q19.
pub const HV_SIMD_FP_REG_Q19: hv_simd_fp_reg_t = 19;

/// The value that identifies register Q20.
pub const HV_SIMD_FP_REG_Q20: hv_simd_fp_reg_t = 20;

/// The value that identifies register Q21.
pub const HV_SIMD_FP_REG_Q21: hv_simd_fp_reg_t = 21;

/// The value that identifies register Q22.
pub const HV_SIMD_FP_REG_Q22: hv_simd_fp_reg_t = 22;

/// The value that identifies register Q23.
pub const HV_SIMD_FP_REG_Q23: hv_simd_fp_reg_t = 23;

/// The value that identifies register Q24.
pub const HV_SIMD_FP_REG_Q24: hv_simd_fp_reg_t = 24;

/// The value that identifies register Q25.
pub const HV_SIMD_FP_REG_Q25: hv_simd_fp_reg_t = 25;

/// The value that identifies register Q26.
pub const HV_SIMD_FP_REG_Q26: hv_simd_fp_reg_t = 26;

/// The value that identifies register Q27.
pub const HV_SIMD_FP_REG_Q27: hv_simd_fp_reg_t = 27;

/// The value that identifies register Q28.
pub const HV_SIMD_FP_REG_Q28: hv_simd_fp_reg_t = 28;

/// The value that identifies register Q29.
pub const HV_SIMD_FP_REG_Q29: hv_simd_fp_reg_t = 29;

/// The value that identifies register Q30.
pub const HV_SIMD_FP_REG_Q30: hv_simd_fp_reg_t = 30;

/// The value that identifies register Q31.
pub const HV_SIMD_FP_REG_Q31: hv_simd_fp_reg_t = 31;


/// The value that identifies register DBGBVR0_EL1.
pub const HV_SYS_REG_DBGBVR0_EL1: hv_sys_reg_t = 0x8004;

/// The value that identifies register DBGBCR0_EL1.
pub const HV_SYS_REG_DBGBCR0_EL1: hv_sys_reg_t = 0x8005;

/// The value that identifies register DBGWVR0_EL1.
pub const HV_SYS_REG_DBGWVR0_EL1: hv_sys_reg_t = 0x8006;

/// The value that identifies register DBGWCR0_EL1.
pub const HV_SYS_REG_DBGWCR0_EL1: hv_sys_reg_t = 0x8007;

/// The value that identifies register DBGBVR1_EL1.
pub const HV_SYS_REG_DBGBVR1_EL1: hv_sys_reg_t = 0x800c;

/// The value that identifies register DBGBCR1_EL1.
pub const HV_SYS_REG_DBGBCR1_EL1: hv_sys_reg_t = 0x800d;

/// The value that identifies register DBGWVR1_EL1.
pub const HV_SYS_REG_DBGWVR1_EL1: hv_sys_reg_t = 0x800e;

/// The value that identifies register DBGWCR1_EL1.
pub const HV_SYS_REG_DBGWCR1_EL1: hv_sys_reg_t = 0x800f;

/// The value that identifies register MDCCINT_EL1.
pub const HV_SYS_REG_MDCCINT_EL1: hv_sys_reg_t = 0x8010;

/// The value that identifies register MDSCR_EL1.
pub const HV_SYS_REG_MDSCR_EL1: hv_sys_reg_t = 0x8012;

/// The value that identifies register DBGBVR2_EL1.
pub const HV_SYS_REG_DBGBVR2_EL1: hv_sys_reg_t = 0x8014;

/// The value that identifies register DBGBCR2_EL1.
pub const HV_SYS_REG_DBGBCR2_EL1: hv_sys_reg_t = 0x8015;

/// The value that identifies register DBGWVR2_EL1.
pub const HV_SYS_REG_DBGWVR2_EL1: hv_sys_reg_t = 0x8016;

/// The value that identifies register DBGWCR2_EL1.
pub const HV_SYS_REG_DBGWCR2_EL1: hv_sys_reg_t = 0x8017;

/// The value that identifies register DBGBVR3_EL1.
pub const HV_SYS_REG_DBGBVR3_EL1: hv_sys_reg_t = 0x801c;

/// The value that identifies register DBGBCR3_EL1.
pub const HV_SYS_REG_DBGBCR3_EL1: hv_sys_reg_t = 0x801d;

/// The value that identifies register DBGWVR3_EL1.
pub const HV_SYS_REG_DBGWVR3_EL1: hv_sys_reg_t = 0x801e;

/// The value that identifies register DBGWCR3_EL1.
pub const HV_SYS_REG_DBGWCR3_EL1: hv_sys_reg_t = 0x801f;

/// The value that identifies register DBGBVR4_EL1.
pub const HV_SYS_REG_DBGBVR4_EL1: hv_sys_reg_t = 0x8024;

/// The value that identifies register DBGBCR4_EL1.
pub const HV_SYS_REG_DBGBCR4_EL1: hv_sys_reg_t = 0x8025;

/// The value that identifies register DBGWVR4_EL1.
pub const HV_SYS_REG_DBGWVR4_EL1: hv_sys_reg_t = 0x8026;

/// The value that identifies register DBGWCR4_EL1.
pub const HV_SYS_REG_DBGWCR4_EL1: hv_sys_reg_t = 0x8027;

/// The value that identifies register DBGBVR5_EL1.
pub const HV_SYS_REG_DBGBVR5_EL1: hv_sys_reg_t = 0x802c;

/// The value that identifies register DBGBCR5_EL1.
pub const HV_SYS_REG_DBGBCR5_EL1: hv_sys_reg_t = 0x802d;

/// The value that identifies register DBGWVR5_EL1.
pub const HV_SYS_REG_DBGWVR5_EL1: hv_sys_reg_t = 0x802e;

/// The value that identifies register BGWCR5_EL1.
pub const HV_SYS_REG_DBGWCR5_EL1: hv_sys_reg_t = 0x802f;

/// The value that identifies register DBGBVR6_EL1.
pub const HV_SYS_REG_DBGBVR6_EL1: hv_sys_reg_t = 0x8034;

/// The value that identifies register DBGBCR6_EL1.
pub const HV_SYS_REG_DBGBCR6_EL1: hv_sys_reg_t = 0x8035;

/// The value that identifies register DBGWVR6_EL1.
pub const HV_SYS_REG_DBGWVR6_EL1: hv_sys_reg_t = 0x8036;

/// The value that identifies register DBGWCR6_EL1.
pub const HV_SYS_REG_DBGWCR6_EL1: hv_sys_reg_t = 0x8037;

/// The value that identifies register DBGBVR7_EL1.
pub const HV_SYS_REG_DBGBVR7_EL1: hv_sys_reg_t = 0x803c;

/// The value that identifies register DBGBCR7_EL1.
pub const HV_SYS_REG_DBGBCR7_EL1: hv_sys_reg_t = 0x803d;

/// The value that identifies register DBGWVR7_EL1.
pub const HV_SYS_REG_DBGWVR7_EL1: hv_sys_reg_t = 0x803e;

/// The value that identifies register DBGWCR7_EL1.
pub const HV_SYS_REG_DBGWCR7_EL1: hv_sys_reg_t = 0x803f;

/// The value that identifies register DBGBVR8_EL1.
pub const HV_SYS_REG_DBGBVR8_EL1: hv_sys_reg_t = 0x8044;

/// The value that identifies register DBGBCR8_EL1.
pub const HV_SYS_REG_DBGBCR8_EL1: hv_sys_reg_t = 0x8045;

/// The value that identifies register DBGWVR8_EL1.
pub const HV_SYS_REG_DBGWVR8_EL1: hv_sys_reg_t = 0x8046;

/// The value that identifies register DBGWCR8_EL1.
pub const HV_SYS_REG_DBGWCR8_EL1: hv_sys_reg_t = 0x8047;

/// The value that identifies register DBGBVR9_EL1.
pub const HV_SYS_REG_DBGBVR9_EL1: hv_sys_reg_t = 0x804c;

/// The value that identifies register DBGBCR9_EL1.
pub const HV_SYS_REG_DBGBCR9_EL1: hv_sys_reg_t = 0x804d;

/// The value that identifies register DBGWVR9_EL1.
pub const HV_SYS_REG_DBGWVR9_EL1: hv_sys_reg_t = 0x804e;

/// The value that identifies register DBGWCR9_EL1.
pub const HV_SYS_REG_DBGWCR9_EL1: hv_sys_reg_t = 0x804f;

/// The value that identifies register DBGBVR10_EL1.
pub const HV_SYS_REG_DBGBVR10_EL1: hv_sys_reg_t = 0x8054;

/// The value that identifies register DBGBCR10_EL1.
pub const HV_SYS_REG_DBGBCR10_EL1: hv_sys_reg_t = 0x8055;

/// The value that identifies register DBGWVR10_EL1.
pub const HV_SYS_REG_DBGWVR10_EL1: hv_sys_reg_t = 0x8056;

/// The value that identifies register DBGWCR10_EL1.
pub const HV_SYS_REG_DBGWCR10_EL1: hv_sys_reg_t = 0x8057;

/// The value that identifies register DBGBVR11_EL1.
pub const HV_SYS_REG_DBGBVR11_EL1: hv_sys_reg_t = 0x805c;

/// The value that identifies register DBGBCR11_EL1.
pub const HV_SYS_REG_DBGBCR11_EL1: hv_sys_reg_t = 0x805d;

/// The value that identifies register DBGWVR11_EL1.
pub const HV_SYS_REG_DBGWVR11_EL1: hv_sys_reg_t = 0x805e;

/// The value that identifies register DBGWCR11_EL1.
pub const HV_SYS_REG_DBGWCR11_EL1: hv_sys_reg_t = 0x805f;

/// The value that identifies register DBGBVR12_EL1.
pub const HV_SYS_REG_DBGBVR12_EL1: hv_sys_reg_t = 0x8064;

/// The value that identifies register DBGBCR12_EL1.
pub const HV_SYS_REG_DBGBCR12_EL1: hv_sys_reg_t = 0x8065;

/// The value that identifies register DBGWVR12_EL1.
pub const HV_SYS_REG_DBGWVR12_EL1: hv_sys_reg_t = 0x8066;

/// The value that identifies register DBGWCR12_EL1.
pub const HV_SYS_REG_DBGWCR12_EL1: hv_sys_reg_t = 0x8067;

/// The value that identifies register DBGBVR13_EL1.
pub const HV_SYS_REG_DBGBVR13_EL1: hv_sys_reg_t = 0x806c;

/// The value that identifies register DBGBCR13_EL1.
pub const HV_SYS_REG_DBGBCR13_EL1: hv_sys_reg_t = 0x806d;

/// The value that identifies register DBGWVR13_EL1.
pub const HV_SYS_REG_DBGWVR13_EL1: hv_sys_reg_t = 0x806e;

/// The value that identifies register DBGWCR13_EL1.
pub const HV_SYS_REG_DBGWCR13_EL1: hv_sys_reg_t = 0x806f;

/// The value that identifies register DBGBVR14_EL1.
pub const HV_SYS_REG_DBGBVR14_EL1: hv_sys_reg_t = 0x8074;

/// The value that identifies register DBGBCR14_EL1.
pub const HV_SYS_REG_DBGBCR14_EL1: hv_sys_reg_t = 0x8075;

/// The value that identifies register DBGWVR14_EL1.
pub const HV_SYS_REG_DBGWVR14_EL1: hv_sys_reg_t = 0x8076;

/// The value that identifies register DBGWCR14_EL1.
pub const HV_SYS_REG_DBGWCR14_EL1: hv_sys_reg_t = 0x8077;

/// The value that identifies register DBGBVR15_EL1.
pub const HV_SYS_REG_DBGBVR15_EL1: hv_sys_reg_t = 0x807c;

/// The value that identifies register DBGBCR15_EL1.
pub const HV_SYS_REG_DBGBCR15_EL1: hv_sys_reg_t = 0x807d;

/// The value that identifies register DBGWVR15_EL1.
pub const HV_SYS_REG_DBGWVR15_EL1: hv_sys_reg_t = 0x807e;

/// The value that identifies register DBGWCR15_EL1.
pub const HV_SYS_REG_DBGWCR15_EL1: hv_sys_reg_t = 0x807f;

/// The value that identifies register MIDR_EL1.
pub const HV_SYS_REG_MIDR_EL1: hv_sys_reg_t = 0xc000;

/// The value that identifies register MPIDR_EL1.
pub const HV_SYS_REG_MPIDR_EL1: hv_sys_reg_t = 0xc005;

/// The value that identifies register AA64PFR0_EL1.
pub const HV_SYS_REG_ID_AA64PFR0_EL1: hv_sys_reg_t = 0xc020;

/// The value that identifies register AA64PFR1_EL1.
pub const HV_SYS_REG_ID_AA64PFR1_EL1: hv_sys_reg_t = 0xc021;

/// The value that identifies register AA64DFR0_EL1.
pub const HV_SYS_REG_ID_AA64DFR0_EL1: hv_sys_reg_t = 0xc028;

/// The value that identifies register AA64DFR1_EL1.
pub const HV_SYS_REG_ID_AA64DFR1_EL1: hv_sys_reg_t = 0xc029;

/// The value that identifies register AA64ISAR0_EL1.
pub const HV_SYS_REG_ID_AA64ISAR0_EL1: hv_sys_reg_t = 0xc030;

/// The value that identifies register AA64ISAR1_EL1.
pub const HV_SYS_REG_ID_AA64ISAR1_EL1: hv_sys_reg_t = 0xc031;

/// The value that identifies register AA64MMFR0_EL1.
pub const HV_SYS_REG_ID_AA64MMFR0_EL1: hv_sys_reg_t = 0xc038;

/// The value that identifies register AA64MMFR1_EL1.
pub const HV_SYS_REG_ID_AA64MMFR1_EL1: hv_sys_reg_t = 0xc039;

/// The value that identifies register AA64MMFR2_EL1.
pub const HV_SYS_REG_ID_AA64MMFR2_EL1: hv_sys_reg_t = 0xc03a;

/// The value that identifies register SCTLR_EL1.
pub const HV_SYS_REG_SCTLR_EL1: hv_sys_reg_t = 0xc080;

/// The value that identifies register CPACR_EL1.
pub const HV_SYS_REG_CPACR_EL1: hv_sys_reg_t = 0xc082;

/// The value that identifies register TTBR0_EL1.
pub const HV_SYS_REG_TTBR0_EL1: hv_sys_reg_t = 0xc100;

/// The value that identifies register TTBR1_EL1.
pub const HV_SYS_REG_TTBR1_EL1: hv_sys_reg_t = 0xc101;

/// The value that identifies register TCR_EL1.
pub const HV_SYS_REG_TCR_EL1: hv_sys_reg_t = 0xc102;

/// The value that identifies register APIAKEYLO_EL1.
pub const HV_SYS_REG_APIAKEYLO_EL1: hv_sys_reg_t = 0xc108;

/// The value that identifies register APIAKEYHI_EL1.
pub const HV_SYS_REG_APIAKEYHI_EL1: hv_sys_reg_t = 0xc109;

/// The value that identifies register APIBKEYLO_EL1.
pub const HV_SYS_REG_APIBKEYLO_EL1: hv_sys_reg_t = 0xc10a;

/// The value that identifies register APIBKEYHI_EL1.
pub const HV_SYS_REG_APIBKEYHI_EL1: hv_sys_reg_t = 0xc10b;

/// The value that identifies register APDAKEYLO_EL1.
pub const HV_SYS_REG_APDAKEYLO_EL1: hv_sys_reg_t = 0xc110;

/// The value that identifies register APDAKEYHI_EL1.
pub const HV_SYS_REG_APDAKEYHI_EL1: hv_sys_reg_t = 0xc111;

/// The value that identifies register APDBKEYLO_EL1.
pub const HV_SYS_REG_APDBKEYLO_EL1: hv_sys_reg_t = 0xc112;

/// The value that identifies register APDBKEYHI_EL1.
pub const HV_SYS_REG_APDBKEYHI_EL1: hv_sys_reg_t = 0xc113;

/// The value that identifies register APGAKEYLO_EL1.
pub const HV_SYS_REG_APGAKEYLO_EL1: hv_sys_reg_t = 0xc118;

/// The value that identifies register APGAKEYHI_EL1.
pub const HV_SYS_REG_APGAKEYHI_EL1: hv_sys_reg_t = 0xc119;

/// The value that identifies register SPSR_EL1.
pub const HV_SYS_REG_SPSR_EL1: hv_sys_reg_t = 0xc200;

/// The value that identifies register ELR_EL1.
pub const HV_SYS_REG_ELR_EL1: hv_sys_reg_t = 0xc201;

/// The value that identifies register SP_EL0.
pub const HV_SYS_REG_SP_EL0: hv_sys_reg_t = 0xc208;

/// The value that identifies register AFSR0_EL1.
pub const HV_SYS_REG_AFSR0_EL1: hv_sys_reg_t = 0xc288;

/// The value that identifies register AFSR1_EL1.
pub const HV_SYS_REG_AFSR1_EL1: hv_sys_reg_t = 0xc289;

/// The value that identifies register ESR_EL1.
pub const HV_SYS_REG_ESR_EL1: hv_sys_reg_t = 0xc290;

/// The value that identifies register FAR_EL1.
pub const HV_SYS_REG_FAR_EL1: hv_sys_reg_t = 0xc300;

/// The value that identifies register PAR_EL1.
pub const HV_SYS_REG_PAR_EL1: hv_sys_reg_t = 0xc3a0;

/// The value that identifies register MAIR_EL1.
pub const HV_SYS_REG_MAIR_EL1: hv_sys_reg_t = 0xc510;

/// The value that identifies register AMAIR_EL1.
pub const HV_SYS_REG_AMAIR_EL1: hv_sys_reg_t = 0xc518;

/// The value that identifies register VBAR_EL1.
pub const HV_SYS_REG_VBAR_EL1: hv_sys_reg_t = 0xc600;

/// The value that identifies register CONTEXTIDR_EL1.
pub const HV_SYS_REG_CONTEXTIDR_EL1: hv_sys_reg_t = 0xc681;

/// The value that identifies register TPIDR_EL1.
pub const HV_SYS_REG_TPIDR_EL1: hv_sys_reg_t = 0xc684;

/// The value that identifies register CNTKCTL_EL1.
pub const HV_SYS_REG_CNTKCTL_EL1: hv_sys_reg_t = 0xc708;

/// The value that identifies register CSSELR_EL1.
pub const HV_SYS_REG_CSSELR_EL1: hv_sys_reg_t = 0xd000;

/// The value that identifies register TPIDR_EL0.
pub const HV_SYS_REG_TPIDR_EL0: hv_sys_reg_t = 0xde82;

/// The value that identifies register TPIDRRO_EL0.
pub const HV_SYS_REG_TPIDRRO_EL0: hv_sys_reg_t = 0xde83;

/// The value that identifies register CNTV_CTL_EL0.
pub const HV_SYS_REG_CNTV_CTL_EL0: hv_sys_reg_t = 0xdf19;

/// The value that identifies register CNTV_CVAL_EL0.
pub const HV_SYS_REG_CNTV_CVAL_EL0: hv_sys_reg_t = 0xdf1a;

/// The value that identifies register SP_EL1.
pub const HV_SYS_REG_SP_EL1: hv_sys_reg_t = 0xe208;

/// An ARM IRQ.
pub const HV_INTERRUPT_TYPE_IRQ: hv_interrupt_type_t = 0;

/// An ARM FIQ.
pub const HV_INTERRUPT_TYPE_FIQ: hv_interrupt_type_t = 1;

/// Data cache.
pub const HV_CACHE_TYPE_DATA: hv_cache_type_t = 0;

/// Instruction cache.
pub const HV_CACHE_TYPE_INSTRUCTION: hv_cache_type_t = 1;

/// Success.
pub const HV_SUCCESS: hv_return_t = 0;

/// Hypervisor Error.
pub const HV_ERROR: hv_return_t = 0xfae94001;

/// Busy.
pub const HV_BUSY: hv_return_t = 0xfae94002;

/// Bad argument.
pub const HV_BAD_ARGUMENT: hv_return_t = 0xfae94003;

/// Illegal guest state.
pub const HV_ILLEGAL_GUEST_STATE: hv_return_t = 0xfae94004;

/// No resources.
pub const HV_NO_RESOURCES: hv_return_t = 0xfae94005;

/// No device.
pub const HV_NO_DEVICE: hv_return_t = 0xfae94006;

/// Denied.
pub const HV_DENIED: hv_return_t = 0xfae94007;

/// Unsupported.
pub const HV_UNSUPPORTED: hv_return_t = 0xfae9400f;

/// Read memory permission.
pub const HV_MEMORY_READ: hv_memory_flags_t = 1 << 0;

/// Write memory permission.
pub const HV_MEMORY_WRITE: hv_memory_flags_t = 1 << 1;

/// Execute memory permission.
pub const HV_MEMORY_EXEC: hv_memory_flags_t = 1 << 2;

/// The value that identifies feature register AA64DFR0_EL1.
pub const HV_FEATURE_REG_ID_AA64DFR0_EL1: hv_feature_reg_t = 0;

/// The value that identifies feature register AA64DFR1_EL1.
pub const HV_FEATURE_REG_ID_AA64DFR1_EL1: hv_feature_reg_t = 1;

/// The value that identifies feature register AA64ISAR0_EL1.
pub const HV_FEATURE_REG_ID_AA64ISAR0_EL1: hv_feature_reg_t = 2;

/// The value that identifies feature register AA64ISAR1_EL1.
pub const HV_FEATURE_REG_ID_AA64ISAR1_EL1: hv_feature_reg_t = 3;

/// The value that identifies feature register AA64MMFR0_EL1.
pub const HV_FEATURE_REG_ID_AA64MMFR0_EL1: hv_feature_reg_t = 4;

/// The value that identifies feature register AA64MMFR1_EL1.
pub const HV_FEATURE_REG_ID_AA64MMFR1_EL1: hv_feature_reg_t = 5;

/// The value that identifies feature register AA64MMFR2_EL1.
pub const HV_FEATURE_REG_ID_AA64MMFR2_EL1: hv_feature_reg_t = 6;

/// The value that identifies feature register AA64PFR0_EL1.
pub const HV_FEATURE_REG_ID_AA64PFR0_EL1: hv_feature_reg_t = 7;

/// The value that identifies feature register AA64PFR1_EL1.
pub const HV_FEATURE_REG_ID_AA64PFR1_EL1: hv_feature_reg_t = 8;

/// The value that identifies feature register CTR_EL0.
pub const HV_FEATURE_REG_CTR_EL0: hv_feature_reg_t = 9;

/// The value that identifies feature register CLIDR_EL1.
pub const HV_FEATURE_REG_CLIDR_EL1: hv_feature_reg_t = 10;

/// The value that identifies feature register DCZID_EL0.
pub const HV_FEATURE_REG_DCZID_EL0: hv_feature_reg_t = 11;
