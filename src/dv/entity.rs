use crate::dv;
use std::ops::Deref;

#[link(name = "differvoid")]
extern "C" {
    fn DV_ENTITY_ask_class(
        entity: dv::DV_ENTITY_t,
        class: *mut dv::DV_CLASS_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_ENTITY_delete(entity: dv::DV_ENTITY_t) -> dv::DV_ERROR_code_t;
}

pub(crate) const NULL: i32 = 0;

pub trait ENTITY: From<i32> + Copy + Clone + Eq + PartialEq {
    fn tag(&self) -> i32;

    fn ask_class(&self) -> dv::DVResult<dv::CLASS_t> {
        let mut class: dv::DV_CLASS_t = dv::CLASS_e::null.into();

        dv::common_::wrap_result(
            unsafe { DV_ENTITY_ask_class(self.tag(), &mut class) },
            || class.try_into().unwrap(),
        )
    }

    fn delete(&self) -> dv::DVResult<()> {
        dv::common_::wrap_result(unsafe { DV_ENTITY_delete(self.tag()) }, || ())
    }

    fn cast<T: ENTITY>(&self) -> T {
        self.tag().into()
    }

    fn null() -> Self {
        NULL.into()
    }
}

impl From<i32> for dv::ENTITY_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ENTITY for dv::ENTITY_t {
    fn tag(&self) -> i32 {
        self.0
    }
}
