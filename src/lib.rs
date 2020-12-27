// TODO: Currently, the SIMD types doesn't seem to support interoperability with C, find a way to bypass that.
#![feature(negative_impls/*, stdsimd, simd_ffi) */)]
#![deny(clippy::missing_docs_in_private_items)]
#![no_std]

//!
//! Apple Hypervisor
//!
//! This crate allows interaction with the Hypervisor Framework on Apple Silicon in a safe (``ahv`` module) and unsafe (``ffi`` module) way.
//!
//! # Example:
//!
//! The following example execute a move of the immediate value 2 to register x0 at EL1 and then call HVC 0.
//!
//! ```
//! use ahv::*;
//!
//! fn main() -> Result<()> {
//!     let el1_user_payload = [
//!         0x40, 0x00, 0x80, 0xD2, // mov x0, #2
//!         0x02, 0x00, 0x00, 0xD4, // hvc #0
//!     ];
//!
//!     const EL1_USER_PAYLOAD_ADDRESS: hv_ipa_t = 0x20000;
//!     let mut virtual_machine: VirtualMachine = VirtualMachine::new(None)?;
//!     let el1_user_payload_allocation_handle = virtual_machine.allocate_from(&el1_user_payload)?;
//!     virtual_machine.map(el1_user_payload_allocation_handle,
//!                         EL1_USER_PAYLOAD_ADDRESS,
//!                         MemoryPermission::READ_WRITE_EXECUTE)?;
//!
//!     {
//!         // vCPU scope
//!         let mut vcpu = virtual_machine.create_vcpu(None)?;
//!
//!         vcpu.set_register(Register::CPSR, 0x3c4)?;
//!         vcpu.set_register(Register::PC, EL1_USER_PAYLOAD_ADDRESS)?;
//!         vcpu.set_trap_debug_exceptions(true)?;
//!     
//!         loop {
//!             let result = vcpu.run()?;
//!     
//!             match result {
//!                 VirtualCpuExitReason::Exception { exception } => {
//!                     let ec = (exception.syndrome >> 26) & 0x3f;
//!     
//!                     if ec == 0x16 {
//!                         println!("HVC executed! x0 is {}", vcpu.get_register(Register::X0)?);
//!                         break;
//!                     } else {
//!                         println!("Unknown exception class 0x{:x}", ec);
//!                         break;
//!                     }
//!                 }
//!                 reason => {
//!                     println!("Unexpected exit! Reason: {:?}", reason);
//!                     break;
//!                 }
//!             }
//!         }
//!     }
//!
//!     // VirtualMachine will unmap and deallocate on drop.
//!
//!     Ok(())
//! }
//!
//! **To run this example make sure to give the built binary the ``com.apple.security.hypervisor`` entitlement.**
//!
//! ```

extern crate alloc;

pub mod api;
pub mod ffi;

pub use api::*;
pub use ffi::types::hv_ipa_t;
pub use ffi::types::hv_vcpu_t;
