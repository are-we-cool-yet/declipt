//! A collection of hooks and patches.
#![allow(non_snake_case)]

use std::ffi;

use winapi::shared::{minwindef, ntdef};

use crate::constants::{Mdl, KPROCESSOR_MODE, LOCK_OPERATION, MEMORY_CACHING_TYPE, QWORD};

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

pub unsafe extern "fastcall" fn MmChangeImageProtection(_arg0: QWORD, _arg1: QWORD, _arg2: QWORD, _arg3: QWORD) -> winapi::ctypes::__int64 {
    minwindef::TRUE as _
}

pub unsafe extern "stdcall" fn IoAllocateMdl(virtual_address: ntdef::PVOID, length: minwindef::ULONG, _secondary_buffer: ntdef::BOOLEAN, _charge_quota: ntdef::BOOLEAN, irp: ntdef::PVOID) -> *mut Mdl {
    println!("IoAllocateMdl");
    if !irp.is_null() {
        eprintln!("Non-null IRP found! Non-null IRPs are unsupported.");
    }
    let mut mdl = Mdl {
        virtual_address,
        length,
        _pad: Default::default(),
    };
    &mut mdl as *mut Mdl
}

pub unsafe extern "stdcall" fn IoFreeMdl(_mdl: *mut Mdl) {
    println!("IoFreeMdl");
}

pub unsafe extern "stdcall" fn MmProbeAndLockPages(_memory_descriptor_list: *mut Mdl, _access_mode: KPROCESSOR_MODE, _operation: LOCK_OPERATION) {
    println!("MmProbeAndLockPages");
}

pub unsafe extern "stdcall" fn MmUnlockPages(_memory_descriptor_list: *mut Mdl) {
    println!("MmUnlockPages");
}

pub unsafe extern "stdcall" fn MmLockPagableDataSection(_address_within_section: ntdef::PVOID) -> ntdef::PVOID {
    println!("MmLockPagableDataSection");
    0x0 as _
}

pub unsafe extern "stdcall" fn MmMapLockedPagesSpecifyCache(_memory_descriptor_list: *mut Mdl, _access_mode: KPROCESSOR_MODE, _cache_type: MEMORY_CACHING_TYPE) -> ntdef::PVOID {
    println!("MmMapLockedPagesSpecifyCache");
    0x0 as _
}

pub unsafe extern "stdcall" fn MmUnmapLockedPages(_base_address: ntdef::PVOID, _memory_descriptor_list: *mut Mdl) {
    println!("MmUnmapLockedPages");
}
