use super::{enum_, error, ffi_};

pub type DVResult<U> = Result<U, error::code_e>;

#[inline]
pub(crate) fn wrap_result<U, F>(code: ffi_::DV_ERROR_code_t, f: F) -> DVResult<U>
where
    F: FnOnce() -> U,
{
    code.try_into().map_or_else(
        |e| Err(error::code_e::unset),
        |v| {
            if v == error::code_e::ok {
                Ok(f())
            } else {
                Err(v)
            }
        },
    )
}
