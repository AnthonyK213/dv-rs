use super::entity::{self, ENTITY};
use super::{common_, ffi_, poly, xyz_t};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_TESSEL_create_tetrasphere(
        center: *const xyz_t::PNT3D_t,
        radius: ffi::c_double,
        level: ffi::c_int,
        tetrasphere: *mut ffi_::DV_POLY_t,
    ) -> ffi_::DV_ERROR_code_t;
}

pub fn create_tetrasphere(
    center: &xyz_t::PNT3D_t,
    radius: f64,
    level: i32,
) -> common_::DVResult<poly::POLY_t> {
    let mut tetrasphere = entity::NULL;

    common_::wrap_result(
        unsafe { DV_TESSEL_create_tetrasphere(center, radius, level, &mut tetrasphere) },
        || tetrasphere.into(),
    )
}
