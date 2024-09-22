#![feature(macro_metavar_expr)]

use std::{ffi, path::Path};

use constants::offset_addr;
use error::Error;
use minhook::MinHook;

pub mod constants;
pub mod error;
pub mod hook;

fn main() -> Result<(), Error> {
    // SAFETY: It is assumed that the library is safe to load and that the platform supports calling functions via DLL offset.
    // It also assumes that Microsoft hasn't changed anything. If these conditions aren't met, god help you.

    let lib_path = std::fs::canonicalize(
        if Path::new(constants::CLIPSP).exists() {
            constants::CLIPSP
        } else if Path::new(constants::DEBUG_CLIPSP).exists() {
            constants::DEBUG_CLIPSP
        } else {
            panic!("emu64/ClipSp.sys not found! Read the directions in README.md.");
        }
    )?;

    unsafe {
        println!("Loading ClipSp.sys");
        let lib = libloading::os::windows::Library::load_with_flags(&lib_path, libloading::os::windows::LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR)?;
        let handle = lib.into_raw();
        println!("0x{handle:X}");

        // hook ntoskrnl functions
        create_hooks_with_handle! { handle:
            IoAllocateMdl;
            IoFreeMdl;
            MmProbeAndLockPages;
            MmUnlockPages;
            MmLockPagableDataSection;
            MmMapLockedPagesSpecifyCache;
            MmUnmapLockedPages;
        };

        MinHook::enable_all_hooks()?;

        for &(const_data, rw_data, decrypt_fn_addr) in constants::DATA.iter() {
            let rw_data_ptr: *mut ffi::c_void = offset_addr(rw_data, handle);
            let const_data_ptr = offset_addr::<winapi::ctypes::__int64>(const_data, handle);
            if *((const_data_ptr.byte_offset(0x50)) as *mut winapi::shared::minwindef::DWORD) & 1 == 0 {
                println!("Oops! Something is wrong with the Const Data provided.");
                println!("const_data + 0x50    0x{:X}", const_data_ptr.byte_offset(0x50) as usize);
                println!("*(DWORD *)(const_data + 0x50)    0x{:X}", *(const_data_ptr.byte_offset(0x50) as *mut winapi::shared::minwindef::DWORD));
                println!("*(DWORD *)(const_data + 0x50) & 1    0x{:X}", *(const_data_ptr.byte_offset(0x50) as *mut winapi::shared::minwindef::DWORD) & 1);
            }
            let decrypt_fn_ptr = offset_addr(decrypt_fn_addr, handle);
            let decrypt_fn = std::mem::transmute::<*mut ffi::c_void, constants::WarbirdDecrypt>(decrypt_fn_ptr);
            println!("Decrypting rw_data (0x{rw_data:X}) and const_data (0x{const_data:X}) w/ 0x{decrypt_fn_addr:X}");
            let decrypted = decrypt_fn(const_data_ptr as _, rw_data_ptr as *mut _);
            println!("Decrypted address: 0x{decrypted:X}");
        }

        // uninitialize hooks
        MinHook::disable_all_hooks()?;
        MinHook::uninitialize();

        // unload library
        let lib = libloading::os::windows::Library::from_raw(handle);
        lib.close()?;
    }

    Ok(())
}
