use crate::dv::{self, ENTITY, GEOM};
use std::ffi;

impl From<i32> for dv::POLY_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for dv::POLY_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl GEOM for dv::POLY_t {}
