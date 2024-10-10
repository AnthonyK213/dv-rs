use super::axis2_sf_t;

/* DV_PLANE_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PLANE_sf_t {
    pub basis_set: axis2_sf_t::AXIS2_sf_t,
}
