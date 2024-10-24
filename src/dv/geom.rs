use super::entity::{self, ENTITY};
use super::{common_, ffi_, logical_t};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[link(name = "differvoid")]
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

pub trait GEOM: ENTITY {
    fn copy(&self) -> common_::DVResult<GEOM_t> {
        let mut copy = entity::NULL;

        common_::wrap_result(unsafe { DV_GEOM_copy(self.tag(), &mut copy) }, || {
            copy.into()
        })
    }

    fn is_valid(&self) -> common_::DVResult<bool> {
        let mut is_valid = logical_t::FALSE;

        common_::wrap_result(
            unsafe { DV_GEOM_is_valid(self.tag(), &mut is_valid) },
            || logical_t::to_bool(is_valid),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GEOM_t(ffi_::DV_GEOM_t);

impl From<i32> for GEOM_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for GEOM_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl GEOM for GEOM_t {}
