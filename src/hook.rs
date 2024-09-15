//! A collection of hooks and patches.

use std::ffi;

/// Replace the first six bytes of the main entrypoint with these bytes.
/// Do note that there are multiple entrypoints; you want the one that is called upon driver initialization (the "true" entrypoint).
/// ```asm
/// mov eax, 1
/// ret
/// ```
#[allow(unused)]
pub const CANCEL_DRIVER_ENTRY: &'static [u8] = &[0xB8, 0x01, 0x00, 0x00, 0x00, 0xC3];

#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn cancel_ExAcquireFastMutex(_fast_mutex: *mut ffi::c_void) {}

#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn cancel_ExReleaseFastMutex(_fast_mutex: *mut ffi::c_void) {}
