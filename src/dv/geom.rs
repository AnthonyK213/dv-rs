use super::{common_, ffi_, logical_t, object};
use num_enum::{IntoPrimitive, TryFromPrimitive};

extern "C" {
    fn DV_GEOM_copy(geom: ffi_::GEOM_t, copy: *mut ffi_::GEOM_t) -> ffi_::DV_ERROR_code_t;

    fn DV_GEOM_is_valid(
        geom: ffi_::GEOM_t,
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

pub fn copy(geom: ffi_::GEOM_t) -> common_::DVResult<ffi_::GEOM_t> {
    let mut copy = object::NULL;
    common_::wrap_result(unsafe { DV_GEOM_copy(geom, &mut copy) }, || copy)
}

pub fn is_valid(geom: ffi_::GEOM_t) -> common_::DVResult<bool> {
    let mut is_valid = logical_t::FALSE;

    common_::wrap_result(unsafe { DV_GEOM_is_valid(geom, &mut is_valid) }, || {
        logical_t::to_bool(is_valid)
    })
}
