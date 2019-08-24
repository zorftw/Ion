use std::mem::transmute;
use crate::utils;
use crate::ion::sdk::definitions;

#[derive(Debug)]
pub struct Client {
    pub base: *mut usize,
}

type get_all_classes_fn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> *const definitions::clientclass::ClientClass;

impl Client {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self {
            base: addr,
        }
    }

    pub fn get_all_classes(&self) -> *const definitions::clientclass::ClientClass {
        unsafe {
            transmute::<_, get_all_classes_fn>(utils::native::get_virtual_function(self.base,8))(self.base)
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}