use super::{alias_, class, common_, enum_, ffi_};
use std::marker::PhantomData;
use std::ops::Deref;

#[link(name = "differvoid")]
extern "C" {
    fn DV_ENTITY_ask_class(
        entity: ffi_::DV_ENTITY_t,
        class: *mut ffi_::DV_CLASS_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_ENTITY_delete(entity: ffi_::DV_ENTITY_t) -> ffi_::DV_ERROR_code_t;
}

pub(crate) const NULL: i32 = 0;

pub trait ENTITY: From<i32> + Copy + Clone + Eq + PartialEq {
    fn tag(&self) -> i32;

    fn ask_class(&self) -> common_::DVResult<class::CLASS_t> {
        let mut class: ffi_::DV_CLASS_t = enum_::CLASS_e::null.into();

        common_::wrap_result(
            unsafe { DV_ENTITY_ask_class(self.tag(), &mut class) },
            || class.try_into().unwrap(),
        )
    }

    fn delete(&self) -> common_::DVResult<()> {
        common_::wrap_result(unsafe { DV_ENTITY_delete(self.tag()) }, || ())
    }

    fn cast<T: ENTITY>(&self) -> T {
        self.tag().into()
    }

    fn null() -> Self {
        NULL.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ENTITY_t(ffi_::DV_ENTITY_t);

impl From<i32> for ENTITY_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for ENTITY_t {
    fn tag(&self) -> i32 {
        self.0
    }
}
