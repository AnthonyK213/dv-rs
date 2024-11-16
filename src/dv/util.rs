use crate::dv::{self, ENTITY};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_UTIL_create_tetrasphere(
        center: *const dv::PNT3D_t,
        radius: ffi::c_double,
        level: ffi::c_int,
        tetrasphere: *mut dv::DV_POLY_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_UTIL_points_convex_hull_2d(
        n_points: ffi::c_int,
        points: *const dv::PNT2D_t,
        algo: dv::DV_ALGO_t,
        n_convex_points: *mut ffi::c_int,
        convex_indices: *mut *mut ffi::c_int,
        convex_points: *mut *mut dv::PNT2D_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_UTIL_points_enclosing_disc(
        n_points: ffi::c_int,
        points: *const dv::PNT2D_t,
        origin: *mut dv::PNT2D_t,
        radius: *mut ffi::c_double,
    ) -> dv::DV_ERROR_code_t;
}

pub fn create_tetrasphere(
    center: &dv::PNT3D_t,
    radius: f64,
    level: i32,
) -> dv::DVResult<dv::POLY_t> {
    let mut tetrasphere = dv::entity::NULL;

    dv::common_::wrap_result(
        unsafe { DV_UTIL_create_tetrasphere(center, radius, level, &mut tetrasphere) },
        || tetrasphere.into(),
    )
}

pub fn points_convex_hull_2d(
    points: &[dv::PNT2D_t],
    algo: dv::ALGO_e,
) -> dv::DVResult<(i32, dv::Int32Array, dv::XYArray)> {
    let mut n_convex_points: i32 = 0;
    let mut convex_indices: *mut ffi::c_int = std::ptr::null_mut();
    let mut convex_points: *mut dv::PNT2D_t = std::ptr::null_mut();

    dv::common_::wrap_result(
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
                dv::array_::Array::new(convex_indices, n_convex_points),
                dv::array_::Array::new(convex_points, n_convex_points),
            )
        },
    )
}

pub fn points_enclosing_disc(points: &[dv::PNT2D_t]) -> dv::DVResult<(dv::PNT2D_t, f64)> {
    let mut origin = dv::PNT2D_t { x: 0., y: 0. };
    let mut radius: f64 = 0.;

    dv::common_::wrap_result(
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
