use crate::dv::{self, ENTITY, GEOM};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_CURVE_ask_interval(
        curve: dv::DV_CURVE_t,
        interval: *mut dv::INTERVAL_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_CURVE_eval(
        curve: dv::DV_CURVE_t,
        t: ffi::c_double,
        n_derivs: ffi::c_int,
        p: *mut dv::VEC3D_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_CURVE_eval_curvature(
        curve: dv::DV_CURVE_t,
        t: ffi::c_double,
        tangent: *mut dv::VEC3D_t,
        principal_normal: *mut dv::VEC3D_t,
        binormal: *mut dv::VEC3D_t,
        curvature: *mut ffi::c_double,
    ) -> dv::DV_ERROR_code_t;
}

pub trait CURVE: ENTITY {
    fn ask_interval(&self) -> dv::DVResult<dv::INTERVAL_t> {
        let mut interval = dv::INTERVAL_t { t0: 0., t1: 0. };

        dv::common_::wrap_result(
            unsafe { DV_CURVE_ask_interval(self.tag(), &mut interval) },
            || interval,
        )
    }

    fn eval(&self, t: f64, n_derivs: i32) -> dv::DVResult<dv::XYZArray> {
        let mut p = dv::XYZArray::alloc(n_derivs + 1);

        dv::common_::wrap_result(
            unsafe { DV_CURVE_eval(self.tag(), t, n_derivs, p.as_mut_ptr()) },
            || p,
        )
    }

    fn eval_curvature(
        &self,
        t: ffi::c_double,
    ) -> dv::DVResult<(dv::VEC3D_t, dv::VEC3D_t, dv::VEC3D_t, f64)> {
        let mut tangent = dv::VEC3D_t::default();
        let mut principal_normal = dv::VEC3D_t::default();
        let mut binormal = dv::VEC3D_t::default();
        let mut curvature = 0_f64;

        dv::common_::wrap_result(
            unsafe {
                DV_CURVE_eval_curvature(
                    self.tag(),
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
}

impl From<i32> for dv::CURVE_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for dv::CURVE_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl GEOM for dv::CURVE_t {}

impl CURVE for dv::CURVE_t {}

#[cfg(test)]
mod tests {
    use crate::dv::{self, CURVE};

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
            .set_knot(&knot, &knot_mult)
            .set_is_periodic(false);

        let bcurve = dv::BCURVE_t::create(&bcurve_sf).unwrap();

        let t = 0.25;

        let p = bcurve.eval(t, 2).unwrap();
        println!("deriv_0 = {:?}", p[0]);
        println!("deriv_1 = {:?}", p[1]);
        println!("deriv_2 = {:?}", p[2]);

        let (tangent, principal_normal, binormal, curvature) = bcurve.eval_curvature(t).unwrap();
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
