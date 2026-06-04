mod dangan_one;
mod ui;

use std::{
    ptr::null,
    thread::{self, sleep},
    time::Duration,
};

use once_cell::sync::Lazy;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

use crate::{dangan_one::Vec3, ui::APP};

const DLL_PROCESS_ATTACH: u32 = 1;
const _DLL_PROCESS_DETACH: u32 = 0;

#[unsafe(no_mangle)]
extern "system" fn DllMain(_h_instance: u32, fwd_reason: u32, _: *mut core::ffi::c_void) {
    match fwd_reason {
        DLL_PROCESS_ATTACH => init(),
        _ => {}
    }
}

fn spawn_ui() {
    ui::show().expect("Failed to start UI")
}

pub static BASE_ADDRESS: Lazy<u32> = Lazy::new(|| unsafe { GetModuleHandleW(null()) as u32 });

fn init() {
    println!("Hello from Danganronpa!");
    thread::spawn(spawn_ui);
    thread::spawn(read_position);
}

fn read_position() {
    let pos_address = *BASE_ADDRESS + 0x33cc80;
    let position_ptr = pos_address as *const Vec3;
    loop {
        let position = unsafe { *position_ptr };
        APP.position.lock().unwrap().x = position.x;
        APP.position.lock().unwrap().y = position.y;
        APP.position.lock().unwrap().z = position.z;
        sleep(Duration::from_secs(1));
    }
}
