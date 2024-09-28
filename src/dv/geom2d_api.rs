use super::array_::*;
use super::enum_::*;
use super::ffi_::*;
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_GEOM2DAPI_convex_hull(
        n_points: ffi::c_int,
        points: *const PNT2D_t,
        algo: ALGO_t,
        n_convex_points: *mut ffi::c_int,
        convex_indices: *mut *mut ffi::c_int,
        convex_points: *mut *mut PNT2D_t,
    ) -> CODE_t;

    fn DV_GEOM2DAPI_enclosing_disc(
        n_points: ffi::c_int,
        points: *const PNT2D_t,
        origin: *mut PNT2D_t,
        radius: *mut ffi::c_double,
    ) -> CODE_t;
}

pub fn convex_hull(points: &[PNT2D_t], algo: ALGO_e) -> Result<(i32, Int32Array, XYArray), CODE_e> {
    let mut n_convex_points: i32 = 0;
    let mut convex_indices: *mut ffi::c_int = std::ptr::null_mut();
    let mut convex_points: *mut PNT2D_t = std::ptr::null_mut();

    unsafe {
        DV_GEOM2DAPI_convex_hull(
            points.len() as ffi::c_int,
            points.as_ptr(),
            algo.into(),
            &mut n_convex_points,
            &mut convex_indices,
            &mut convex_points,
        )
        .try_into()
        .map_or_else(
            |e| Err(CODE_e::err),
            |v| {
                if v == CODE_e::ok {
                    Ok((
                        n_convex_points,
                        Array::new(convex_indices, n_convex_points),
                        Array::new(convex_points, n_convex_points),
                    ))
                } else {
                    Err(v)
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convex_hull_test() {
        let points: Vec<PNT2D_t> = vec![
            PNT2D_t { x: 0., y: 2. },
            PNT2D_t { x: 1., y: 3. },
            PNT2D_t { x: 2., y: 2. },
            PNT2D_t { x: 2., y: 0. },
            PNT2D_t { x: 3., y: 1. },
            PNT2D_t { x: 3., y: 4. },
            PNT2D_t { x: 4., y: 2. },
            PNT2D_t { x: 4., y: 3. },
        ];

        match convex_hull(&points, ALGO_e::quick_hull_c) {
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
}
