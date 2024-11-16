use crate::dv::{self, ENTITY, GEOM};

#[link(name = "differvoid")]
extern "C" {
    fn DV_POINT_ask(point: dv::DV_POINT_t, point_sf: *mut dv::POINT_sf_t) -> dv::DV_ERROR_code_t;

    fn DV_POINT_create(
        point_sf: *const dv::POINT_sf_t,
        point: *mut dv::DV_POINT_t,
    ) -> dv::DV_ERROR_code_t;
}

impl From<i32> for dv::POINT_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for dv::POINT_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl GEOM for dv::POINT_t {}

impl dv::POINT_t {
    pub fn ask(&self) -> dv::DVResult<dv::POINT_sf_t> {
        let mut point_sf = dv::POINT_sf_t::default();
        dv::common_::wrap_result(unsafe { DV_POINT_ask(self.0, &mut point_sf) }, || point_sf)
    }

    pub fn create(point_sf: &dv::POINT_sf_t) -> dv::DVResult<Self> {
        let mut point = dv::entity::NULL;

        dv::common_::wrap_result(unsafe { DV_POINT_create(point_sf, &mut point) }, || {
            point.into()
        })
    }
}
