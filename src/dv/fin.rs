use crate::dv::{self, ENTITY, TOPOL};

#[link(name = "differvoid")]
extern "C" {
    fn DV_FIN_ask_edge(fin: dv::DV_FIN_t, edge: *mut dv::DV_EDGE_t) -> dv::DV_ERROR_code_t;

    fn DV_FIN_is_positive(
        fin: dv::DV_FIN_t,
        is_positive: *mut dv::LOGICAL_t,
    ) -> dv::DV_ERROR_code_t;
}

impl From<i32> for dv::FIN_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for dv::FIN_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl TOPOL for dv::FIN_t {}

impl dv::FIN_t {
    pub fn ask_edge(&self) -> dv::DVResult<dv::EDGE_t> {
        let mut edge = dv::entity::NULL;

        dv::common_::wrap_result(unsafe { DV_FIN_ask_edge(self.tag(), &mut edge) }, || {
            edge.into()
        })
    }

    pub fn is_positive(&self) -> dv::DVResult<bool> {
        let mut is_positive = dv::logical_t::FALSE;

        dv::common_::wrap_result(
            unsafe { DV_FIN_is_positive(self.tag(), &mut is_positive) },
            || dv::logical_t::to_bool(is_positive),
        )
    }
}
