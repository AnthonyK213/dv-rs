use super::{array_, common_, enum_, ffi_, logical_t};
use std::ffi;

/* DV_CURVE_make_wire_body_o_t */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct make_wire_body_o_s {
    o_t_version: ffi::c_int,
    tolerance: ffi::c_double,
    allow_disjoint: logical_t::LOGICAL_t,
    allow_general: logical_t::LOGICAL_t,
    check: logical_t::LOGICAL_t,
    want_edges: logical_t::LOGICAL_t,
    want_indices: logical_t::LOGICAL_t,
}

#[link(name = "differvoid")]
extern "C" {
    fn DV_CURVE_ask_interval(curve: ffi_::CURVE_t, interval: *mut ffi_::INTERVAL_t)
        -> ffi_::DV_CODE_t;

    fn DV_CURVE_eval(
        curve: ffi_::CURVE_t,
        t: ffi::c_double,
        n_derivs: ffi::c_int,
        p: *mut ffi_::VEC3D_t,
    ) -> ffi_::DV_CODE_t;

    fn DV_CURVE_eval_curvature(
        curve: ffi_::CURVE_t,
        t: ffi::c_double,
        tangent: *mut ffi_::VEC3D_t,
        principal_normal: *mut ffi_::VEC3D_t,
        binormal: *mut ffi_::VEC3D_t,
        curvature: *mut ffi::c_double,
    ) -> ffi_::DV_CODE_t;
}

pub fn ask_interval(curve: ffi_::CURVE_t) -> common_::DVResult<ffi_::INTERVAL_t> {
    let mut interval = ffi_::INTERVAL_t { t0: 0., t1: 0. };

    common_::wrap_result(
        unsafe { DV_CURVE_ask_interval(curve, &mut interval) },
        || interval,
    )
}

pub fn eval(curve: ffi_::CURVE_t, t: f64, n_derivs: i32) -> common_::DVResult<array_::XYZArray> {
    let mut p = array_::XYZArray::alloc(n_derivs + 1);

    common_::wrap_result(
        unsafe { DV_CURVE_eval(curve, t, n_derivs, p.as_mut_ptr()) },
        || p,
    )
}

pub fn eval_curvature(
    curve: ffi_::CURVE_t,
    t: ffi::c_double,
) -> common_::DVResult<(ffi_::VEC3D_t, ffi_::VEC3D_t, ffi_::VEC3D_t, f64)> {
    let mut tangent = ffi_::VEC3D_t {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut principal_normal = ffi_::VEC3D_t {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut binormal = ffi_::VEC3D_t {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut curvature = 0_f64;

    common_::wrap_result(
        unsafe {
            DV_CURVE_eval_curvature(
                curve,
                t,
                &mut tangent,
                &mut principal_normal,
                &mut binormal,
                &mut curvature,
            )
        },
        || (tangent, principal_normal, binormal, curvature),
    )
}

#[cfg(test)]
mod tests {
    use crate::dv;

    #[test]
    fn eval_test() {
        let s = 0.5_f64.sqrt();
        let s10 = s * 10.;

        let vertex: Vec<f64> = vec![
            10.0, 0.0, 1.0, s10, s10, s, 0.0, 10.0, 1.0, -s10, s10, s, -10.0, 0.0, 1.0, -s10, -s10,
            s, 0.0, -10.0, 1.0, s10, -s10, s, 10.0, 0.0, 1.0,
        ];
        let knot: Vec<f64> = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let knot_mult: Vec<i32> = vec![3, 2, 2, 2, 3];

        let mut bcurve_sf = dv::BCURVE_sf_t::new();
        bcurve_sf
            .set_degree(2)
            .set_is_rational(true)
            .set_vertex(&vertex, 3)
            .set_knot(&knot, &knot_mult);

        let bcurve = dv::bcurve::create(&bcurve_sf).unwrap();

        let t = 0.25;

        let p = dv::curve::eval(bcurve, t, 2).unwrap();
        println!("deriv_0 = {:?}", p[0]);
        println!("deriv_1 = {:?}", p[1]);
        println!("deriv_2 = {:?}", p[2]);

        let (tangent, principal_normal, binormal, curvature) =
            dv::curve::eval_curvature(bcurve, t).unwrap();
        assert_eq!(
            dv::VEC3D_t {
                x: -1.,
                y: 0.,
                z: 0.
            },
            tangent
        );
        assert_eq!(
            dv::VEC3D_t {
                x: 0.,
                y: -1.,
                z: 0.
            },
            principal_normal
        );
        assert_eq!(
            dv::VEC3D_t {
                x: 0.,
                y: 0.,
                z: 1.
            },
            binormal
        );
        assert_eq!(0.1, curvature);
    }
}
