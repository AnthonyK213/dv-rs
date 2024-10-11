use super::object::{self, OBJECT};
use super::{ffi_, triangle_t, xy_t, xyz_t};
use std::convert::From;
use std::default::Default;
use std::ffi;
use std::marker::PhantomData;
use std::ops::{Deref, Drop, Index, IndexMut};

#[derive(Debug)]
pub struct Array<T> {
    __data: *mut T,
    __size: i32,
}

impl<T> Default for Array<T> {
    fn default() -> Self {
        Self {
            __data: std::ptr::null_mut(),
            __size: 0,
        }
    }
}

impl<T> Deref for Array<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.__data, self.__size as usize) }
    }
}

impl<T> Drop for Array<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi_::DV_MEMORY_free(self.__data as *const ffi::c_void);
        }
    }
}

impl<T: Copy> From<&[T]> for Array<T> {
    fn from(value: &[T]) -> Self {
        let array = Self::alloc(value.len() as i32);
        unsafe {
            array.__data.copy_from(value.as_ptr(), array.len() as usize);
        }
        array
    }
}

impl<T> Index<i32> for Array<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: i32) -> &Self::Output {
        unsafe { &*(self.__data.offset(index as isize)) }
    }
}

impl<T> IndexMut<i32> for Array<T> {
    #[inline]
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        unsafe { &mut *(self.__data.offset(index as isize)) }
    }
}

impl<T> Array<T> {
    pub fn new(data: *mut T, size: i32) -> Self {
        Self {
            __data: data,
            __size: size,
        }
    }

    pub fn alloc(size: i32) -> Self {
        let mut data: *mut ffi::c_void = std::ptr::null_mut();

        unsafe {
            ffi_::DV_MEMORY_alloc(size as i64 * std::mem::size_of::<T>() as i64, &mut data);
        }

        Self {
            __data: data as *mut T,
            __size: size,
        }
    }

    #[inline]
    pub fn len(&self) -> i32 {
        self.__size
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        if index < 0 || index >= self.__size {
            None
        } else {
            Some(&self[index])
        }
    }

    pub fn get_mut(&mut self, index: i32) -> Option<&mut T> {
        if index < 0 || index >= self.__size {
            None
        } else {
            Some(&mut self[index])
        }
    }

    pub(crate) fn as_ptr(&self) -> *const T {
        self.__data
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut T {
        self.__data
    }
}

pub struct ObjectArray<T>
where
    T: OBJECT,
{
    __data: Array<i32>,
    __mark: PhantomData<T>,
}

impl<T> From<&[T]> for ObjectArray<T>
where
    T: OBJECT,
{
    fn from(value: &[T]) -> Self {
        let mut array = Array::<i32>::alloc(value.len() as i32);

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

impl<T> From<Array<i32>> for ObjectArray<T>
where
    T: OBJECT,
{
    fn from(value: Array<i32>) -> Self {
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
    type Target = Array<i32>;

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
