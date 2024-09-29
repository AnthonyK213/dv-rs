use super::{common_, ffi_};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_OBJECT_ask_class(object: ffi_::OBJECT_t, class: *mut ffi_::CLASS_t) -> ffi_::CODE_t;

    fn DV_OBJECT_delete(object: ffi_::OBJECT_t) -> ffi_::CODE_t;
}

pub const NULL: i32 = 0;

pub fn ask_class(object: ffi_::OBJECT_t) -> common_::DVResult<ffi_::CLASS_t> {
    let mut class: ffi_::CLASS_t = 0;
    common_::wrap_result(unsafe { DV_OBJECT_ask_class(object, &mut class) }, || class)
}

pub fn delete(object: ffi_::OBJECT_t) -> common_::DVResult<()> {
    common_::wrap_result(unsafe { DV_OBJECT_delete(object) }, || ())
}
