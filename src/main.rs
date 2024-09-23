use std::{cell::RefCell, ffi, ops::Deref, path::Path, sync::{mpsc, LazyLock}, thread};
use error::Error;
use minhook::MinHook;
use pretty_hex::config_hex;
use util::offset_addr;

pub mod constants;
pub mod error;
pub mod hook;
pub mod util;
pub mod types;

thread_local! {
    pub static DECRYPT_RX: RefCell<Option<mpsc::Receiver<types::DecryptMessage>>> = RefCell::new(None);
}

pub static DECRYPT_TX: LazyLock<mpsc::SyncSender<types::DecryptMessage>> = LazyLock::new(|| {
    let (tx, rx) = mpsc::sync_channel(constants::DATA.len());
    DECRYPT_RX.set(Some(rx));
    tx
});

fn main() -> Result<(), Error> {
    // SAFETY: It is assumed that the library is safe to load and that the platform supports calling functions via DLL offset.
    // It also assumes that Microsoft hasn't changed anything. If these conditions aren't met, god help you.

    let mut lib_path = std::fs::canonicalize(
        if Path::new(constants::CLIPSP).exists() {
            constants::CLIPSP
        } else if Path::new(constants::DEBUG_CLIPSP).exists() {
            constants::DEBUG_CLIPSP
        } else {
            panic!("emu64 not found! Read the directions in README.md.");
        }
    )?;
    let mut data_dir = lib_path.clone();
    data_dir.push("data");
    std::fs::create_dir(data_dir.clone())?;
    lib_path.push("ClipSp.sys");

    unsafe {
        println!("Loading ClipSp.sys");
        let lib = libloading::os::windows::Library::load_with_flags(&lib_path, libloading::os::windows::LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR)?;
        let handle = lib.into_raw();

        // hook ntoskrnl functions
        create_hooks_with_handle! { handle:
            MmChangeImageProtection;
            IoAllocateMdl;
            IoFreeMdl;
            MmProbeAndLockPages;
            MmUnlockPages;
            MmLockPagableDataSection;
            MmMapLockedPagesSpecifyCache;
            MmUnmapLockedPages;
        };

        MinHook::enable_all_hooks()?;

        let _ = DECRYPT_TX.deref();

        let thread_handle = thread::spawn(move || {
            // Call decryption functions
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
                let decrypt_fn = std::mem::transmute::<*mut ffi::c_void, types::WarbirdDecrypt>(decrypt_fn_ptr);
                println!("Decrypting rw_data (0x{rw_data:X}) and const_data (0x{const_data:X}) w/ 0x{decrypt_fn_addr:X}");
                let decrypted = decrypt_fn(const_data_ptr as _, rw_data_ptr as *mut _);
                println!("Error Code: 0x{decrypted:X}");
            }
        });

        // Receive decrypted data
        let datas = constants::DATA
            .iter()
            .map(|_| {
                DECRYPT_RX.with_borrow(|rx| {
                    rx.as_ref().unwrap().recv()
                })
            })
            .collect::<Result<Vec<Vec<_>>, _>>()?;

        if constants::PRINT_DATA {
            datas
                .iter()
                .for_each(|data| {
                    println!("{}", config_hex(data, constants::HEX_CONFIG));
                });
        }

        datas
            .iter()
            .enumerate()
            .try_for_each::<_, Result<_, Error>>(|(i, data)| {
                let mut data_file_path = data_dir.clone();
                data_file_path.push(format!("data_{i}.bin"));
                std::fs::write(data_file_path, data)?;
                Ok(())
            })?;

        thread_handle.join().expect("couldn't join thread");

        // uninitialize hooks
        MinHook::disable_all_hooks()?;
        MinHook::uninitialize();

        // unload library
        let lib = libloading::os::windows::Library::from_raw(handle);
        lib.close()?;
    }

    Ok(())
}
