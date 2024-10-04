use super::{bsurf_sf_t, common_, ffi_, object};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[link(name = "differvoid")]
extern "C" {
    fn DV_BSURF_ask(
        bsurf: ffi_::BSURF_t,
        bsurf_sf: *mut bsurf_sf_t::DV_BSURF_sf_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_BSURF_create(
        bsurf_sf: *const bsurf_sf_t::DV_BSURF_sf_t,
        bsurf: *mut ffi_::BSURF_t,
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

pub fn ask(bsurf: ffi_::BSURF_t) -> common_::DVResult<bsurf_sf_t::BSURF_sf_t> {
    let mut bsurf_sf = bsurf_sf_t::BSURF_sf_t::new();

    common_::wrap_result(
        unsafe { DV_BSURF_ask(bsurf, bsurf_sf.get_data_mut()) },
        || {
            bsurf_sf.update_cache();
            bsurf_sf
        },
    )
}

pub fn create(bsurf_sf: &bsurf_sf_t::BSURF_sf_t) -> common_::DVResult<ffi_::BSURF_t> {
    let mut bsurf = object::NULL;

    common_::wrap_result(
        unsafe { DV_BSURF_create(bsurf_sf.get_data(), &mut bsurf) },
        || bsurf,
    )
}
