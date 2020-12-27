// TODO: Currently, the SIMD types doesn't seem to support interoperability with C, find a way to bypass that.
#![feature(negative_impls/*, stdsimd, simd_ffi) */)]
#![deny(clippy::missing_docs_in_private_items)]
#![no_std]

//!
//! Apple Hypervisor
//!
//! This crate allows interaction with the Hypervisor Framework on Apple Silicon in a safe (``ahv`` module, still WIP) and unsafe (``ffi`` module) way.
//!

extern crate alloc;

pub mod api;
pub mod ffi;

pub use ffi::types::hv_ipa_t;
pub use ffi::types::hv_vcpu_t;
pub use api::*;
