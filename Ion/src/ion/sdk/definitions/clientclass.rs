use crate::utils::math;

use std::os::raw::c_char;

type create_client_class_fn = unsafe extern "system" fn(ent: i32, serial: i32);
type create_event_fn = unsafe extern "system" fn();

#[derive(Clone)]
#[repr(C)]
pub struct ClientClass {
    create_client_class: create_client_class_fn,
    create_event: create_event_fn,
    network_name: *mut c_char,
    recv_table: *mut usize,
    next: *mut usize,
    class_id: i32,
}