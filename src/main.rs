use error::Error;
use minhook::MinHook;

pub mod constants;
pub mod error;
pub mod hook;

fn main() -> Result<(), Error> {
    // SAFETY: It is assumed that the library is safe to load and that the platform supports calling functions via DLL offset.
    // It also assumes that Microsoft hasn't changed anything. If these conditions aren't met, god help you.

    let lib_path = std::fs::canonicalize("./maps64/ClipSp.sys.")?;

    unsafe {
        println!("Loading ClipSp.sys");
        let lib = libloading::os::windows::Library::load_with_flags(&lib_path, libloading::os::windows::LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR)?;
        let handle = lib.into_raw();

        // unload library
        let lib = libloading::os::windows::Library::from_raw(handle);
        lib.close()?;

        // Disable hooks just in case™
        MinHook::disable_all_hooks()?;
    }

    Ok(())
}
