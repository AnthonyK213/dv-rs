use std::ffi;

/* DV_XY_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct XY_t {
    pub x: ffi::c_double,
    pub y: ffi::c_double,
}

impl Default for XY_t {
    fn default() -> Self {
        Self { x: 0., y: 0. }
    }
}

/* DV_PNT2D_t */

pub type PNT2D_t = XY_t;

/* DV_VEC2D_t */

pub type VEC2D_t = XY_t;

/* DV_UV_t */

pub type UV_t = XY_t;
