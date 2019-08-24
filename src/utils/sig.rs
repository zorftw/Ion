
/**
    Signature utilities
*/

use winapi::{
    shared::minwindef::{HMODULE, DWORD},
    um::winnt::{PIMAGE_DOS_HEADER, PIMAGE_NT_HEADERS},
    ctypes::{c_char, c_long},
};

use std::str;
use std::i32;

fn pattern_to_bytes(pattern: String) -> Vec<i32> {
    let mut res = vec![];

    let pattern_replaced = pattern.replace(' ', "");
    let subs = pattern_replaced.as_bytes()
        .chunks(2)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    subs.into_iter().for_each(|q| {
        if q.contains('?') {
            res.push(-1);
        } else {
            res.push(i32::from_str_radix(q, 16).unwrap());
        }
    });

    res
}

pub fn pattern_scan(module: HMODULE, sig: &str) -> *mut u8 {
    unsafe {
        let dos_headers = module as PIMAGE_DOS_HEADER;

        let module_addr = module as usize;
        let e_lfanew = (*dos_headers).e_lfanew as c_long;

        let nt_headers = (module_addr + e_lfanew as usize) as PIMAGE_NT_HEADERS;

        let size_of_image = (*nt_headers).OptionalHeader.SizeOfImage as usize;
        let pattern_bytes = pattern_to_bytes(sig.to_owned());
        let bytes = module as *mut u8;

        let size = pattern_bytes.len();

        for i in 0..(size_of_image - size as usize) {
            let mut found = true;
            for j in 0..size {
                if *bytes.offset(i as isize + j as isize) != pattern_bytes[j] as _ && pattern_bytes[j] != -1 {
                    found = false;
                    break;
                }
            }

            if found {
                return bytes.offset(i as _) as _;
            }
        }
    }

    0 as *mut _
}