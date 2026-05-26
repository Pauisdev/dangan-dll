mod ui;

use std::{
    ptr::null,
    thread::{self, sleep},
    time::Duration,
};

use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

use crate::ui::APP;

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

fn init() {
    println!("Hello from Danganronpa!");
    thread::spawn(spawn_ui);
    let base_address = unsafe { GetModuleHandleW(null()) as u32 };
    let set_player_pos_fn_address = base_address + 0x33cc80;
    let fn_ptr = set_player_pos_fn_address as *const fn(x: f32, y: f32, z: f32, rotation: f32);
    let set_player_pos = unsafe { *fn_ptr };
    thread::spawn(move || read_position(base_address));
}

fn read_position(base_address: u32) {
    let pos_address = base_address + 0x33cc80;
    let position_ptr = pos_address as *const Vec3;
    loop {
        let position = unsafe { *position_ptr };
        APP.position.lock().unwrap().x = position.x;
        APP.position.lock().unwrap().y = position.y;
        APP.position.lock().unwrap().z = position.z;
        println!("Pos: {position:?}");
        sleep(Duration::from_secs(1));
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn one() -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}
