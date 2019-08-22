
use winapi::ctypes::c_float;
use std::ops::Add;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec3 {
    x: c_float,
    y: c_float,
    z: c_float,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec2 {
    yaw: c_float,
    pitch: c_float,
}

impl Add for Vec3 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}