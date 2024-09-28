use super::ffi_;
use core::slice;
use std::ffi;
use std::ops::{Deref, Drop, Index, IndexMut};
use std::process::Output;

#[derive(Debug)]
pub struct Array<T> {
    m_data: *mut T,
    m_size: i32,
}

impl<T> Deref for Array<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.m_data, self.m_size as usize) }
    }
}

impl<T> Drop for Array<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi_::DV_MEMORY_free(self.m_data as *const ffi::c_void);
        }
    }
}

impl<T> Index<i32> for Array<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: i32) -> &Self::Output {
        unsafe { &*(self.m_data.offset(index as isize)) }
    }
}

impl<T> IndexMut<i32> for Array<T> {
    #[inline]
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        unsafe { &mut *(self.m_data.offset(index as isize)) }
    }
}

impl<T> Array<T> {
    pub fn new(data: *mut T, size: i32) -> Self {
        Self {
            m_data: data,
            m_size: size,
        }
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        if (index < 0 || index >= self.m_size) {
            None
        } else {
            Some(&self[index])
        }
    }

    pub fn get_mut(&mut self, index: i32) -> Option<&mut T> {
        if (index < 0 || index >= self.m_size) {
            None
        } else {
            Some(&mut self[index])
        }
    }
}

pub type Int32Array = Array<i32>;
pub type TriangleArray = Array<ffi_::TRIANGLE_t>;
pub type XYArray = Array<ffi_::XY_t>;
pub type XYZArray = Array<ffi_::XYZ_t>;
