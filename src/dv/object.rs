use super::{array_, class, common_, enum_, ffi_};
use std::marker::PhantomData;
use std::ops::Deref;

#[link(name = "differvoid")]
extern "C" {
    fn DV_OBJECT_ask_class(
        object: ffi_::DV_OBJECT_t,
        class: *mut ffi_::DV_CLASS_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_OBJECT_delete(object: ffi_::DV_OBJECT_t) -> ffi_::DV_ERROR_code_t;
}

pub(crate) const NULL: i32 = 0;

pub trait OBJECT: From<i32> + Copy + Clone + Eq + PartialEq {
    fn tag(&self) -> i32;

    fn ask_class(&self) -> common_::DVResult<class::CLASS_t> {
        let mut class: ffi_::DV_CLASS_t = enum_::CLASS_e::null.into();

        common_::wrap_result(
            unsafe { DV_OBJECT_ask_class(self.tag(), &mut class) },
            || class.try_into().unwrap(),
        )
    }

    fn delete(&self) -> common_::DVResult<()> {
        common_::wrap_result(unsafe { DV_OBJECT_delete(self.tag()) }, || ())
    }

    fn cast<T: OBJECT>(&self) -> T {
        self.tag().into()
    }

    fn null() -> Self {
        NULL.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OBJECT_t(ffi_::DV_OBJECT_t);

impl From<i32> for OBJECT_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl OBJECT for OBJECT_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

pub struct ObjectArray<T>
where
    T: OBJECT,
{
    __data: array_::Int32Array,
    __mark: PhantomData<T>,
}

impl<T> From<&[T]> for ObjectArray<T>
where
    T: OBJECT,
{
    fn from(value: &[T]) -> Self {
        let mut array = array_::Int32Array::alloc(value.len() as i32);

        let mut index: i32 = 0;
        for v in value {
            array[index] = v.tag();
            index = index + 1;
        }

        Self {
            __data: array,
            __mark: PhantomData::default(),
        }
    }
}

impl<T> From<&[i32]> for ObjectArray<T>
where
    T: OBJECT,
{
    fn from(value: &[i32]) -> Self {
        Self {
            __data: value.into(),
            __mark: PhantomData::default(),
        }
    }
}

impl<T> From<array_::Int32Array> for ObjectArray<T>
where
    T: OBJECT,
{
    fn from(value: array_::Int32Array) -> Self {
        Self {
            __data: value,
            __mark: PhantomData::default(),
        }
    }
}

impl<T> Deref for ObjectArray<T>
where
    T: OBJECT,
{
    type Target = array_::Int32Array;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.__data
    }
}

impl<T> ObjectArray<T>
where
    T: OBJECT,
{
    pub fn len(&self) -> i32 {
        self.__data.len()
    }

    pub fn val(&self, index: i32) -> T {
        self.__data[index].into()
    }

    pub fn set_val(&mut self, index: i32, value: T) {
        self.__data[index] = value.tag();
    }
}
