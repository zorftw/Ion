
pub struct Entity {
    base: *mut usize,
}

impl Entity {

    pub unsafe fn from_raw(base: *mut usize) -> Self {
        Self {
            base,
        }
    }
}