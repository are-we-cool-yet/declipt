use std::ffi;

use constants::offset_addr;
use error::Error;

pub mod constants;
pub mod error;
pub mod hook;

fn main() -> Result<(), Error> {
    // SAFETY: It is assumed that the library is safe to load and that the platform supports calling functions via DLL offset.
    // It also assumes that Microsoft hasn't changed anything. If these conditions aren't met, god help you.

    let lib_path = std::fs::canonicalize("./emu64/ClipSp.sys.")?;

    unsafe {
        println!("Loading ClipSp.sys");
        let lib = libloading::os::windows::Library::load_with_flags(&lib_path, libloading::os::windows::LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR)?;
        let handle = lib.into_raw();
        println!("0x{handle:X}");

        for &(rw_data, const_data, decrypt_fn_addr) in constants::DATA.iter() {
            let rw_data: *mut ffi::c_void = offset_addr(rw_data, handle);
            println!("0x{handle:X}");
            let const_data2 = offset_addr::<winapi::ctypes::__int64>(const_data, handle);
            println!("{const_data2:X?}");
            let const_data = *const_data2;
            let decrypt_fn_ptr = offset_addr(decrypt_fn_addr, handle);
            let decrypt_fn = std::mem::transmute::<*mut ffi::c_void, constants::WarbirdDecrypt>(decrypt_fn_ptr);
            println!("Decrypting");
            let decrypted = decrypt_fn(rw_data as _, const_data as *mut _);
            println!("0x{decrypted:X}");
        }

        // unload library
        let lib = libloading::os::windows::Library::from_raw(handle);
        lib.close()?;
    }

    Ok(())
}
