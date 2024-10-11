use super::{alias_, array_, bsurf, enum_, ffi_, logical_t};
use std::ffi;

/* DV_BSURF_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct DV_BSURF_sf_t {
    pub u_degree: ffi::c_int,
    pub v_degree: ffi::c_int,
    pub n_u_vertices: ffi::c_int,
    pub n_v_vertices: ffi::c_int,
    pub vertex_dim: ffi::c_int,
    pub is_rational: logical_t::LOGICAL_t,
    pub vertex: *mut ffi::c_double,
    pub form: ffi_::DV_BSURF_form_t,
    pub n_u_knots: ffi::c_int,
    pub n_v_knots: ffi::c_int,
    pub u_knot_mult: *mut ffi::c_int,
    pub v_knot_mult: *mut ffi::c_int,
    pub u_knot: *mut ffi::c_double,
    pub v_knot: *mut ffi::c_double,
    pub u_knot_type: ffi_::DV_knot_type_t,
    pub v_knot_type: ffi_::DV_knot_type_t,
    pub is_u_periodic: logical_t::LOGICAL_t,
    pub is_v_periodic: logical_t::LOGICAL_t,
    pub is_u_closed: logical_t::LOGICAL_t,
    pub is_v_closed: logical_t::LOGICAL_t,
}

impl Default for DV_BSURF_sf_t {
    fn default() -> Self {
        Self {
            u_degree: 0,
            v_degree: 0,
            n_u_vertices: 0,
            n_v_vertices: 0,
            vertex_dim: 0,
            is_rational: logical_t::FALSE,
            vertex: std::ptr::null_mut(),
            form: bsurf::form_e::unset_c.into(),
            n_u_knots: 0,
            n_v_knots: 0,
            u_knot_mult: std::ptr::null_mut(),
            v_knot_mult: std::ptr::null_mut(),
            u_knot: std::ptr::null_mut(),
            v_knot: std::ptr::null_mut(),
            u_knot_type: enum_::knot_type_e::unset_c.into(),
            v_knot_type: enum_::knot_type_e::unset_c.into(),
            is_u_periodic: logical_t::FALSE,
            is_v_periodic: logical_t::FALSE,
            is_u_closed: logical_t::FALSE,
            is_v_closed: logical_t::FALSE,
        }
    }
}

#[derive(Debug, Default)]
pub struct BSURF_sf_t {
    __data: DV_BSURF_sf_t,
    __vertex: alias_::DoubleArray,
    __u_knot_mult: alias_::Int32Array,
    __v_knot_mult: alias_::Int32Array,
    __u_knot: alias_::DoubleArray,
    __v_knot: alias_::DoubleArray,
}

impl BSURF_sf_t {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get_u_degree(&self) -> i32 {
        self.__data.u_degree
    }

    pub fn get_v_degree(&self) -> i32 {
        self.__data.v_degree
    }

    pub fn get_vertex_dim(&self) -> i32 {
        self.__data.vertex_dim
    }

    pub fn get_is_rational(&self) -> bool {
        logical_t::to_bool(self.__data.is_rational)
    }

    pub fn get_n_u_vertices(&self) -> i32 {
        self.__data.n_u_vertices
    }

    pub fn get_n_v_vertices(&self) -> i32 {
        self.__data.n_v_vertices
    }

    pub fn get_vertex(&self) -> &alias_::DoubleArray {
        &self.__vertex
    }

    pub fn get_form(&self) -> bsurf::form_e {
        self.__data.form.try_into().unwrap()
    }

    pub fn get_u_knot_mult(&self) -> &alias_::Int32Array {
        &self.__u_knot_mult
    }

    pub fn get_v_knot_mult(&self) -> &alias_::Int32Array {
        &self.__v_knot_mult
    }

    pub fn get_u_knot(&self) -> &alias_::DoubleArray {
        &self.__u_knot
    }

    pub fn get_v_knot(&self) -> &alias_::DoubleArray {
        &self.__v_knot
    }

    pub fn get_u_knot_type(&self) -> enum_::knot_type_e {
        self.__data.u_knot_type.try_into().unwrap()
    }

    pub fn get_v_knot_type(&self) -> enum_::knot_type_e {
        self.__data.v_knot_type.try_into().unwrap()
    }

    pub fn get_is_u_closed(&self) -> bool {
        logical_t::to_bool(self.__data.is_u_closed)
    }

    pub fn get_is_v_closed(&self) -> bool {
        logical_t::to_bool(self.__data.is_v_closed)
    }

    pub fn set_u_degree(&mut self, value: i32) -> &mut Self {
        self.__data.u_degree = value;
        self
    }

    pub fn set_v_degree(&mut self, value: i32) -> &mut Self {
        self.__data.v_degree = value;
        self
    }

    pub fn set_is_rational(&mut self, value: bool) -> &mut Self {
        self.__data.is_rational = logical_t::from_bool(value);
        self
    }

    pub fn set_vertex(
        &mut self,
        vertex: &[f64],
        n_u_vertices: i32,
        n_v_vertices: i32,
        vertex_dim: i32,
    ) -> &mut Self {
        self.__vertex = vertex.into();
        self.__data.vertex_dim = vertex_dim;
        self.__data.n_u_vertices = n_u_vertices;
        self.__data.n_v_vertices = n_v_vertices;
        self.__data.vertex = self.__vertex.as_mut_ptr();
        self
    }

    pub fn set_form(&mut self, value: bsurf::form_e) -> &mut Self {
        self.__data.form = value.into();
        self
    }

    pub fn set_u_knot(&mut self, u_knot: &[f64], u_knot_mult: &[i32]) -> &mut Self {
        self.__u_knot = u_knot.into();
        self.__u_knot_mult = u_knot_mult.into();
        self.__data.n_u_knots = self.__u_knot.len();
        self.__data.u_knot = self.__u_knot.as_mut_ptr();
        self.__data.u_knot_mult = self.__u_knot_mult.as_mut_ptr();
        self
    }

    pub fn set_v_knot(&mut self, v_knot: &[f64], v_knot_mult: &[i32]) -> &mut Self {
        self.__v_knot = v_knot.into();
        self.__v_knot_mult = v_knot_mult.into();
        self.__data.n_v_knots = self.__v_knot.len();
        self.__data.v_knot = self.__v_knot.as_mut_ptr();
        self.__data.v_knot_mult = self.__v_knot_mult.as_mut_ptr();
        self
    }

    pub fn set_is_u_periodic(&mut self, value: bool) -> &mut Self {
        self.__data.is_u_periodic = logical_t::from_bool(value);
        self
    }

    pub fn set_is_v_periodic(&mut self, value: bool) -> &mut Self {
        self.__data.is_v_periodic = logical_t::from_bool(value);
        self
    }
}

impl BSURF_sf_t {
    pub(crate) fn get_data(&self) -> &DV_BSURF_sf_t {
        &self.__data
    }

    pub(crate) fn get_data_mut(&mut self) -> &mut DV_BSURF_sf_t {
        &mut self.__data
    }

    pub(crate) fn update_cache(&mut self) {
        self.__vertex = array_::Array::new(
            self.__data.vertex,
            self.__data.n_u_vertices * self.__data.n_v_vertices * self.__data.vertex_dim,
        );
        self.__u_knot = array_::Array::new(self.__data.u_knot, self.__data.n_u_knots);
        self.__v_knot = array_::Array::new(self.__data.v_knot, self.__data.n_v_knots);
        self.__u_knot_mult = array_::Array::new(self.__data.u_knot_mult, self.__data.n_u_knots);
        self.__v_knot_mult = array_::Array::new(self.__data.v_knot_mult, self.__data.n_v_knots);
    }
}
