use super::{array_, common_, enum_, ffi_, logical_t};
use std::ffi;

#[derive(Debug)]
pub struct BCURVE_sf_t {
    __data: ffi_::BCURVE_sf_t,
    __vertex: array_::DoubleArray,
    __knot: array_::DoubleArray,
    __knot_mult: array_::Int32Array,
}

impl BCURVE_sf_t {
    pub fn new() -> Self {
        Self {
            __data: ffi_::BCURVE_sf_t {
                degree: 0,
                n_vertices: 0,
                vertex_dim: 0,
                is_rational: logical_t::FALSE,
                vertex: std::ptr::null_mut(),
                form: enum_::BCURVE_form_e::unset_c.into(),
                n_knots: 0,
                knot_mult: std::ptr::null_mut(),
                knot: std::ptr::null_mut(),
                is_periodic: logical_t::FALSE,
                is_closed: logical_t::FALSE,
            },
            __vertex: array_::Array::default(),
            __knot: array_::Array::default(),
            __knot_mult: array_::Array::default(),
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

    pub fn get_form(&self) -> enum_::BCURVE_form_e {
        self.__data.form.try_into().unwrap()
    }

    pub fn get_knot_mult(&self) -> &array_::Int32Array {
        &self.__knot_mult
    }

    pub fn get_knot(&self) -> &array_::DoubleArray {
        &self.__knot
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

    pub fn set_form(&mut self, value: enum_::BCURVE_form_e) -> &mut Self {
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

    pub fn set_is_closed(&mut self, value: bool) -> &mut Self {
        self.__data.is_closed = logical_t::from_bool(value);
        self
    }
}

impl BCURVE_sf_t {
    pub(crate) fn get_data(&self) -> &ffi_::BCURVE_sf_t {
        &self.__data
    }

    pub(crate) fn get_data_mut(&mut self) -> &mut ffi_::BCURVE_sf_t {
        &mut self.__data
    }

    pub(crate) fn update_cache(&mut self) -> &mut Self {
        self.__vertex = array_::Array::new(
            self.__data.vertex,
            self.__data.n_vertices * self.__data.vertex_dim,
        );
        self.__knot = array_::Array::new(self.__data.knot, self.__data.n_knots);
        self.__knot_mult = array_::Array::new(self.__data.knot_mult, self.__data.n_knots);
        self
    }
}