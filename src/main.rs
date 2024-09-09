use libscemu::emu64;

pub mod constants;
pub mod hook;
pub mod util;

fn main() {
    let mut emu = emu64();
    emu.set_maps_folder("./maps64/");
    emu.init();

    emu.load_code("./ClipSp.sys");

    emu.set_rip(0x1C00BFD95, false);

    emu.run(None).unwrap();
}
