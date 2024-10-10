use super::{array_, bcurve, enum_, ffi_, logical_t};
use std::ffi;

/* DV_BCURVE_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct DV_BCURVE_sf_t {
    pub degree: ffi::c_int,
    pub n_vertices: ffi::c_int,
    pub vertex_dim: ffi::c_int,
    pub is_rational: logical_t::LOGICAL_t,
    pub vertex: *mut ffi::c_double,
    pub form: ffi_::DV_BCURVE_form_t,
    pub n_knots: ffi::c_int,
    pub knot_mult: *mut ffi::c_int,
    pub knot: *mut ffi::c_double,
    pub knot_type: ffi_::DV_knot_type_t,
    pub is_periodic: logical_t::LOGICAL_t,
    pub is_closed: ffi::c_uchar,
}

impl Default for DV_BCURVE_sf_t {
    fn default() -> Self {
        Self {
            degree: 0,
            n_vertices: 0,
            vertex_dim: 0,
            is_rational: logical_t::FALSE,
            vertex: std::ptr::null_mut(),
            form: bcurve::form_e::unset_c.into(),
            n_knots: 0,
            knot_mult: std::ptr::null_mut(),
            knot: std::ptr::null_mut(),
            knot_type: enum_::knot_type_e::unset_c.into(),
            is_periodic: logical_t::FALSE,
            is_closed: logical_t::FALSE,
        }
    }
}

#[derive(Debug, Default)]
pub struct BCURVE_sf_t {
    __data: DV_BCURVE_sf_t,
    __vertex: array_::DoubleArray,
    __knot: array_::DoubleArray,
    __knot_mult: array_::Int32Array,
}

impl BCURVE_sf_t {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get_degree(&self) -> i32 {
        self.__data.degree
    }

    pub fn get_vertex_dim(&self) -> i32 {
        self.__data.vertex_dim
    }

    pub fn get_is_rational(&self) -> bool {
        logical_t::to_bool(self.__data.is_rational)
    }

    pub fn get_vertex(&self) -> &array_::DoubleArray {
        &self.__vertex
    }

    pub fn get_form(&self) -> bcurve::form_e {
        self.__data.form.try_into().unwrap()
    }

    pub fn get_knot_mult(&self) -> &array_::Int32Array {
        &self.__knot_mult
    }

    pub fn get_knot(&self) -> &array_::DoubleArray {
        &self.__knot
    }

    pub fn get_knot_type(&self) -> enum_::knot_type_e {
        self.__data.knot_type.try_into().unwrap()
    }

    pub fn get_is_periodic(&self) -> bool {
        logical_t::to_bool(self.__data.is_periodic)
    }

    pub fn get_is_closed(&self) -> bool {
        logical_t::to_bool(self.__data.is_closed)
    }

    pub fn set_degree(&mut self, value: i32) -> &mut Self {
        self.__data.degree = value;
        self
    }

    pub fn set_is_rational(&mut self, value: bool) -> &mut Self {
        self.__data.is_rational = logical_t::from_bool(value);
        self
    }

    pub fn set_vertex(&mut self, vertex: &[f64], vertex_dim: i32) -> &mut Self {
        self.__vertex = vertex.into();
        self.__data.vertex_dim = vertex_dim;
        self.__data.n_vertices = self.__vertex.len() / vertex_dim;
        self.__data.vertex = self.__vertex.as_mut_ptr();
        self
    }

    pub fn set_form(&mut self, value: bcurve::form_e) -> &mut Self {
        self.__data.form = value.into();
        self
    }

    pub fn set_knot(&mut self, knot: &[f64], knot_mult: &[i32]) -> &mut Self {
        self.__knot = knot.into();
        self.__knot_mult = knot_mult.into();
        self.__data.n_knots = self.__knot.len();
        self.__data.knot = self.__knot.as_mut_ptr();
        self.__data.knot_mult = self.__knot_mult.as_mut_ptr();
        self
    }

    pub fn set_is_periodic(&mut self, value: bool) -> &mut Self {
        self.__data.is_periodic = logical_t::from_bool(value);
        self
    }
}

impl BCURVE_sf_t {
    pub(crate) fn get_data(&self) -> &DV_BCURVE_sf_t {
        &self.__data
    }

    pub(crate) fn get_data_mut(&mut self) -> &mut DV_BCURVE_sf_t {
        &mut self.__data
    }

    pub(crate) fn update_cache(&mut self) {
        self.__vertex = array_::Array::new(
            self.__data.vertex,
            self.__data.n_vertices * self.__data.vertex_dim,
        );
        self.__knot = array_::Array::new(self.__data.knot, self.__data.n_knots);
        self.__knot_mult = array_::Array::new(self.__data.knot_mult, self.__data.n_knots);
    }
}
