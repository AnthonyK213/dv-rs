use std::ffi;

/* DV_TRANSF_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TRANSF_sf_t {
    pub matrix: [[ffi::c_double; 4]; 4],
}

impl Default for TRANSF_sf_t {
    fn default() -> Self {
        Self {
            matrix: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }
}
