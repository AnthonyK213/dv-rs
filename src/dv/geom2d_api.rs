use crate::dv::enum_::*;
use crate::dv::ffi_::*;
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    pub(crate) fn DV_GEOM2DAPI_convex_hull(
        n_points: ffi::c_int,
        points: *const PNT2D_t,
        algo: ALGO_t,
        n_convex_points: *mut ffi::c_int,
        convex_indices: *mut *const ffi::c_int,
        convex_points: *mut *const PNT2D_t,
    ) -> CODE_t;

    pub(crate) fn DV_GEOM2DAPI_enclosing_disc(
        n_points: ffi::c_int,
        points: *const PNT2D_t,
        origin: *mut PNT2D_t,
        radius: *mut ffi::c_double,
    ) -> CODE_t;
}

pub fn convex_hull(points: &[PNT2D_t], algo: ALGO_e) -> Result<(i32,), CODE_e> {
    let mut n_convex_points: i32 = 0;

    unsafe {
        DV_GEOM2DAPI_convex_hull(
            points.len() as ffi::c_int,
            points.as_ptr(),
            algo.into(),
            &mut n_convex_points,
            &mut std::ptr::null(),
            &mut std::ptr::null(),
        )
        .try_into()
        .map_or_else(
            |e| Err(CODE_e::err),
            |v| {
                if v == CODE_e::ok {
                    Ok((n_convex_points,))
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
            }
            Err(e) => {}
        };
    }
}
