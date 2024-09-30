use super::{common_, ffi_};
use std::ffi;

/* DV_BODY_boolean_o_t */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct BODY_boolean_o_t {
    o_t_version: ffi::c_int,
    function: ffi_::DV_boolean_function_t,
}
