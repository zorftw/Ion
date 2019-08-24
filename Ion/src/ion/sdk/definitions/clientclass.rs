use std::os::raw::c_char;

use crate::ion::sdk::definitions::recvprop;

type create_client_class_fn = unsafe extern "system" fn(ent: i32, serial: i32);
type create_event_fn = unsafe extern "system" fn();

#[derive(Clone)]
#[repr(C)]
pub struct ClientClass {
    create_client_class: create_client_class_fn,
    create_event: create_event_fn,
    network_name: *mut c_char,
    pub recv_table: *mut recvprop::c_recv_table,
    pub next: *mut usize,
    pub class_id: i32,
}