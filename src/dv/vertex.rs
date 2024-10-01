use super::{common_, ffi_, logical_t, object};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_VERTEX_ask_point(vertex: ffi_::VERTEX_t, point: *mut ffi_::POINT_t) -> ffi_::DV_CODE_t;

    fn DV_VERTEX_ask_precision(
        vertex: ffi_::VERTEX_t,
        precision: *mut ffi::c_double,
    ) -> ffi_::DV_CODE_t;
}

pub fn ask_point(vertex: ffi_::VERTEX_t) -> common_::DVResult<ffi_::POINT_t> {
    let mut point: ffi_::POINT_t = object::NULL;
    common_::wrap_result(unsafe { DV_VERTEX_ask_point(vertex, &mut point) }, || point)
}

pub fn ask_precision(vertex: ffi_::VERTEX_t) -> common_::DVResult<f64> {
    let mut precision = 0.0_f64;

    common_::wrap_result(
        unsafe { DV_VERTEX_ask_precision(vertex, &mut precision) },
        || precision,
    )
}
