use super::axis1_sf_t;

/* DV_LINE_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct LINE_sf_t {
    pub basis_set: axis1_sf_t::AXIS1_sf_t,
}
