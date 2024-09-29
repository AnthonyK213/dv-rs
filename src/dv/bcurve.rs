use super::{array_, bcurve_sf_t, common_, enum_, ffi_, object};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_BCURVE_create(
        bcurve_sf: *const ffi_::BCURVE_sf_t,
        bcurve: *mut ffi_::BCURVE_t,
    ) -> ffi_::CODE_t;
}

pub fn create(bcurve_sf: &bcurve_sf_t::BCURVE_sf_t) -> common_::DVResult<ffi_::BCURVE_t> {
    let mut bcurve: ffi_::BCURVE_t = object::NULL;

    common_::wrap_result(
        unsafe { DV_BCURVE_create(bcurve_sf.get_data(), &mut bcurve) },
        || bcurve,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_test() {
        let vertex: Vec<f64> = vec![
            -10. * 1.5,
            34. * 1.5,
            6. * 1.5,
            1.5,
            -9. * 2.0,
            15. * 2.0,
            -6. * 2.0,
            2.0,
            -6. * 0.5,
            20. * 0.5,
            1. * 0.5,
            0.5,
            0. * 1.1,
            26. * 1.1,
            2. * 1.1,
            1.1,
            4. * 0.1,
            17. * 0.1,
            -3. * 0.1,
            0.1,
            10.,
            21.,
            10.,
            1.0,
        ];
        let knot: Vec<f64> = vec![0., 0.8, 2.7, 3.];
        let knot_mult: Vec<i32> = vec![4, 1, 1, 4];

        let mut bcurve_sf = bcurve_sf_t::BCURVE_sf_t::new();
        bcurve_sf
            .set_degree(3)
            .set_is_rational(true)
            .set_vertex(&vertex, 4)
            .set_knot(&knot, &knot_mult);

        let bcurve = create(&bcurve_sf).unwrap();

        object::delete(bcurve);
    }
}
