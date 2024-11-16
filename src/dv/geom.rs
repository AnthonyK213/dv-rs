use crate::dv::{self, ENTITY};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[link(name = "differvoid")]
extern "C" {
    fn DV_GEOM_copy(geom: dv::DV_GEOM_t, copy: *mut dv::DV_GEOM_t) -> dv::DV_ERROR_code_t;

    fn DV_GEOM_is_valid(geom: dv::DV_GEOM_t, is_valid: *mut dv::LOGICAL_t) -> dv::DV_ERROR_code_t;
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
    fn copy(&self) -> dv::DVResult<dv::GEOM_t> {
        let mut copy = dv::entity::NULL;

        dv::common_::wrap_result(unsafe { DV_GEOM_copy(self.tag(), &mut copy) }, || {
            copy.into()
        })
    }

    fn is_valid(&self) -> dv::DVResult<bool> {
        let mut is_valid = dv::logical_t::FALSE;

        dv::common_::wrap_result(
            unsafe { DV_GEOM_is_valid(self.tag(), &mut is_valid) },
            || dv::logical_t::to_bool(is_valid),
        )
    }
}

impl From<i32> for dv::GEOM_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for dv::GEOM_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl GEOM for dv::GEOM_t {}
