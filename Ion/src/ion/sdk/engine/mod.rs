
type get_screen_size_fn = unsafe extern "thiscall" fn(thisptr: *mut usize, width: &mut i32, height: &mut i32);

#[derive(Debug)]
pub struct Engine {
    base: *mut usize,
}

use crate::utils;

impl Engine {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self {
            base: addr,
        }
    }

    pub fn get_screen_size(&self, width: &mut i32, height: &mut i32) {
        let vfunc = unsafe { std::mem::transmute::<_, get_screen_size_fn>(utils::native::get_virtual_function(self.base, 5))};
        unsafe { vfunc(self.base, width, height); }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}