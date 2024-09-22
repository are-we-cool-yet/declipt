//! A collection of important constants.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use winapi::shared::{minwindef, ntdef};

// the least retarded shit
pub const CLIPSP: &'static str = "./emu64/ClipSp.sys";
pub const DEBUG_CLIPSP: &'static str = "../../emu64/ClipSp.sys";

const fn from_base(addr: usize) -> usize {
    addr - DLL_BASE
}

// Addresses
/// Encryption data (read-write data, const data, decrypt function)
pub const DATA: &[(usize, usize, usize)] = &[(0x1C00A1E10, 0x1C00AA8E0, 0x1C0001158)];
/// The base address of the DLL.
pub const DLL_BASE: usize = 0x1C0000000;

pub const unsafe fn offset_addr<T>(ptr: usize, offset: isize) -> *mut T {
    (from_base(ptr) as *mut T).byte_offset(offset)
}

/// A less cursed-ass macro that creates hooks.
#[macro_export]
macro_rules! create_hooks_with_handle {
    { $handle:ident: $( $i:ident; )+ } => {
        $( minhook::MinHook::create_hook(*$crate::constants::$i($handle), $crate::hook::$i as _)? );+
    };
}

// Imported Function Addresses
macro_rules! fn_addr {
    ( $i:ident, $offset:literal ) => {
        pub const unsafe fn $i<T>(offset: isize) -> *mut T {
            $crate::constants::offset_addr($offset, offset)
        }
    };
}
fn_addr!(MmChangeImageProtection, 0x1C00B13F0);
fn_addr!(IoAllocateMdl, 0x1C00B13F8);
fn_addr!(IoFreeMdl, 0x1C00B13E8);
fn_addr!(MmProbeAndLockPages, 0x1C00B1420);
fn_addr!(MmUnlockPages, 0x1C00B1428);
fn_addr!(MmLockPagableDataSection, 0x1C00B1408);
fn_addr!(MmMapLockedPagesSpecifyCache, 0x1C00B13C8);
fn_addr!(MmUnmapLockedPages, 0x1C00B13D0);

// DLL Flags
/// https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw
pub const DONT_RESOLVE_DLL_REFERENCES: u32 = 0x00000001;

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
#[repr(C)]
pub struct Mdl {
    pub virtual_address: ntdef::PVOID,
    pub length: minwindef::ULONG,
    pub _pad: [u8; 0x20],
}
const _: () = if core::mem::size_of::<Mdl>() != 0x30 {
    panic!("Memory Descriptor List is not of size 0x30!");
};

/// Relevant addresses where decryption occurs.
/// DO NOT CHANGE THESE. THESE DO NOTHING.
pub const UNUSED_ADDRESSES: [(usize, usize); 66] = [(0x1C003295F, 0x1C003297F), (0x1C00BA995, 0x1C00BA9AA), (0x1C00BA9CB, 0x1C00BA9E0), (0x1C00BAB38, 0x1C00BAB5E), (0x1C00BBA30, 0x1C00BBA4A), (0x1C00BBF24, 0x1C00BBF3E), (0x1C00BC001, 0x1C00BC029), (0x1C00BDF39, 0x1C00BDF4E), (0x1C00BDF54, 0x1C00BDF69), (0x1C00BE102, 0x1C00BE117), (0x1C00BE119, 0x1C00BE12E), (0x1C00BE244, 0x1C00BE259), (0x1C00BE32C, 0x1C00BE341), (0x1C00BE382, 0x1C00BE397), (0x1C00BE549, 0x1C00BE564), (0x1C00BE73E, 0x1C00BE759), (0x1C00BE96D, 0x1C00BE985), (0x1C00BEBB5, 0x1C00BEBD8), (0x1C00BED25, 0x1C00BED40), (0x1C00BEF4E, 0x1C00BEF69), (0x1C00BF124, 0x1C00BF139), (0x1C00BF1BA, 0x1C00BF1D7), (0x1C00BF29D, 0x1C00BF2B7), (0x1C00BF378, 0x1C00BF38F), (0x1C00BF54F, 0x1C00BF56A), (0x1C00BF8D5, 0x1C00BF8ED), (0x1C00BF9FF, 0x1C00BFA23), (0x1C00BFB84, 0x1C00BFB99), (0x1C00BFD95, 0x1C00BFDAA), (0x1C00BFDAC, 0x1C00BFDC1), (0x1C00BFE72, 0x1C00BFE87), (0x1C00BFFED, 0x1C00C0004), (0x1C00C00E8, 0x1C00C00FE), (0x1C00C0341, 0x1C00C0359), (0x1C00C052F, 0x1C00C0544), (0x1C00C05E1, 0x1C00C05F9), (0x1C00C074D, 0x1C00C0767), (0x1C00C1D9B, 0x1C00C1DAE), (0x1C00BA9B0, 0x1C00BA9C5), (0x1C00BAE2C, 0x1C00BAE41), (0x1C00BBA50, 0x1C00BBA65), (0x1C00BBF40, 0x1C00BBF55), (0x1C00BC02F, 0x1C00BC044), (0x1C00BC248, 0x1C00BC261), (0x1C00BE25B, 0x1C00BE270), (0x1C00BE347, 0x1C00BE35C), (0x1C00BE56A, 0x1C00BE57F), (0x1C00BE75F, 0x1C00BE774), (0x1C00BE98B, 0x1C00BE9A0), (0x1C00BEBDE, 0x1C00BEBF3), (0x1C00BED46, 0x1C00BED5B), (0x1C00BEF6F, 0x1C00BEF84), (0x1C00BF13B, 0x1C00BF150), (0x1C00BF1DD, 0x1C00BF1F2), (0x1C00BF2B9, 0x1C00BF2CE), (0x1C00BF570, 0x1C00BF585), (0x1C00BF8F3, 0x1C00BF908), (0x1C00BFA29, 0x1C00BFA3E), (0x1C00BFB9F, 0x1C00BFBB4), (0x1C00C000E, 0x1C00C0023), (0x1C00C011E, 0x1C00C0133), (0x1C00C035B, 0x1C00C0370), (0x1C00C04FC, 0x1C00C0516), (0x1C00C0518, 0x1C00C052D), (0x1C00C05FB, 0x1C00C0610), (0x1C00C0769, 0x1C00C077E)];
