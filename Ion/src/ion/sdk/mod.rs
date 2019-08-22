/**
    Consider this a 'hub' of sorts of modules that are used to change / access
    structures / classes that are from CS:GO itself.
*/

pub mod glow;
pub mod engine;
pub mod surface;
pub mod panel;
pub mod entitylist;

pub mod interfaces;
pub mod definitions;
pub mod hook;

use crate::ion;

pub fn get_local_player() -> definitions::entity::Entity {
    let local_id = ion::interfaces.lock().unwrap().engine.get_local_player();

    unsafe {
        definitions::entity::Entity::from_raw(ion::interfaces.lock().unwrap().entity_list.get_entity_by_id(local_id))
    }
}