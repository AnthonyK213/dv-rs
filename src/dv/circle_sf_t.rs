use super::axis2_sf_t;
use std::ffi;

/* DV_CIRCLE_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CIRCLE_sf_t {
    basis_set: axis2_sf_t::AXIS2_sf_t,
    radius: ffi::c_double,
}
