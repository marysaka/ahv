// TODO: Currently, the SIMD types doesn't seems to support interoperability with C, find a way to bypass that.
//#![feature(stdsimd, simd_ffi)]

#![deny(clippy::missing_docs_in_private_items)]

//! Apple Hypervisor
//! 
//! This crate allows interaction with the Hypervisor Framework on Apple Silicon in a safe (``ahv`` module, still TODO) and unsafe (``ffi`` module) way.
//! 


pub mod ffi;
