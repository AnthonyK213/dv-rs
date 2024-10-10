use super::object::{self, OBJECT};
use super::topol::{self, TOPOL};
use super::{common_, edge, ffi_, logical_t};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_FIN_ask_edge(fin: ffi_::DV_FIN_t, edge: *mut ffi_::DV_EDGE_t) -> ffi_::DV_ERROR_code_t;

    fn DV_FIN_is_positive(
        fin: ffi_::DV_FIN_t,
        is_positive: *mut logical_t::LOGICAL_t,
    ) -> ffi_::DV_ERROR_code_t;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FIN_t(ffi_::DV_FIN_t);

impl From<i32> for FIN_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl OBJECT for FIN_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl TOPOL for FIN_t {}

impl FIN_t {
    pub fn ask_edge(&self) -> common_::DVResult<edge::EDGE_t> {
        let mut edge = object::NULL;

        common_::wrap_result(unsafe { DV_FIN_ask_edge(self.tag(), &mut edge) }, || {
            edge.into()
        })
    }

    pub fn is_positive(&self) -> common_::DVResult<bool> {
        let mut is_positive = logical_t::FALSE;

        common_::wrap_result(
            unsafe { DV_FIN_is_positive(self.tag(), &mut is_positive) },
            || logical_t::to_bool(is_positive),
        )
    }
}
