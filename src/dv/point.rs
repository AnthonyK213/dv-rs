use super::{common_, ffi_, geom, object, point_sf_t};

#[link(name = "differvoid")]
extern "C" {
    fn DV_POINT_ask(
        point: ffi_::DV_POINT_t,
        point_sf: *mut point_sf_t::POINT_sf_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_POINT_create(
        point_sf: *const point_sf_t::POINT_sf_t,
        point: *mut ffi_::DV_POINT_t,
    ) -> ffi_::DV_ERROR_code_t;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct POINT_t(ffi_::DV_POINT_t);

impl From<i32> for POINT_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl object::OBJECT_t for POINT_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl geom::GEOM_t for POINT_t {}

impl POINT_t {
    pub fn ask(&self) -> common_::DVResult<point_sf_t::POINT_sf_t> {
        let mut point_sf = point_sf_t::POINT_sf_t::default();
        common_::wrap_result(unsafe { DV_POINT_ask(self.0, &mut point_sf) }, || point_sf)
    }

    pub fn create(point_sf: &point_sf_t::POINT_sf_t) -> common_::DVResult<POINT_t> {
        let mut point = object::NULL;

        common_::wrap_result(unsafe { DV_POINT_create(point_sf, &mut point) }, || {
            point.into()
        })
    }
}
