//! A collection of hooks and patches.
#![allow(non_snake_case)]

use std::cell::RefCell;

use winapi::{shared::{minwindef, ntdef}, um::{errhandlingapi::GetLastError, memoryapi::VirtualProtectEx, processthreadsapi::GetCurrentProcess, winnt::PAGE_READWRITE}};

use crate::{constants, manually_drop, types::{KPROCESSOR_MODE, LOCK_OPERATION, MDL, MEMORY_CACHING_TYPE, QWORD}};

thread_local! {
    static MDL_LIST: RefCell<Vec<MDL>> = RefCell::new(vec![]);
}

/// Replace the first six bytes of the main entrypoint with these bytes.
/// Do note that there are multiple entrypoints; you want the one that is called upon driver initialization (the "true" entrypoint).
/// ```asm
/// mov eax, 1
/// ret
/// ```
#[allow(unused)]
pub const CANCEL_DRIVER_ENTRY: &'static [u8] = &[0xB8, 0x01, 0x00, 0x00, 0x00, 0xC3];

pub unsafe extern "fastcall" fn MmChangeImageProtection(_arg0: QWORD, _arg1: QWORD, _arg2: QWORD, _arg3: QWORD) -> winapi::ctypes::__int64 {
    println!("MmChangeImageProtection");
    minwindef::TRUE as _
}

pub unsafe extern "stdcall" fn IoAllocateMdl(virtual_address: ntdef::PVOID, length: ntdef::ULONG, _secondary_buffer: ntdef::BOOLEAN, _charge_quota: ntdef::BOOLEAN, irp: ntdef::PVOID) -> *mut MDL {
    print!("IoAllocateMdl (VA @ 0x{:X})    ", virtual_address as usize);
    if !irp.is_null() {
        panic!("Non-null IRP found! Non-null IRPs are unsupported.");
    }

    let process_handle = GetCurrentProcess();
    let mut old_protect = 0;
    let mapped_virtual_address = VirtualProtectEx(process_handle, virtual_address, length as usize, PAGE_READWRITE, &mut old_protect);
    if mapped_virtual_address == minwindef::FALSE {
        let error = GetLastError();
        panic!("Failed to allocate memory @ 0x{:X}\nError Code: 0x{error:X}\nProcess Handle: 0x{:X}", virtual_address as usize, process_handle as isize);
    }
    let next_mdl = MDL_LIST.with_borrow_mut(|list| list.last_mut().map(|x| x as *mut MDL).unwrap_or(core::ptr::null_mut()));
    let mdl = MDL {
        next: manually_drop!(next_mdl),
        size: length as _,
        mdl_flags: constants::MDL_MAPPED_TO_SYSTEM_VA,
        process: manually_drop!(*const constants::EPROCESS),
        mapped_system_va: virtual_address,
        start_va: virtual_address,
        byte_count: length,
        byte_offset: 0,
    };
    MDL_LIST.with_borrow_mut(|list| list.push(mdl));
    let mdl_ptr = MDL_LIST.with_borrow_mut(|list| list.as_mut_ptr_range().end.byte_offset(-(core::mem::size_of::<MDL>() as isize)));
    println!("Allocated MDL (MDL @ 0x{:X})", mdl_ptr as *const _ as usize);
    mdl_ptr as *mut MDL
}

pub unsafe extern "stdcall" fn IoFreeMdl(mdl: *mut MDL) {
    println!("IoFreeMdl (MDL @ 0x{:X})", mdl as usize);
    assert!(!mdl.is_null());
    MDL_LIST.with_borrow_mut(|list| list.remove(list.iter().position(|x| x as *const _ == mdl).unwrap()));
}

pub unsafe extern "stdcall" fn MmProbeAndLockPages(memory_descriptor_list: *mut MDL, _access_mode: KPROCESSOR_MODE, _operation: LOCK_OPERATION) {
    println!("MmProbeAndLockPages (MDL @ {:X})", memory_descriptor_list as usize);
    (*memory_descriptor_list).mdl_flags |= constants::MDL_PAGES_LOCKED;
}

pub unsafe extern "stdcall" fn MmUnlockPages(memory_descriptor_list: *mut MDL) {
    print!("MmUnlockPages (MDL @ 0x{:X})    ", memory_descriptor_list as usize);
    print!("MDL Flags Before 0x{:X}    ", (*memory_descriptor_list).mdl_flags);
    (*memory_descriptor_list).mdl_flags ^= constants::MDL_PAGES_LOCKED;
    println!("MDL Flags After 0x{:X}", (*memory_descriptor_list).mdl_flags);
}

pub unsafe extern "stdcall" fn MmLockPagableDataSection(address_within_section: ntdef::PVOID) -> ntdef::PVOID {
    println!("MmLockPagableDataSection (0x{:X})", address_within_section as usize);
    address_within_section
}

pub unsafe extern "stdcall" fn MmMapLockedPagesSpecifyCache(_memory_descriptor_list: *mut MDL, _access_mode: KPROCESSOR_MODE, _cache_type: MEMORY_CACHING_TYPE) -> ntdef::PVOID {
    println!("MmMapLockedPagesSpecifyCache");
    0x0 as _
}

pub unsafe extern "stdcall" fn MmUnmapLockedPages(_base_address: ntdef::PVOID, _memory_descriptor_list: *mut MDL) {
    println!("MmUnmapLockedPages");
}
