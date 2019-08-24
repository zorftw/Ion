
use crate::utils::math;
use crate::ion::sdk::netvar;
use std::os::raw::c_char;
use std::ffi::{CString, CStr};
use crate::utils::math::vec::Vec3;

pub struct c_entity {
    base: *mut usize,
}

/// Note:
///     offsets are hardcoded as of 22/8/19
///     I need to get to it, calm down
impl c_entity {

    pub fn get_value<T>(&self, offset: usize) -> T {
        unsafe {
           ((self.base as usize + offset) as *mut T).read()
        }
    }

    pub unsafe fn from_raw(base: *mut usize) -> Self {
        Self {
            base,
        }
    }

    pub fn get_health(&self) -> i32 {
        self.get_value(netvar::get_offset("DT_CSPlayer", "m_iHealth"))
    }

    pub fn get_armor(&self) -> i32 {
        self.get_value(netvar::get_offset("DT_CSPlayer", "m_ArmorValue"))
    }

    pub fn get_aim_punch(&self) -> math::vec::Vec3 {
        self.get_value(netvar::get_offset("DT_CSPlayer", "m_aimPunchAngle"))
    }

    pub fn is_scoped(&self) -> bool {
        self.get_value(netvar::get_offset("DT_CSPlayer", "m_bIsScoped"))
    }

    pub fn is_defusing(&self) -> bool {
        self.get_value(netvar::get_offset("DT_CSPlayer", "m_bIsDefusing"))
    }

    pub fn get_team_num(&self) -> i32 {
        self.get_value(netvar::get_offset("DT_CSPlayer", "m_iTeamNum"))
    }

    pub unsafe fn get_origin(&self) -> math::vec::Vec3 {
        ((self.base as usize + netvar::get_offset("DT_CSPlayer", "m_vecOrigin")) as *mut Vec3).read()
    }

    pub fn get_velocity(&self) -> math::vec::Vec3 {
        self.get_value(netvar::get_offset("DT_CSPlayer", "m_vecVelocity"))
    }

    pub fn get_name(&self) -> String {
        let name: [c_char; 260] = self.get_value(netvar::get_offset("DT_CSPlayer", "m_iName"));
        unsafe { CStr::from_ptr(name.as_ptr()).to_str().unwrap().to_owned() }
    }
}