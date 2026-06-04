mod dangan_one;
mod ui;

use std::thread::{self};

use crate::dangan_one::Vec3;

const DLL_PROCESS_ATTACH: u32 = 1;
const _DLL_PROCESS_DETACH: u32 = 0;

#[unsafe(no_mangle)]
extern "system" fn DllMain(_h_instance: u32, fwd_reason: u32, _: *mut core::ffi::c_void) {
    match fwd_reason {
        DLL_PROCESS_ATTACH => init(),
        _ => {}
    }
}

fn init() {
    println!("Hello from Danganronpa!");
    dangan_one::setup_hook();
    println!("Hello?");
    thread::spawn(ui::spawn);
}
