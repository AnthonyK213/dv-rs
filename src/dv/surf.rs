use crate::dv::{self, ENTITY, GEOM};
use std::ffi;

extern "C" {
    fn DV_SURF_eval(
        surface: dv::DV_SURF_t,
        uv: dv::UV_t,
        n_u_derivs: ffi::c_int,
        n_v_derivs: ffi::c_int,
        p: *mut dv::VEC3D_t,
    ) -> dv::DV_ERROR_code_t;
}

pub trait SURF: GEOM {
    fn eval(&self, uv: dv::UV_t, n_u_derivs: i32, n_v_derivs: i32) -> dv::DVResult<dv::XYZArray> {
        let mut p = dv::XYZArray::alloc((n_u_derivs + 1) * (n_v_derivs + 1));

        dv::common_::wrap_result(
            unsafe { DV_SURF_eval(self.tag(), uv, n_u_derivs, n_v_derivs, p.as_mut_ptr()) },
            || p,
        )
    }
}

impl From<i32> for dv::SURF_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for dv::SURF_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl GEOM for dv::SURF_t {}

impl SURF for dv::SURF_t {}

#[cfg(test)]
mod tests {
    use crate::dv::{self, SURF};

    #[test]
    fn eval_test() {
        let vertex: Vec<f64> = vec![
            15. * 0.3,
            -10. * 0.3,
            3. * 0.3,
            0.3,
            15. * 1.4,
            0. * 1.4,
            9. * 1.4,
            1.4,
            15. * 2.9,
            10. * 2.9,
            2. * 2.9,
            2.9,
            25. * 1.2,
            -10. * 1.2,
            1. * 1.2,
            1.2,
            25. * 1.2,
            0. * 1.2,
            0. * 1.2,
            1.2,
            25. * 1.2,
            10. * 1.2,
            -6. * 1.2,
            1.2,
            35. * 0.8,
            -10. * 0.8,
            -4. * 0.8,
            0.8,
            35. * 1.1,
            0. * 1.1,
            1. * 1.1,
            1.1,
            35. * 1.8,
            10. * 1.8,
            5. * 1.8,
            1.8,
        ];
        let u_knot_mult: Vec<i32> = vec![3, 3];
        let v_knot_mult: Vec<i32> = vec![3, 3];
        let u_knot: Vec<f64> = vec![0., 1.];
        let v_knot: Vec<f64> = vec![0., 1.];

        let mut bsurf_sf = dv::BSURF_sf_t::new();
        bsurf_sf
            .set_u_degree(2)
            .set_v_degree(2)
            .set_is_rational(true)
            .set_vertex(&vertex, 3, 3, 4)
            .set_u_knot(&u_knot, &u_knot_mult)
            .set_v_knot(&v_knot, &v_knot_mult)
            .set_is_u_periodic(false)
            .set_is_v_periodic(false);

        let bsurf = dv::BSURF_t::create(&bsurf_sf).unwrap();

        let uv = dv::UV_t { x: 0.3, y: 0.6 };

        let p = bsurf.eval(uv, 2, 2).unwrap();
    }
}
