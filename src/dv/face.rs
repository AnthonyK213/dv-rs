use super::{common_, ffi_, object};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_FACE_ask_first_loop(
        face: ffi_::FACE_t,
        first_loop: *mut ffi_::LOOP_t,
    ) -> ffi_::DV_ERROR_code_t;
}

pub fn ask_first_loop(face: ffi_::FACE_t) -> common_::DVResult<ffi_::LOOP_t> {
    let mut first_loop = object::NULL;

    common_::wrap_result(
        unsafe { DV_FACE_ask_first_loop(face, &mut first_loop) },
        || first_loop,
    )
}
