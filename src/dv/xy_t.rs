use std::ffi;

/* DV_XY_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct XY_t {
    pub x: ffi::c_double,
    pub y: ffi::c_double,
}
