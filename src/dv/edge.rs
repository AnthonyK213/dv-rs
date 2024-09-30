use super::{common_, ffi_, logical_t};
use std::ffi;

/* DV_EDGE_attach_curves_o_t */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct DV_EDGE_attach_curves_o_t {
    o_t_version: ffi::c_int,
    n_intervals: ffi::c_int,
    intervals: *const ffi_::INTERVAL_t,
    interval_map: *const ffi::c_int,
    have_senses: logical_t::LOGICAL_t,
    senses: *const logical_t::LOGICAL_t,
    copy_curves: ffi_::DV_GEOM_copy_t,
    vx_checking: ffi_::DV_check_vx_on_cu_t,
    geom_checking: ffi_::DV_check_geom_t,
}
