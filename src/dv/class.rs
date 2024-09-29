use super::{common_, enum_, ffi_, logical_t};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_CLASS_ask_superclass(
        class: ffi_::CLASS_t,
        superclass: *mut ffi_::CLASS_t,
    ) -> ffi_::CODE_t;

    fn DV_CLASS_is_subclass(
        may_be_subclass: ffi_::CLASS_t,
        class: ffi_::CLASS_t,
        is_subclass: *mut ffi_::LOGICAL_t,
    ) -> ffi_::CODE_t;
}

pub fn ask_superclass(class: enum_::CLASS_e) -> common_::DVResult<enum_::CLASS_e> {
    let mut superclass: ffi_::CLASS_t = enum_::CLASS_e::null.into();

    common_::wrap_result(
        unsafe { DV_CLASS_ask_superclass(class.into(), &mut superclass) },
        || superclass.try_into().unwrap(),
    )
}

pub fn is_subclass(
    may_be_subclass: enum_::CLASS_e,
    class: enum_::CLASS_e,
) -> common_::DVResult<bool> {
    let mut is_subclass = logical_t::FALSE;

    common_::wrap_result(
        unsafe { DV_CLASS_is_subclass(may_be_subclass.into(), class.into(), &mut is_subclass) },
        || logical_t::to_bool(is_subclass),
    )
}
