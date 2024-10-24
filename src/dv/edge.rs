use super::entity::{self, ENTITY};
use super::topol::TOPOL;
use super::{alias_, common_, curve, ffi_, interval_t, logical_t, vertex};
use std::ffi;

/* DV_EDGE_attach_curves_o_t */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct DV_EDGE_attach_curves_o_t {
    o_t_version: ffi::c_int,
    n_intervals: ffi::c_int,
    intervals: *const interval_t::INTERVAL_t,
    interval_map: *const ffi::c_int,
    have_senses: logical_t::LOGICAL_t,
    senses: *const logical_t::LOGICAL_t,
    copy_curves: ffi_::DV_GEOM_copy_t,
    vx_checking: ffi_::DV_check_vx_on_cu_t,
    geom_checking: ffi_::DV_check_geom_t,
}

extern "C" {
    fn DV_EDGE_ask_curve(
        edge: ffi_::DV_EDGE_t,
        curve: *mut ffi_::DV_CURVE_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_EDGE_ask_precision(
        edge: ffi_::DV_EDGE_t,
        precision: *mut ffi::c_double,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_EDGE_ask_vertices(
        edge: ffi_::DV_EDGE_t,
        vertices: *mut ffi_::DV_VERTEX_t,
    ) -> ffi_::DV_ERROR_code_t;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EDGE_t(ffi_::DV_EDGE_t);

impl From<i32> for EDGE_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for EDGE_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl TOPOL for EDGE_t {}

impl EDGE_t {
    pub fn ask_curve(&self) -> common_::DVResult<curve::CURVE_t> {
        let mut curve = entity::NULL;

        common_::wrap_result(unsafe { DV_EDGE_ask_curve(self.tag(), &mut curve) }, || {
            curve.into()
        })
    }

    pub fn ask_precision(&self) -> common_::DVResult<f64> {
        let mut precision = 0_f64;

        common_::wrap_result(
            unsafe { DV_EDGE_ask_precision(self.tag(), &mut precision) },
            || precision,
        )
    }

    pub fn ask_vertices(&self) -> common_::DVResult<[vertex::VERTEX_t; 2]> {
        let mut vertices = alias_::Int32Array::alloc(2);

        common_::wrap_result(
            unsafe { DV_EDGE_ask_vertices(self.0, vertices.as_mut_ptr()) },
            || [vertices[0].into(), vertices[1].into()],
        )
    }
}
