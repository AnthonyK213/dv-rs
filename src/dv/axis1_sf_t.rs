use super::xyz_t;

/* DV_AXIS1_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AXIS1_sf_t {
    pub location: xyz_t::PNT3D_t,
    pub axis: xyz_t::VEC3D_t,
}

impl Default for AXIS1_sf_t {
    fn default() -> Self {
        Self {
            location: xyz_t::PNT3D_t {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            axis: xyz_t::VEC3D_t {
                x: 0.,
                y: 0.,
                z: 1.,
            },
        }
    }
}
