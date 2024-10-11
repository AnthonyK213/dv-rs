use super::geom::GEOM;
use super::object::OBJECT;
use super::{common_, ffi_};
use std::ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct POLY_t(ffi_::DV_POLY_t);

impl From<i32> for POLY_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl OBJECT for POLY_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl GEOM for POLY_t {}
