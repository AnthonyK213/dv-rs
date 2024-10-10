use super::object::{self, OBJECT_t};
use super::{common_, ffi_, logical_t};
use num_enum::{IntoPrimitive, TryFromPrimitive};

extern "C" {
    fn DV_GEOM_copy(geom: ffi_::DV_GEOM_t, copy: *mut ffi_::DV_GEOM_t) -> ffi_::DV_ERROR_code_t;

    fn DV_GEOM_is_valid(
        geom: ffi_::DV_GEOM_t,
        is_valid: *mut logical_t::LOGICAL_t,
    ) -> ffi_::DV_ERROR_code_t;
}

/* DV_GEOM_copy_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum copy_e {
    always_c = 24770,
    never_c,
    auto_c,
}

pub trait GEOM_t: OBJECT_t {
    fn copy(&self) -> common_::DVResult<i32> {
        let mut copy = object::NULL;
        common_::wrap_result(unsafe { DV_GEOM_copy(self.tag(), &mut copy) }, || copy)
    }

    fn is_valid(&self) -> common_::DVResult<bool> {
        let mut is_valid = logical_t::FALSE;

        common_::wrap_result(
            unsafe { DV_GEOM_is_valid(self.tag(), &mut is_valid) },
            || logical_t::to_bool(is_valid),
        )
    }
}
