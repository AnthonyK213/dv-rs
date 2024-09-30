use super::{bsurf, common_, ffi_, logical_t};
use std::ffi;

/* DV_BSURF_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct DV_BSURF_sf_t {
    u_degree: ffi::c_int,
    v_degree: ffi::c_int,
    n_u_vertices: ffi::c_int,
    n_v_vertices: ffi::c_int,
    vertex_dim: ffi::c_int,
    is_rational: logical_t::LOGICAL_t,
    vertex: *mut ffi::c_double,
    form: ffi_::DV_BSURF_form_t,
    n_u_knots: ffi::c_int,
    n_v_knots: ffi::c_int,
    u_knot_mult: *mut ffi::c_int,
    v_knot_mult: *mut ffi::c_int,
    u_knot: *mut ffi::c_double,
    v_knot: *mut ffi::c_double,
    is_u_periodic: logical_t::LOGICAL_t,
    is_v_periodic: logical_t::LOGICAL_t,
    is_u_closed: logical_t::LOGICAL_t,
    is_v_closed: logical_t::LOGICAL_t,
}
