use super::{common_, enum_, ffi_};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_OBJECT_ask_class(
        object: ffi_::DV_OBJECT_t,
        class: *mut ffi_::DV_CLASS_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_OBJECT_delete(object: ffi_::DV_OBJECT_t) -> ffi_::DV_ERROR_code_t;
}

pub const NULL: i32 = 0;

pub trait OBJECT_t: From<i32> + Copy + Clone + Eq + PartialEq {
    fn tag(&self) -> i32;

    fn ask_class(&self) -> common_::DVResult<enum_::CLASS_e> {
        let mut class: ffi_::DV_CLASS_t = enum_::CLASS_e::null.into();

        common_::wrap_result(
            unsafe { DV_OBJECT_ask_class(self.tag(), &mut class) },
            || class.try_into().unwrap(),
        )
    }

    fn delete(&self) -> common_::DVResult<()> {
        common_::wrap_result(unsafe { DV_OBJECT_delete(self.tag()) }, || ())
    }
}
