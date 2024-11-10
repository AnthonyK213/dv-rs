use super::entity::{self, ENTITY};
use super::{alias_, array_, common_, enum_, ffi_, poly, xy_t, xyz_t};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_UTIL_create_tetrasphere(
        center: *const xyz_t::PNT3D_t,
        radius: ffi::c_double,
        level: ffi::c_int,
        tetrasphere: *mut ffi_::DV_POLY_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_UTIL_points_convex_hull_2d(
        n_points: ffi::c_int,
        points: *const xy_t::PNT2D_t,
        algo: ffi_::DV_ALGO_t,
        n_convex_points: *mut ffi::c_int,
        convex_indices: *mut *mut ffi::c_int,
        convex_points: *mut *mut xy_t::PNT2D_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_UTIL_points_enclosing_disc(
        n_points: ffi::c_int,
        points: *const xy_t::PNT2D_t,
        origin: *mut xy_t::PNT2D_t,
        radius: *mut ffi::c_double,
    ) -> ffi_::DV_ERROR_code_t;
}

pub fn create_tetrasphere(
    center: &xyz_t::PNT3D_t,
    radius: f64,
    level: i32,
) -> common_::DVResult<poly::POLY_t> {
    let mut tetrasphere = entity::NULL;

    common_::wrap_result(
        unsafe { DV_UTIL_create_tetrasphere(center, radius, level, &mut tetrasphere) },
        || tetrasphere.into(),
    )
}

pub fn points_convex_hull_2d(
    points: &[xy_t::PNT2D_t],
    algo: enum_::ALGO_e,
) -> common_::DVResult<(i32, alias_::Int32Array, alias_::XYArray)> {
    let mut n_convex_points: i32 = 0;
    let mut convex_indices: *mut ffi::c_int = std::ptr::null_mut();
    let mut convex_points: *mut xy_t::PNT2D_t = std::ptr::null_mut();

    common_::wrap_result(
        unsafe {
            DV_UTIL_points_convex_hull_2d(
                points.len() as ffi::c_int,
                points.as_ptr(),
                algo.into(),
                &mut n_convex_points,
                &mut convex_indices,
                &mut convex_points,
            )
        },
        || {
            (
                n_convex_points,
                array_::Array::new(convex_indices, n_convex_points),
                array_::Array::new(convex_points, n_convex_points),
            )
        },
    )
}

pub fn points_enclosing_disc(points: &[xy_t::PNT2D_t]) -> common_::DVResult<(xy_t::PNT2D_t, f64)> {
    let mut origin = xy_t::PNT2D_t { x: 0., y: 0. };
    let mut radius: f64 = 0.;

    common_::wrap_result(
        unsafe {
            DV_UTIL_points_enclosing_disc(
                points.len() as ffi::c_int,
                points.as_ptr(),
                &mut origin,
                &mut radius,
            )
        },
        || (origin, radius),
    )
}

#[cfg(test)]
mod tests {
    use crate::dv;

    #[test]
    fn convex_hull_test() {
        let points: Vec<dv::PNT2D_t> = vec![
            dv::PNT2D_t { x: 0., y: 2. },
            dv::PNT2D_t { x: 1., y: 3. },
            dv::PNT2D_t { x: 2., y: 2. },
            dv::PNT2D_t { x: 2., y: 0. },
            dv::PNT2D_t { x: 3., y: 1. },
            dv::PNT2D_t { x: 3., y: 4. },
            dv::PNT2D_t { x: 4., y: 2. },
            dv::PNT2D_t { x: 4., y: 3. },
        ];

        match dv::util::points_convex_hull_2d(&points, dv::ALGO_e::quick_hull_c) {
            Ok(r) => {
                assert_eq!(7, r.0);
                assert_eq!(4, r.1[2]);
                assert_eq!(points[4], r.2[2]);
                assert_eq!(points[1], r.2[6]);
            }
            Err(e) => {
                panic!("Err({:?})", e);
            }
        };
    }

    #[test]
    fn enclosing_disc_test() {
        let points: Vec<dv::PNT2D_t> = vec![
            dv::PNT2D_t { x: 0., y: 2. },
            dv::PNT2D_t { x: 1., y: 3. },
            dv::PNT2D_t { x: 2., y: 2. },
            dv::PNT2D_t { x: 2., y: 0. },
            dv::PNT2D_t { x: 3., y: 1. },
            dv::PNT2D_t { x: 3., y: 4. },
            dv::PNT2D_t { x: 4., y: 2. },
            dv::PNT2D_t { x: 4., y: 3. },
        ];

        match dv::util::points_enclosing_disc(&points) {
            Ok(r) => {}
            Err(e) => {
                panic!("Err({:?})", e);
            }
        };
    }
}
