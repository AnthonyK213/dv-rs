use super::object::{self, OBJECT};
use super::topol::{self, TOPOL};
use super::{common_, ffi_, loop_};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_FACE_ask_first_loop(
        face: ffi_::DV_FACE_t,
        first_loop: *mut ffi_::DV_LOOP_t,
    ) -> ffi_::DV_ERROR_code_t;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FACE_t(ffi_::DV_FACE_t);

impl From<i32> for FACE_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl object::OBJECT for FACE_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl topol::TOPOL for FACE_t {}

impl FACE_t {
    pub fn ask_first_loop(&self) -> common_::DVResult<loop_::LOOP_t> {
        let mut first_loop = object::NULL;

        common_::wrap_result(
            unsafe { DV_FACE_ask_first_loop(self.0, &mut first_loop) },
            || first_loop.into(),
        )
    }
}
