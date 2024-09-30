use super::ffi_;
use std::ffi;

/* DV_LOGICAL_t */

pub(crate) type LOGICAL_t = ffi::c_uchar;

pub const TRUE: LOGICAL_t = 1;
pub const FALSE: LOGICAL_t = 0;

#[inline]
pub(crate) fn to_bool(logical: LOGICAL_t) -> bool {
    match logical {
        FALSE => false,
        _ => true,
    }
}

#[inline]
pub(crate) fn from_bool(bool_val: bool) -> LOGICAL_t {
    match bool_val {
        true => TRUE,
        false => FALSE,
    }
}
