use super::object::{self, OBJECT};
use super::topol::TOPOL;
use super::{common_, ffi_, point};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_VERTEX_ask_point(
        vertex: ffi_::DV_VERTEX_t,
        point: *mut ffi_::DV_POINT_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_VERTEX_ask_precision(
        vertex: ffi_::DV_VERTEX_t,
        precision: *mut ffi::c_double,
    ) -> ffi_::DV_ERROR_code_t;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VERTEX_t(ffi_::DV_VERTEX_t);

impl From<i32> for VERTEX_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl OBJECT for VERTEX_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl TOPOL for VERTEX_t {}

impl VERTEX_t {
    pub fn ask_point(&self) -> common_::DVResult<point::POINT_t> {
        let mut point = object::NULL;

        common_::wrap_result(
            unsafe { DV_VERTEX_ask_point(self.tag(), &mut point) },
            || point.into(),
        )
    }

    pub fn ask_precision(&self) -> common_::DVResult<f64> {
        let mut precision = 0.0_f64;

        common_::wrap_result(
            unsafe { DV_VERTEX_ask_precision(self.tag(), &mut precision) },
            || precision,
        )
    }
}
