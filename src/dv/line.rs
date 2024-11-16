use crate::dv::{self, CURVE, ENTITY, GEOM};

#[link(name = "differvoid")]
extern "C" {
    fn DV_LINE_ask(line: dv::DV_LINE_t, line_sf: *mut dv::LINE_sf_t) -> dv::DV_ERROR_code_t;

    fn DV_LINE_create(
        line_sf: *const dv::LINE_sf_t,
        line: *mut dv::DV_LINE_t,
    ) -> dv::DV_ERROR_code_t;
}

impl dv::LINE_t {
    pub fn ask(&self) -> dv::DVResult<dv::LINE_sf_t> {
        let mut line_sf = dv::LINE_sf_t::default();
        dv::common_::wrap_result(unsafe { DV_LINE_ask(self.0, &mut line_sf) }, || line_sf)
    }

    pub fn create(line_sf: &dv::LINE_sf_t) -> dv::DVResult<Self> {
        let mut line = dv::entity::NULL;

        dv::common_::wrap_result(unsafe { DV_LINE_create(line_sf, &mut line) }, || {
            line.into()
        })
    }
}

impl From<i32> for dv::LINE_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for dv::LINE_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl GEOM for dv::LINE_t {}

impl CURVE for dv::LINE_t {}

#[cfg(test)]
mod tests {
    use crate::dv;

    #[test]
    fn line_test() {
        let mut line_sf = dv::LINE_sf_t::default();
        line_sf.basis_set.axis.x = 1_f64;
        line_sf.basis_set.axis.y = 2_f64;
        line_sf.basis_set.axis.z = 3_f64;
        line_sf.basis_set.location.x = 4_f64;
        line_sf.basis_set.location.y = 5_f64;
        line_sf.basis_set.location.z = 6_f64;

        let line = dv::LINE_t::create(&line_sf).unwrap();

        let line_sf_asked = line.ask().unwrap();

        assert_eq!(line_sf.basis_set.location, line_sf_asked.basis_set.location);
    }
}
