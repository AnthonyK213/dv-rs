use super::{enum_, ffi_};

pub type DVResult<U> = Result<U, enum_::CODE_e>;

#[inline]
pub(crate) fn wrap_result<U, F>(code: ffi_::DV_CODE_t, f: F) -> DVResult<U>
where
    F: FnOnce() -> U,
{
    code.try_into().map_or_else(
        |e| Err(enum_::CODE_e::unset),
        |v| {
            if v == enum_::CODE_e::ok {
                Ok(f())
            } else {
                Err(v)
            }
        },
    )
}
