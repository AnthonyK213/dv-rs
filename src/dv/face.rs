use crate::dv::{self, ENTITY, TOPOL};

#[link(name = "differvoid")]
extern "C" {
    fn DV_FACE_ask_first_loop(
        face: dv::DV_FACE_t,
        first_loop: *mut dv::DV_LOOP_t,
    ) -> dv::DV_ERROR_code_t;
}

impl From<i32> for dv::FACE_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for dv::FACE_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl TOPOL for dv::FACE_t {}

impl dv::FACE_t {
    pub fn ask_first_loop(&self) -> dv::DVResult<dv::LOOP_t> {
        let mut first_loop = dv::entity::NULL;

        dv::common_::wrap_result(
            unsafe { DV_FACE_ask_first_loop(self.0, &mut first_loop) },
            || first_loop.into(),
        )
    }
}
