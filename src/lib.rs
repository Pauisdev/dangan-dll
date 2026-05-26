use std::{
    ptr::null,
    thread::{self, sleep},
    time::Duration,
};

use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

const DLL_PROCESS_ATTACH: u32 = 1;
const _DLL_PROCESS_DETACH: u32 = 0;

#[unsafe(no_mangle)]
extern "system" fn DllMain(h_instance: u32, fwd_reason: u32, _: *mut core::ffi::c_void) {
    match fwd_reason {
        DLL_PROCESS_ATTACH => init(h_instance),
        _ => {}
    }
}

fn init(h_instance: u32) {
    //let dialogue_addr = 0x2FD2B0;
    //thread::spawn(move || do_hacky_wacky(h_instance));
    unsafe {
        let base_address = GetModuleHandleW(null()) as *mut u32;
        let pos_address = *base_address + 0x33cc80;
        let position_ptr = pos_address as *const Vec3;
        let position = *position_ptr;
        println!("Pos: {position:?}");
    }

    println!("Hello from Danganronpa!");
}

fn do_hacky_wacky(h_instance: u32) {
    /*loop {
        let addr = h_instance + 0x33cc80;
        let position_ptr = addr as *const Vec3;
        unsafe {
            let value = *position_ptr;
            println!("Position: {:?}", value);
        }
        sleep(Duration::from_secs(1));
    }*/
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
