mod ui;

use std::{
    mem,
    ptr::null,
    thread::{self, sleep},
    time::Duration,
};

use once_cell::sync::Lazy;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

use crate::{dr_funcs::render_debug_text, ui::APP};

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

pub mod dr_funcs {
    use std::{ffi::c_void, mem, os::raw::c_int};

    use crate::{Asd, BASE_ADDRESS, Vec3};

    pub fn set_player_pos(x: f32, y: f32, z: f32, rotation: f32) {
        let fn_address = *BASE_ADDRESS + 0x6c120;
        let fn_ptr = fn_address as *const ();
        let set_fn: extern "C" fn(f32, f32, f32, f32) = unsafe { mem::transmute(fn_ptr) };
        set_fn(x, y, z, rotation);
    }

    pub fn render_debug_text(x: i32, y: i32, text: String) {
        let fn_address = *BASE_ADDRESS + 0x33cc80;
        let fn_ptr = fn_address as *const Vec3;
        let _fn = unsafe { *fn_ptr };
        println!("WOW");
        println!("The value aa is: {:?}", fn_ptr);
        //_fn(); // crash
        println!("mmm");
    }
}

fn init() {
    println!("Hello from Danganronpa!");
    println!("The base address is: {}", *BASE_ADDRESS);
    println!("And the offset to the first BYTE of the function is: 0x135b0");
    println!(
        "After adding them up the address is: {}",
        *BASE_ADDRESS + 0x135b0
    );

    //render_debug_text(1, 2, String::new());
    //thread::spawn(read_byte);
    thread::spawn(spawn_ui);
    thread::spawn(read_position);
}

fn read_byte() {
    let fn_address = *BASE_ADDRESS + 0x135b0;
    let fn_ptr = fn_address as *const ();
    loop {
        let _fn: extern "C" fn() = unsafe { mem::transmute(fn_ptr) };
        //println!("value is: {}", _fn);
        _fn();
        println!("Called fn");
        sleep(Duration::from_secs(1));
    }
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Asd {
    x: u8,
    y: u8,
    z: u8,
}
