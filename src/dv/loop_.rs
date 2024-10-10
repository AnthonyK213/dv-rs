use super::{array_, common_, ffi_, object, topol};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::ffi;

/* DV_LOOP_type_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum type_e {
    vertex_c = 5410,
    outer_c,
    inner_c,
}

#[link(name = "differvoid")]
extern "C" {
    fn DV_LOOP_ask_fins(
        loop_: ffi_::DV_LOOP_t,
        n_fins: *mut ffi::c_int,
        fins: *mut *mut ffi_::DV_FIN_t,
    ) -> ffi_::DV_ERROR_code_t;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LOOP_t(ffi_::DV_LOOP_t);

impl From<i32> for LOOP_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl object::OBJECT_t for LOOP_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl topol::TOPOL_t for LOOP_t {}

impl LOOP_t {
    pub fn ask_fins(&self) -> common_::DVResult<array_::Int32Array> {
        let mut n_fins = 0_i32;
        let mut fins: *mut ffi_::DV_FIN_t = std::ptr::null_mut();

        common_::wrap_result(
            unsafe { DV_LOOP_ask_fins(self.0, &mut n_fins, &mut fins) },
            || array_::Array::new(fins, n_fins),
        )
    }
}
