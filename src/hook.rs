//! A collection of hooks and patches.
#![allow(non_snake_case)]

use std::ffi;

use winapi::shared::{minwindef::ULONG, ntdef::{BOOLEAN, PVOID}};

use crate::constants::{KPROCESSOR_MODE, LOCK_OPERATION, MEMORY_CACHING_TYPE};

/// Replace the first six bytes of the main entrypoint with these bytes.
/// Do note that there are multiple entrypoints; you want the one that is called upon driver initialization (the "true" entrypoint).
/// ```asm
/// mov eax, 1
/// ret
/// ```
#[allow(unused)]
pub const CANCEL_DRIVER_ENTRY: &'static [u8] = &[0xB8, 0x01, 0x00, 0x00, 0x00, 0xC3];

pub unsafe extern "stdcall" fn cancel_ExAcquireFastMutex(_fast_mutex: *mut ffi::c_void) {}

pub unsafe extern "stdcall" fn cancel_ExReleaseFastMutex(_fast_mutex: *mut ffi::c_void) {}

pub unsafe extern "stdcall" fn IoAllocateMdl(_virtual_address: PVOID, _length: ULONG, _secondary_buffer: BOOLEAN, _charge_quota: BOOLEAN, _irp: PVOID) -> PVOID {
    println!("Input IRP 0x{:X}", _irp as usize);
    _irp
}

pub unsafe extern "stdcall" fn IoFreeMdl(_mdl: PVOID) {
    println!("IoFreeMdl");
}

pub unsafe extern "stdcall" fn MmProbeAndLockPages(_memory_descriptor_list: PVOID, _access_mode: KPROCESSOR_MODE, _operation: LOCK_OPERATION) {
    println!("MmProbeAndLockPages");
}

pub unsafe extern "stdcall" fn MmLockPagableDataSection(_address_within_section: PVOID) -> PVOID {
    println!("MmLockPagableDataSection");
    0x0 as _
}

pub unsafe extern "stdcall" fn MmMapLockedPagesSpecifyCache(_memory_descriptor_list: PVOID, _access_mode: KPROCESSOR_MODE, _cache_type: MEMORY_CACHING_TYPE) -> PVOID {
    println!("MmMapLockedPagesSpecifyCache");
    0x0 as _
}

pub unsafe extern "stdcall" fn MmUnlockPages(_memory_descriptor_list: PVOID) {
    println!("MmUnlockPages");
}
