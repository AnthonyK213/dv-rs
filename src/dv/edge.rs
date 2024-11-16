use crate::dv::{self, ENTITY, TOPOL};
use std::ffi;

extern "C" {
    fn DV_EDGE_ask_curve(edge: dv::DV_EDGE_t, curve: *mut dv::DV_CURVE_t) -> dv::DV_ERROR_code_t;

    fn DV_EDGE_ask_precision(
        edge: dv::DV_EDGE_t,
        precision: *mut ffi::c_double,
    ) -> dv::DV_ERROR_code_t;

    fn DV_EDGE_ask_vertices(
        edge: dv::DV_EDGE_t,
        vertices: *mut dv::DV_VERTEX_t,
    ) -> dv::DV_ERROR_code_t;
}

impl From<i32> for dv::EDGE_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for dv::EDGE_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl TOPOL for dv::EDGE_t {}

impl dv::EDGE_t {
    pub fn ask_curve(&self) -> dv::DVResult<dv::CURVE_t> {
        let mut curve = dv::entity::NULL;

        dv::common_::wrap_result(unsafe { DV_EDGE_ask_curve(self.tag(), &mut curve) }, || {
            curve.into()
        })
    }

    pub fn ask_precision(&self) -> dv::DVResult<f64> {
        let mut precision = 0_f64;

        dv::common_::wrap_result(
            unsafe { DV_EDGE_ask_precision(self.tag(), &mut precision) },
            || precision,
        )
    }

    pub fn ask_vertices(&self) -> dv::DVResult<[dv::VERTEX_t; 2]> {
        let mut vertices = dv::Int32Array::alloc(2);

        dv::common_::wrap_result(
            unsafe { DV_EDGE_ask_vertices(self.0, vertices.as_mut_ptr()) },
            || [vertices[0].into(), vertices[1].into()],
        )
    }
}
