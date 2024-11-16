use crate::dv;

pub const TRUE: dv::LOGICAL_t = 1;
pub const FALSE: dv::LOGICAL_t = 0;

#[inline]
pub(crate) fn to_bool(logical: dv::LOGICAL_t) -> bool {
    match logical {
        FALSE => false,
        _ => true,
    }
}

#[inline]
pub(crate) fn from_bool(bool_val: bool) -> dv::LOGICAL_t {
    match bool_val {
        true => TRUE,
        false => FALSE,
    }
}
