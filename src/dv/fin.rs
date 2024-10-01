use super::{common_, ffi_, logical_t, object};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_FIN_ask_edge(fin: ffi_::FIN_t, edge: *mut ffi_::EDGE_t) -> ffi_::DV_CODE_t;

    fn DV_FIN_is_positive(
        fin: ffi_::FIN_t,
        is_positive: *mut logical_t::LOGICAL_t,
    ) -> ffi_::DV_CODE_t;
}

pub fn ask_edge(fin: ffi_::FIN_t) -> common_::DVResult<ffi_::EDGE_t> {
    let mut edge: ffi_::EDGE_t = object::NULL;
    common_::wrap_result(unsafe { DV_FIN_ask_edge(fin, &mut edge) }, || edge)
}

pub fn is_positive(fin: ffi_::FIN_t) -> common_::DVResult<bool> {
    let mut is_positive = logical_t::FALSE;

    common_::wrap_result(unsafe { DV_FIN_is_positive(fin, &mut is_positive) }, || {
        logical_t::to_bool(is_positive)
    })
}
