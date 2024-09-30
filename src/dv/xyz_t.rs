use std::ffi;

/* DV_XYZ_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct XYZ_t {
    pub x: ffi::c_double,
    pub y: ffi::c_double,
    pub z: ffi::c_double,
}

impl Default for XYZ_t {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

/* DV_PNT3D_t */

pub type PNT3D_t = XYZ_t;

/* DV_VEC3D_t */

pub type VEC3D_t = XYZ_t;
