
use crate::utils::math;

pub struct Entity {
    base: *mut usize,
}

/// Note:
///     offsets are hardcoded as of 22/8/19
///     I need to get to it, calm down
impl Entity {

    pub unsafe fn from_raw(base: *mut usize) -> Self {
        Self {
            base,
        }
    }

    pub fn get_health(&self) -> i32 {
        unsafe {
            ((self.base as usize + 0x100) as *mut i32).read()
        }
    }

    pub fn get_armor(&self) -> i32 {
        unsafe {
            ((self.base as usize + 0xB340) as *mut i32).read()
        }
    }

    pub fn get_aim_punch(&self) -> math::vec::Vec3 {
        unsafe {
            ((self.base as usize + 0x302C) as *mut math::vec::Vec3).read()
        }
    }

    pub fn is_scoped(&self) -> bool {
        unsafe {
            ((self.base as usize + 0x3910) as *mut bool).read()
        }
    }

    pub fn get_team_num(&self) -> i32 {
        unsafe {
            ((self.base as usize + 0xF4) as *mut i32).read()
        }
    }

    pub fn get_origin(&self) -> math::vec::Vec3 {
        unsafe {
            ((self.base as usize + 0x138) as *mut math::vec::Vec3).read()
        }
    }

    pub fn get_velocity(&self) -> math::vec::Vec3 {
        unsafe {
            ((self.base as usize + 0x114) as *mut math::vec::Vec3).read()
        }
    }
}