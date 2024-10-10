use super::ffi_;
use super::object::OBJECT;

pub trait TOPOL: OBJECT {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TOPOL_t(ffi_::DV_TOPOL_t);

impl From<i32> for TOPOL_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl OBJECT for TOPOL_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl TOPOL for TOPOL_t {}
