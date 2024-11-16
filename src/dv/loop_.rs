use crate::dv::{self, ENTITY, TOPOL};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::ffi;

/* DV_LOOP_type_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum type_e {
    vertex_c = 5410,
    outer_c,
    inner_c,
}

#[link(name = "differvoid")]
extern "C" {
    fn DV_LOOP_ask_fins(
        loop_: dv::DV_LOOP_t,
        n_fins: *mut ffi::c_int,
        fins: *mut *mut dv::DV_FIN_t,
    ) -> dv::DV_ERROR_code_t;
}

impl From<i32> for dv::LOOP_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for dv::LOOP_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl TOPOL for dv::LOOP_t {}

impl dv::LOOP_t {
    pub fn ask_fins(&self) -> dv::DVResult<dv::FinArray> {
        let mut n_fins = 0_i32;
        let mut fins: *mut dv::DV_FIN_t = std::ptr::null_mut();

        dv::common_::wrap_result(
            unsafe { DV_LOOP_ask_fins(self.0, &mut n_fins, &mut fins) },
            || dv::array_::Array::new(fins, n_fins).into(),
        )
    }
}
