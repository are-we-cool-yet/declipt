//! Types relating to Windows.

#![allow(non_camel_case_types)]

use core::mem::ManuallyDrop;

use winapi::shared::ntdef;

/// The decrypted memory, to be accessed by the main thread.
pub type DecryptMessage = Vec<u8>;

// Function Types
pub type WarbirdDecrypt = unsafe extern "fastcall" fn(rw_data: winapi::ctypes::__int64, const_data: *mut winapi::ctypes::c_int) -> winapi::ctypes::__int64;

// Types n Shit
pub type QWORD = winapi::ctypes::c_ulonglong;
pub type KPROCESSOR_MODE = winapi::shared::ntdef::CCHAR;
#[repr(C)]
pub enum LOCK_OPERATION {
    IoReadAccess = 0x0,
    IoWriteAccess = 0x1,
    IoModifyAccess = 0x2,
}
#[repr(C)]
pub enum MEMORY_CACHING_TYPE {
    MmNonCached = 0x0,
    MmCached = 0x1,
    MmWriteCombined = 0x2,
    MmHardwareCoherentCached = 0x3,
    MmNonCachedUnordered = 0x4,
    MmUSWCCached = 0x5,
    MmMaximumCacheType = 0x6,
    MmNotMapped = 0xFFFFFFFF,
}
/// An opaque structure used in [Mdl].
///
/// https://learn.microsoft.com/en-us/windows-hardware/drivers/kernel/eprocess#eprocess
#[repr(C)]
pub struct EPROCESS {
    pub pid: u16,
}
#[repr(C)]
pub struct MDL {
    pub next: ManuallyDrop<*mut MDL>,
    pub size: ntdef::CSHORT,
    pub mdl_flags: ntdef::CSHORT,
    pub process: ManuallyDrop<*const EPROCESS>,
    pub mapped_system_va: ntdef::PVOID,
    pub start_va: ntdef::PVOID,
    pub byte_count: ntdef::ULONG,
    pub byte_offset: ntdef::ULONG,
}
const _: () = if core::mem::size_of::<MDL>() != 0x30 {
    panic!("Memory Descriptor List is not of size 0x30!");
};
