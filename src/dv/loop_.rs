use super::{array_, common_, ffi_};
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
        loop_: ffi_::LOOP_t,
        n_fins: *mut ffi::c_int,
        fins: *mut *mut ffi_::FIN_t,
    ) -> ffi_::DV_ERROR_code_t;
}

pub fn ask_fins(loop_: ffi_::LOOP_t) -> common_::DVResult<array_::Int32Array> {
    let mut n_fins = 0_i32;
    let mut fins: *mut ffi_::FIN_t = std::ptr::null_mut();

    common_::wrap_result(
        unsafe { DV_LOOP_ask_fins(loop_, &mut n_fins, &mut fins) },
        || array_::Array::new(fins, n_fins),
    )
}
