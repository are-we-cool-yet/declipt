//! A collection of Windows API hooks.

#![allow(non_snake_case)]

use libscemu::emu::Emu;

pub fn trace_winapi_call(emu: &mut Emu, _ip_addr: u64, api_addr: u64) -> bool {
    let name = emu.api_addr_to_name(api_addr);
    return match name.as_str() {
        "ExAcquireFastMutex" => false,
        "ExReleaseFastMutex" => false,
        _ => true,
    }
}
