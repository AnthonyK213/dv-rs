use crate::dv;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[link(name = "differvoid")]
extern "C" {
    fn DV_BCURVE_ask(
        bcurve: dv::DV_BCURVE_t,
        bcurve_sf: *mut dv::DV_BCURVE_sf_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_BCURVE_create(
        bcurve_sf: *const dv::DV_BCURVE_sf_t,
        bcurve: *mut dv::DV_BCURVE_t,
    ) -> dv::DV_ERROR_code_t;
}

/* DV_BCURVE_form_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum form_e {
    unset_c = 8650,
    arbitrary_c,
    polyline_c,
    circular_c,
    elliptic_c,
    parabolic_c,
    hyperbolic_c,
}

impl From<i32> for dv::BCURVE_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl dv::ENTITY for dv::BCURVE_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl dv::GEOM for dv::BCURVE_t {}

impl dv::CURVE for dv::BCURVE_t {}

impl dv::BCURVE_t {
    pub fn ask(&self) -> dv::DVResult<dv::BCURVE_sf_t> {
        let mut bcurve_sf = dv::BCURVE_sf_t::new();

        dv::common_::wrap_result(
            unsafe { DV_BCURVE_ask(self.0, bcurve_sf.get_data_mut()) },
            || {
                bcurve_sf.update_cache();
                bcurve_sf
            },
        )
    }

    pub fn create(bcurve_sf: &dv::BCURVE_sf_t) -> dv::DVResult<Self> {
        let mut bcurve = dv::entity::NULL;

        dv::common_::wrap_result(
            unsafe { DV_BCURVE_create(bcurve_sf.get_data(), &mut bcurve) },
            || bcurve.into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::dv::{self, ENTITY};

    #[test]
    fn create_ask_test() {
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

        let mut bcurve_sf = dv::BCURVE_sf_t::new();
        bcurve_sf
            .set_degree(3)
            .set_is_rational(true)
            .set_vertex(&vertex, 4)
            .set_knot(&knot, &knot_mult);

        let bcurve = dv::BCURVE_t::create(&bcurve_sf).unwrap();

        assert_eq!(dv::CLASS_e::nurbs_curve, *(bcurve.ask_class().unwrap()));

        let bcurve_sf_asked = bcurve.ask().unwrap();

        assert_eq!(bcurve_sf.get_degree(), bcurve_sf_asked.get_degree());
        assert_eq!(bcurve_sf.get_vertex_dim(), bcurve_sf_asked.get_vertex_dim());
        assert_eq!(
            bcurve_sf.get_is_rational(),
            bcurve_sf_asked.get_is_rational()
        );

        let mut vertex_asked = Vec::new();
        vertex_asked.extend_from_slice(bcurve_sf_asked.get_vertex());
        assert_eq!(vertex, vertex_asked);

        let mut knot_asked = Vec::new();
        knot_asked.extend_from_slice(bcurve_sf_asked.get_knot());
        assert_eq!(knot, knot_asked);

        let mut knot_mult_asked = Vec::new();
        knot_mult_asked.extend_from_slice(bcurve_sf_asked.get_knot_mult());
        assert_eq!(knot_mult, knot_mult_asked);

        bcurve.delete().unwrap();
    }
}
