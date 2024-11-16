use crate::dv::{self, ENTITY};

pub trait TOPOL: ENTITY {}

impl From<i32> for dv::TOPOL_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for dv::TOPOL_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl TOPOL for dv::TOPOL_t {}
