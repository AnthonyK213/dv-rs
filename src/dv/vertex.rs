use crate::dv::{self, ENTITY, TOPOL};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_VERTEX_ask_point(
        vertex: dv::DV_VERTEX_t,
        point: *mut dv::DV_POINT_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_VERTEX_ask_precision(
        vertex: dv::DV_VERTEX_t,
        precision: *mut ffi::c_double,
    ) -> dv::DV_ERROR_code_t;
}

impl From<i32> for dv::VERTEX_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for dv::VERTEX_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl TOPOL for dv::VERTEX_t {}

impl dv::VERTEX_t {
    pub fn ask_point(&self) -> dv::DVResult<dv::POINT_t> {
        let mut point = dv::entity::NULL;

        dv::common_::wrap_result(
            unsafe { DV_VERTEX_ask_point(self.tag(), &mut point) },
            || point.into(),
        )
    }

    pub fn ask_precision(&self) -> dv::DVResult<f64> {
        let mut precision = 0.0_f64;

        dv::common_::wrap_result(
            unsafe { DV_VERTEX_ask_precision(self.tag(), &mut precision) },
            || precision,
        )
    }
}
