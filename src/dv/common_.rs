use crate::dv;

#[inline]
pub(crate) fn wrap_result<U, F>(code: dv::DV_ERROR_code_t, f: F) -> dv::DVResult<U>
where
    F: FnOnce() -> U,
{
    code.try_into().map_or_else(
        |e| Err(dv::error::code_e::unset),
        |v| {
            if v == dv::error::code_e::ok {
                Ok(f())
            } else {
                Err(v)
            }
        },
    )
}
