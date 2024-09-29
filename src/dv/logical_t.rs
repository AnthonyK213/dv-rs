use super::ffi_;
use std::ffi;

pub const TRUE: ffi_::LOGICAL_t = 1;
pub const FALSE: ffi_::LOGICAL_t = 0;

#[inline]
pub(crate) fn to_bool(logical: ffi_::LOGICAL_t) -> bool {
    match logical {
        FALSE => false,
        _ => true,
    }
}

#[inline]
pub(crate) fn from_bool(bool_val: bool) -> ffi_::LOGICAL_t {
    match bool_val {
        true => TRUE,
        false => FALSE,
    }
}
