use super::{triangle_t, xyz_t};
use std::ffi;

/* DV_POLY_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct DV_POLY_sf_t {
    i_offset: ffi::c_int,
    n_vertices: ffi::c_int,
    n_triangles: ffi::c_int,
    vertex: *mut xyz_t::PNT3D_t,
    triangle: *mut triangle_t::TRIANGLE_t,
}
