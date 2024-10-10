use super::xyz_t;

/* DV_POINT_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct POINT_sf_t {
    pub position: xyz_t::PNT3D_t,
}
