use once_cell::sync::Lazy;
use std::ptr::null;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

pub static BASE_ADDRESS: Lazy<u32> = Lazy::new(|| unsafe { GetModuleHandleW(null()) as u32 });

pub mod dr_funcs {
    use crate::BASE_ADDRESS;
    use std::mem;

    pub fn set_player_pos(x: f32, z: f32, unknown: f32, rotation: f32) {
        let fn_address = *BASE_ADDRESS + 0x6c120;
        let fn_ptr = fn_address as *const ();
        let set_fn: extern "C" fn(f32, f32, f32, f32) = unsafe { mem::transmute(fn_ptr) };
        set_fn(x, z, unknown, rotation);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
