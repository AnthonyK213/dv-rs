use std::ffi;

/* DV_TRIANGLE_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TRIANGLE_t {
    pub v0: ffi::c_int,
    pub v1: ffi::c_int,
    pub v2: ffi::c_int,
}
