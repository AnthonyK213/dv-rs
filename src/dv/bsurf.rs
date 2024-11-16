use crate::dv::{self, ENTITY};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[link(name = "differvoid")]
extern "C" {
    fn DV_BSURF_ask(bsurf: dv::DV_BSURF_t, bsurf_sf: *mut dv::DV_BSURF_sf_t)
        -> dv::DV_ERROR_code_t;

    fn DV_BSURF_create(
        bsurf_sf: *const dv::DV_BSURF_sf_t,
        bsurf: *mut dv::DV_BSURF_t,
    ) -> dv::DV_ERROR_code_t;
}

/* DV_BSURF_form_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum form_e {
    unset_c = 8700,
    arbitrary_c,
    planar_c,
    cylindrical_c,
    conical_c,
    spherical_c,
    toroidal_c,
    revolved_c,
    ruled_c,
    gen_cone_c,
    quadric_c,
    swept_c,
}

impl From<i32> for dv::BSURF_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl dv::ENTITY for dv::BSURF_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl dv::GEOM for dv::BSURF_t {}

impl dv::SURF for dv::BSURF_t {}

impl dv::BSURF_t {
    pub fn ask(&self) -> dv::DVResult<dv::BSURF_sf_t> {
        let mut bsurf_sf = dv::BSURF_sf_t::new();

        dv::common_::wrap_result(
            unsafe { DV_BSURF_ask(self.tag(), bsurf_sf.get_data_mut()) },
            || {
                bsurf_sf.update_cache();
                bsurf_sf
            },
        )
    }

    pub fn create(bsurf_sf: &dv::BSURF_sf_t) -> dv::DVResult<Self> {
        let mut bsurf = dv::entity::NULL;

        dv::common_::wrap_result(
            unsafe { DV_BSURF_create(bsurf_sf.get_data(), &mut bsurf) },
            || bsurf.into(),
        )
    }
}
