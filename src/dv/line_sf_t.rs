use super::axis1_sf_t;

/* DV_LINE_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LINE_sf_t {
    basis_set: axis1_sf_t::AXIS1_sf_t,
}
