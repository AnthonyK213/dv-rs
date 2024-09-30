use super::{common_, enum_, ffi_};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_OBJECT_ask_class(object: ffi_::OBJECT_t, class: *mut ffi_::DV_CLASS_t)
        -> ffi_::DV_CODE_t;

    fn DV_OBJECT_delete(object: ffi_::OBJECT_t) -> ffi_::DV_CODE_t;
}

pub const NULL: i32 = 0;

pub fn ask_class(object: ffi_::OBJECT_t) -> common_::DVResult<enum_::CLASS_e> {
    let mut class: ffi_::DV_CLASS_t = enum_::CLASS_e::null.into();
    common_::wrap_result(unsafe { DV_OBJECT_ask_class(object, &mut class) }, || {
        class.try_into().unwrap()
    })
}

pub fn delete(object: ffi_::OBJECT_t) -> common_::DVResult<()> {
    common_::wrap_result(unsafe { DV_OBJECT_delete(object) }, || ())
}
