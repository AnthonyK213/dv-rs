use std::ffi;

/* DV_INTERVAL_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct INTERVAL_t {
    pub t0: ffi::c_double,
    pub t1: ffi::c_double,
}
