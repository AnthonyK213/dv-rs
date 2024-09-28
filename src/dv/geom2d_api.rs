use super::{array_, common_, enum_, ffi_};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_GEOM2DAPI_convex_hull(
        n_points: ffi::c_int,
        points: *const ffi_::PNT2D_t,
        algo: ffi_::ALGO_t,
        n_convex_points: *mut ffi::c_int,
        convex_indices: *mut *mut ffi::c_int,
        convex_points: *mut *mut ffi_::PNT2D_t,
    ) -> ffi_::CODE_t;

    fn DV_GEOM2DAPI_enclosing_disc(
        n_points: ffi::c_int,
        points: *const ffi_::PNT2D_t,
        origin: *mut ffi_::PNT2D_t,
        radius: *mut ffi::c_double,
    ) -> ffi_::CODE_t;
}

pub fn convex_hull(
    points: &[ffi_::PNT2D_t],
    algo: enum_::ALGO_e,
) -> common_::DVResult<(i32, array_::Int32Array, array_::XYArray)> {
    let mut n_convex_points: i32 = 0;
    let mut convex_indices: *mut ffi::c_int = std::ptr::null_mut();
    let mut convex_points: *mut ffi_::PNT2D_t = std::ptr::null_mut();

    common_::wrap_result(
        unsafe {
            DV_GEOM2DAPI_convex_hull(
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

pub fn enclosing_disc(points: &[ffi_::PNT2D_t]) -> common_::DVResult<(ffi_::PNT2D_t, f64)> {
    let mut origin = ffi_::PNT2D_t { x: 0., y: 0. };
    let mut radius: f64 = 0.;

    common_::wrap_result(
        unsafe {
            DV_GEOM2DAPI_enclosing_disc(
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
    use super::*;

    #[test]
    fn convex_hull_test() {
        let points: Vec<ffi_::PNT2D_t> = vec![
            ffi_::PNT2D_t { x: 0., y: 2. },
            ffi_::PNT2D_t { x: 1., y: 3. },
            ffi_::PNT2D_t { x: 2., y: 2. },
            ffi_::PNT2D_t { x: 2., y: 0. },
            ffi_::PNT2D_t { x: 3., y: 1. },
            ffi_::PNT2D_t { x: 3., y: 4. },
            ffi_::PNT2D_t { x: 4., y: 2. },
            ffi_::PNT2D_t { x: 4., y: 3. },
        ];

        match convex_hull(&points, enum_::ALGO_e::quick_hull_c) {
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
        let points: Vec<ffi_::PNT2D_t> = vec![
            ffi_::PNT2D_t { x: 0., y: 2. },
            ffi_::PNT2D_t { x: 1., y: 3. },
            ffi_::PNT2D_t { x: 2., y: 2. },
            ffi_::PNT2D_t { x: 2., y: 0. },
            ffi_::PNT2D_t { x: 3., y: 1. },
            ffi_::PNT2D_t { x: 3., y: 4. },
            ffi_::PNT2D_t { x: 4., y: 2. },
            ffi_::PNT2D_t { x: 4., y: 3. },
        ];

        match enclosing_disc(&points) {
            Ok(r) => {}
            Err(e) => {
                panic!("Err({:?})", e);
            }
        };
    }
}
