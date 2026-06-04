use once_cell::sync::Lazy;
use std::ptr::null;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

type SetPlayerPosFn = extern "C" fn(f32, f32, f32, f32);
type LoadBustup = extern "C" fn(i32, i32, i32);

pub static BASE_ADDRESS: Lazy<u32> = Lazy::new(|| unsafe { GetModuleHandleW(null()) as u32 });
static mut ORIGINAL_SET_PLAYER_POS_FN: Option<SetPlayerPosFn> = None;
static mut ORIGINAL_LOAD_BUSTUP: Option<LoadBustup> = None;

use minhook::MinHook;
use std::{mem, os::raw::c_void};

pub fn setup_hook() {
    println!("Installing hooks...");
    unsafe {
        let set_player_pos_ptr = (*BASE_ADDRESS + 0x6c120) as *mut ();
        let hook = MinHook::create_hook(
            mem::transmute(set_player_pos_ptr),
            set_player_pos as *mut c_void,
        )
        .unwrap();
        ORIGINAL_SET_PLAYER_POS_FN = Some(mem::transmute(hook));
        let load_bustup_ptr = (*BASE_ADDRESS + 0x22b30) as *mut ();

        let hook =
            MinHook::create_hook(mem::transmute(load_bustup_ptr), load_bustup as *mut c_void)
                .unwrap();
        ORIGINAL_LOAD_BUSTUP = Some(mem::transmute(hook));
        MinHook::enable_all_hooks().unwrap();
    }
    println!("Hooks installed!");
}

pub extern "C" fn set_player_pos(x: f32, z: f32, unknown: f32, rotation: f32) {
    println!("Intercepted call! x={x}, z={z}");
    unsafe {
        if let Some(original) = ORIGINAL_SET_PLAYER_POS_FN {
            original(x, z, unknown, rotation);
        }
    }
}

pub extern "C" fn load_bustup(unk1: i32, unk2: i32, unk3: i32) {
    println!("Intercepted call from load_bustup: {unk1}, {unk2}, {unk3}");

    unsafe {
        if let Some(original) = ORIGINAL_LOAD_BUSTUP {
            original(unk1, unk2, unk3);
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
