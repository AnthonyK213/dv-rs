use crate::dv;
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    pub(crate) fn DV_MEMORY_alloc(
        nbytes: ffi::c_longlong,
        pointer: *mut *mut ffi::c_void,
    ) -> dv::DV_ERROR_code_t;

    pub(crate) fn DV_MEMORY_free(pointer: *const ffi::c_void) -> dv::DV_ERROR_code_t;
}
