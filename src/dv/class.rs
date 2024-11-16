use crate::dv;
use std::ops::Deref;

#[link(name = "differvoid")]
extern "C" {
    fn DV_CLASS_ask_superclass(
        class: dv::DV_CLASS_t,
        superclass: *mut dv::DV_CLASS_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_CLASS_is_subclass(
        may_be_subclass: dv::DV_CLASS_t,
        class: dv::DV_CLASS_t,
        is_subclass: *mut dv::LOGICAL_t,
    ) -> dv::DV_ERROR_code_t;
}

impl From<dv::CLASS_e> for dv::CLASS_t {
    fn from(value: dv::CLASS_e) -> Self {
        Self(value)
    }
}

impl TryFrom<i32> for dv::CLASS_t {
    type Error = dv::error::code_e;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if let Ok(v) = dv::CLASS_e::try_from(value) {
            Ok(v.into())
        } else {
            Err(dv::error::code_e::unset)
        }
    }
}

impl Deref for dv::CLASS_t {
    type Target = dv::CLASS_e;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl dv::CLASS_t {
    pub fn ask_superclass(&self) -> dv::DVResult<dv::CLASS_t> {
        let mut superclass: dv::DV_CLASS_t = dv::CLASS_e::null.into();

        dv::common_::wrap_result(
            unsafe { DV_CLASS_ask_superclass(self.0.into(), &mut superclass) },
            || superclass.try_into().unwrap(),
        )
    }

    pub fn is_subclass(&self, class: dv::CLASS_t) -> dv::DVResult<bool> {
        let mut is_subclass = dv::logical_t::FALSE;

        dv::common_::wrap_result(
            unsafe { DV_CLASS_is_subclass(self.0.into(), class.0.into(), &mut is_subclass) },
            || dv::logical_t::to_bool(is_subclass),
        )
    }
}
