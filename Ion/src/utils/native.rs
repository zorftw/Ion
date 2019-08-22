
/**
    Some simple wrappers
*/

use winapi::{
    shared::minwindef::HMODULE,
    um::libloaderapi::{GetModuleHandleA, GetProcAddress},
    um::winuser::{MessageBoxA},
    ctypes::c_void,
};

pub fn get_module_handle(name: *const u8) -> HMODULE {
    unsafe { return GetModuleHandleA(name as _) }
}

pub fn message_box(text: *const u8) {
    unsafe { MessageBoxA(std::ptr::null_mut(), text as _, b"Ion\0".as_ptr() as _, 0); }
}

pub fn get_proc_address(module: HMODULE, name: *const u8) -> *const c_void {
    unsafe { return GetProcAddress(module, name as _) as _}
}

pub fn get_virtual_function(base: *mut usize, idx: isize) -> *mut usize {
    unsafe {
        /* credits to cyanidee on github */
        let vt = *base as *mut usize;
        let vfn = vt.offset(idx).read() as *mut usize;

        return vfn;
    }
    std::ptr::null_mut()
}