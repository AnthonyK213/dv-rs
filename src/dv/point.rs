use super::{common_, ffi_};

#[link(name = "differvoid")]
extern "C" {
    fn DV_POINT_ask(point: ffi_::POINT_t) -> ffi_::DV_CODE_t;

    fn DV_POINT_create() -> ffi_::DV_CODE_t;
}
