use super::geom::{self, GEOM};
use super::object::{self, OBJECT};
use super::surf::{self, SURF};
use super::{bsurf_sf_t, common_, ffi_};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[link(name = "differvoid")]
extern "C" {
    fn DV_BSURF_ask(
        bsurf: ffi_::DV_BSURF_t,
        bsurf_sf: *mut bsurf_sf_t::DV_BSURF_sf_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_BSURF_create(
        bsurf_sf: *const bsurf_sf_t::DV_BSURF_sf_t,
        bsurf: *mut ffi_::DV_BSURF_t,
    ) -> ffi_::DV_ERROR_code_t;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BSURF_t(ffi_::DV_BSURF_t);

impl From<i32> for BSURF_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl OBJECT for BSURF_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl GEOM for BSURF_t {}

impl SURF for BSURF_t {}

impl BSURF_t {
    pub fn ask(&self) -> common_::DVResult<bsurf_sf_t::BSURF_sf_t> {
        let mut bsurf_sf = bsurf_sf_t::BSURF_sf_t::new();

        common_::wrap_result(
            unsafe { DV_BSURF_ask(self.tag(), bsurf_sf.get_data_mut()) },
            || {
                bsurf_sf.update_cache();
                bsurf_sf
            },
        )
    }

    pub fn create(bsurf_sf: &bsurf_sf_t::BSURF_sf_t) -> common_::DVResult<BSURF_t> {
        let mut bsurf = object::NULL;

        common_::wrap_result(
            unsafe { DV_BSURF_create(bsurf_sf.get_data(), &mut bsurf) },
            || bsurf.into(),
        )
    }
}
