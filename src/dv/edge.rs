use super::{array_, common_, ffi_, interval_t, logical_t, object};
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
    fn DV_EDGE_ask_curve(edge: ffi_::EDGE_t, curve: *mut ffi_::CURVE_t) -> ffi_::DV_ERROR_code_t;

    fn DV_EDGE_ask_precision(
        edge: ffi_::EDGE_t,
        precision: *mut ffi::c_double,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_EDGE_ask_vertices(
        edge: ffi_::EDGE_t,
        vertices: *mut ffi_::VERTEX_t,
    ) -> ffi_::DV_ERROR_code_t;
}

pub fn ask_curve(edge: ffi_::EDGE_t) -> common_::DVResult<ffi_::CURVE_t> {
    let mut curve = object::NULL;
    common_::wrap_result(unsafe { DV_EDGE_ask_curve(edge, &mut curve) }, || curve)
}

pub fn ask_precision(edge: ffi_::EDGE_t) -> common_::DVResult<f64> {
    let mut precision = 0_f64;

    common_::wrap_result(
        unsafe { DV_EDGE_ask_precision(edge, &mut precision) },
        || precision,
    )
}

pub fn ask_vertices(edge: ffi_::EDGE_t) -> common_::DVResult<array_::Int32Array> {
    let mut vertices = array_::Int32Array::alloc(2);

    common_::wrap_result(
        unsafe { DV_EDGE_ask_vertices(edge, vertices.as_mut_ptr()) },
        || vertices,
    )
}
