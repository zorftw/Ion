
use winapi::{
    shared::minwindef::{HMODULE},
    ctypes::{c_void, c_char, c_int},
};

use crate::utils;
use crate::ion::sdk;

pub fn capture_interface(module: HMODULE, interface: *const u8) -> *const c_void {
    unsafe {
        let fn_addr = utils::native::get_proc_address(module, b"CreateInterface\0".as_ptr());

        let fn_capture_interface = std::mem::transmute::<*const c_void, extern "system" fn(*const c_char, *const c_int) -> *const c_void>(fn_addr);

        let interface_addr = fn_capture_interface(interface as _, std::ptr::null_mut());

        if !interface_addr.is_null() {
            println!("[capture_interface] captured {} at 0x{:X}", std::ffi::CStr::from_ptr(interface as _).to_str().unwrap(), interface_addr as usize);

            return interface_addr;
        }
    }
    std::ptr::null_mut()
}

#[derive(Debug)]
pub struct Interfaces {
    pub client: *mut usize,
    pub client_mode: *mut usize,
    pub vgui_surface: sdk::surface::Surface,
    pub vgui_panel: sdk::panel::Panel,
    pub entity_list: sdk::entitylist::EntityList,
    pub engine: sdk::engine::Engine,
    pub glow_object_manager: *const sdk::glow::glow_object_manager_t,
}

impl Default for Interfaces {
    fn default() -> Self {
        Self {
            client: std::ptr::null_mut(),
            client_mode: std::ptr::null_mut(),
            engine: sdk::engine::Engine::default(),
            glow_object_manager: std::ptr::null_mut(),
            vgui_panel: sdk::panel::Panel::default(),
            entity_list: sdk::entitylist::EntityList::default(),
            vgui_surface: sdk::surface::Surface::default(),
        }
    }
}

unsafe impl Send for Interfaces {}