use super::{common_, ffi_, object, point_sf_t};

#[link(name = "differvoid")]
extern "C" {
    fn DV_POINT_ask(point: ffi_::POINT_t, point_sf: *mut point_sf_t::POINT_sf_t)
        -> ffi_::DV_CODE_t;

    fn DV_POINT_create(
        point_sf: *const point_sf_t::POINT_sf_t,
        point: *mut ffi_::POINT_t,
    ) -> ffi_::DV_CODE_t;
}

pub fn ask(point: ffi_::POINT_t) -> common_::DVResult<point_sf_t::POINT_sf_t> {
    let mut point_sf = point_sf_t::POINT_sf_t::default();
    common_::wrap_result(unsafe { DV_POINT_ask(point, &mut point_sf) }, || point_sf)
}

pub fn create(point_sf: &point_sf_t::POINT_sf_t) -> common_::DVResult<ffi_::POINT_t> {
    let mut point = object::NULL;
    common_::wrap_result(unsafe { DV_POINT_create(point_sf, &mut point) }, || point)
}
