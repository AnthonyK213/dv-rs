use super::{common_, curve, ffi_, geom, line_sf_t, object};

#[link(name = "differvoid")]
extern "C" {
    fn DV_LINE_ask(
        line: ffi_::DV_LINE_t,
        line_sf: *mut line_sf_t::LINE_sf_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_LINE_create(
        line_sf: *const line_sf_t::LINE_sf_t,
        line: *mut ffi_::DV_LINE_t,
    ) -> ffi_::DV_ERROR_code_t;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LINE_t(ffi_::DV_LINE_t);

impl LINE_t {
    pub fn ask(&self) -> common_::DVResult<line_sf_t::LINE_sf_t> {
        let mut line_sf = line_sf_t::LINE_sf_t::default();
        common_::wrap_result(unsafe { DV_LINE_ask(self.0, &mut line_sf) }, || line_sf)
    }

    pub fn create(line_sf: &line_sf_t::LINE_sf_t) -> common_::DVResult<Self> {
        let mut line = object::NULL;

        common_::wrap_result(unsafe { DV_LINE_create(line_sf, &mut line) }, || {
            line.into()
        })
    }
}

impl From<i32> for LINE_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl object::OBJECT for LINE_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl geom::GEOM for LINE_t {}

impl curve::CURVE for LINE_t {}

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

        let line = dv::line::LINE_t::create(&line_sf).unwrap();

        let line_sf_asked = line.ask().unwrap();

        assert_eq!(line_sf.basis_set.location, line_sf_asked.basis_set.location);
    }
}
