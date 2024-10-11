use super::object::{self, OBJECT};
use super::topol::{self, TOPOL};
use super::{array_, axis2_sf_t, common_, enum_, face, ffi_};
use std::ffi;

/* DV_BODY_boolean_o_t */

#[repr(C)]
#[derive(Debug)]
pub(crate) struct DV_BODY_boolean_o_t {
    o_t_version: ffi::c_int,
    function: ffi_::DV_boolean_function_t,
}

impl Default for DV_BODY_boolean_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            function: enum_::boolean_function_e::unite_c.into(),
        }
    }
}

#[link(name = "differvoid")]
extern "C" {
    fn DV_BODY_ask_edges(
        body: ffi_::DV_BODY_t,
        n_edges: *mut ffi::c_int,
        edges: *mut *mut ffi_::DV_EDGE_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_BODY_ask_faces(
        body: ffi_::DV_BODY_t,
        n_faces: *mut ffi::c_int,
        faces: *mut *mut ffi_::DV_FACE_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_BODY_ask_fin(
        body: ffi_::DV_BODY_t,
        n_fins: *mut ffi::c_int,
        fins: *mut *mut ffi_::DV_FIN_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_BODY_ask_loops(
        body: ffi_::DV_BODY_t,
        n_loops: *mut ffi::c_int,
        loops: *mut *mut ffi_::DV_LOOP_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_BODY_ask_parent(
        body: ffi_::DV_BODY_t,
        parent: *mut ffi_::DV_BODY_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_BODY_ask_regions(
        body: ffi_::DV_BODY_t,
        n_regions: *mut ffi::c_int,
        regions: *mut *mut ffi_::DV_REGION_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_BODY_ask_shells(
        body: ffi_::DV_BODY_t,
        n_shells: *mut ffi::c_int,
        shells: *mut *mut ffi_::DV_SHELL_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_BODY_ask_vertices(
        body: ffi_::DV_BODY_t,
        n_vertices: *mut ffi::c_int,
        vertices: *mut *mut ffi_::DV_VERTEX_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_BODY_boolean(
        target: ffi_::DV_BODY_t,
        n_tools: ffi::c_int,
        tools: *const ffi_::DV_BODY_t,
        options: *const DV_BODY_boolean_o_t,
    ) -> ffi_::DV_ERROR_code_t;

    fn DV_BODY_create_solid_block(
        x: ffi::c_double,
        y: ffi::c_double,
        z: ffi::c_double,
        basis_set: *const axis2_sf_t::AXIS2_sf_t,
        body: *mut ffi_::DV_BODY_t,
    ) -> ffi_::DV_ERROR_code_t;
}

#[derive(Debug, Default)]
pub struct boolean_o_t {
    __data: DV_BODY_boolean_o_t,
}

impl boolean_o_t {
    pub fn get_function(&self) -> enum_::boolean_function_e {
        self.__data.function.try_into().unwrap()
    }

    pub fn set_function(&mut self, value: enum_::boolean_function_e) {
        self.__data.function = value.into();
    }
}

impl boolean_o_t {
    pub(crate) fn get_data(&self) -> &DV_BODY_boolean_o_t {
        &self.__data
    }

    pub(crate) fn get_data_mut(&mut self) -> &mut DV_BODY_boolean_o_t {
        &mut self.__data
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BODY_t(ffi_::DV_BODY_t);

impl From<i32> for BODY_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl OBJECT for BODY_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl TOPOL for BODY_t {}

impl BODY_t {
    pub fn ask_faces(&self) -> common_::DVResult<object::ObjectArray<face::FACE_t>> {
        let mut n_faces: i32 = 0;
        let mut faces: *mut ffi_::DV_FACE_t = std::ptr::null_mut();

        common_::wrap_result(
            unsafe { DV_BODY_ask_faces(self.0, &mut n_faces, &mut faces) },
            || array_::Array::new(faces, n_faces).into(),
        )
    }

    pub fn boolean(&self, tools: &[BODY_t], options: &boolean_o_t) -> common_::DVResult<()> {
        let tool_array: object::ObjectArray<BODY_t> = tools.into();

        common_::wrap_result(
            unsafe {
                DV_BODY_boolean(
                    self.tag(),
                    tool_array.len(),
                    tool_array.as_ptr(),
                    options.get_data(),
                )
            },
            || (),
        )
    }

    pub fn create_solid_block(
        x: f64,
        y: f64,
        z: f64,
        basis_set: &axis2_sf_t::AXIS2_sf_t,
    ) -> common_::DVResult<BODY_t> {
        let mut body = object::NULL;

        common_::wrap_result(
            unsafe { DV_BODY_create_solid_block(x, y, z, basis_set, &mut body) },
            || body.into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::dv;

    #[test]
    fn create_solid_block_test() {
        let solid_block =
            dv::BODY_t::create_solid_block(7., 8., 10., &dv::AXIS2_sf_t::default()).unwrap();

        let faces = solid_block.ask_faces().unwrap();
        assert_eq!(6, faces.len());

        let loop_ = faces.val(0).ask_first_loop().unwrap();
        let fins = loop_.ask_fins().unwrap();
        assert_eq!(4, fins.len());

        let edge = fins.val(0).ask_edge().unwrap();
        let vertices = edge.ask_vertices().unwrap();
    }
}
