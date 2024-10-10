use super::{common_, enum_, error, ffi_, logical_t, CLASS_e};
use std::ops::Deref;

#[link(name = "differvoid")]
extern "C" {
    fn DV_CLASS_ask_superclass(
        class: ffi_::DV_CLASS_t,
        superclass: *mut ffi_::DV_CLASS_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_CLASS_is_subclass(
        may_be_subclass: ffi_::DV_CLASS_t,
        class: ffi_::DV_CLASS_t,
        is_subclass: *mut logical_t::LOGICAL_t,
    ) -> ffi_::DV_ERROR_code_t;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CLASS_t(enum_::CLASS_e);

impl From<enum_::CLASS_e> for CLASS_t {
    fn from(value: enum_::CLASS_e) -> Self {
        Self(value)
    }
}

impl TryFrom<i32> for CLASS_t {
    type Error = error::code_e;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if let Ok(v) = CLASS_e::try_from(value) {
            Ok(v.into())
        } else {
            Err(error::code_e::unset)
        }
    }
}

impl Deref for CLASS_t {
    type Target = enum_::CLASS_e;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CLASS_t {
    pub fn ask_superclass(&self) -> common_::DVResult<CLASS_t> {
        let mut superclass: ffi_::DV_CLASS_t = enum_::CLASS_e::null.into();

        common_::wrap_result(
            unsafe { DV_CLASS_ask_superclass(self.0.into(), &mut superclass) },
            || superclass.try_into().unwrap(),
        )
    }

    pub fn is_subclass(&self, class: CLASS_t) -> common_::DVResult<bool> {
        let mut is_subclass = logical_t::FALSE;

        common_::wrap_result(
            unsafe { DV_CLASS_is_subclass(self.0.into(), class.0.into(), &mut is_subclass) },
            || logical_t::to_bool(is_subclass),
        )
    }
}
