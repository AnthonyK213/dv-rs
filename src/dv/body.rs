use crate::dv::{self, ENTITY};
use std::ffi;

#[link(name = "differvoid")]
extern "C" {
    fn DV_BODY_ask_edges(
        body: dv::DV_BODY_t,
        n_edges: *mut ffi::c_int,
        edges: *mut *mut dv::DV_EDGE_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_BODY_ask_faces(
        body: dv::DV_BODY_t,
        n_faces: *mut ffi::c_int,
        faces: *mut *mut dv::DV_FACE_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_BODY_ask_fin(
        body: dv::DV_BODY_t,
        n_fins: *mut ffi::c_int,
        fins: *mut *mut dv::DV_FIN_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_BODY_ask_loops(
        body: dv::DV_BODY_t,
        n_loops: *mut ffi::c_int,
        loops: *mut *mut dv::DV_LOOP_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_BODY_ask_parent(body: dv::DV_BODY_t, parent: *mut dv::DV_BODY_t) -> dv::DV_ERROR_code_t;

    fn DV_BODY_ask_regions(
        body: dv::DV_BODY_t,
        n_regions: *mut ffi::c_int,
        regions: *mut *mut dv::DV_REGION_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_BODY_ask_shells(
        body: dv::DV_BODY_t,
        n_shells: *mut ffi::c_int,
        shells: *mut *mut dv::DV_SHELL_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_BODY_ask_vertices(
        body: dv::DV_BODY_t,
        n_vertices: *mut ffi::c_int,
        vertices: *mut *mut dv::DV_VERTEX_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_BODY_boolean(
        target: dv::DV_BODY_t,
        n_tools: ffi::c_int,
        tools: *const dv::DV_BODY_t,
        options: *const dv::DV_BODY_boolean_o_t,
    ) -> dv::DV_ERROR_code_t;

    fn DV_BODY_create_solid_block(
        x: ffi::c_double,
        y: ffi::c_double,
        z: ffi::c_double,
        basis_set: *const dv::AXIS2_sf_t,
        body: *mut dv::DV_BODY_t,
    ) -> dv::DV_ERROR_code_t;
}

impl Default for dv::DV_BODY_boolean_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            function: dv::boolean_function_e::unite_c.into(),
        }
    }
}

#[derive(Debug, Default)]
pub struct boolean_o_t {
    __data: dv::DV_BODY_boolean_o_t,
}

impl boolean_o_t {
    pub fn get_function(&self) -> dv::boolean_function_e {
        self.__data.function.try_into().unwrap()
    }

    pub fn set_function(&mut self, value: dv::boolean_function_e) {
        self.__data.function = value.into();
    }
}

impl boolean_o_t {
    pub(crate) fn get_data(&self) -> &dv::DV_BODY_boolean_o_t {
        &self.__data
    }

    pub(crate) fn get_data_mut(&mut self) -> &mut dv::DV_BODY_boolean_o_t {
        &mut self.__data
    }
}

impl From<i32> for dv::BODY_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl dv::ENTITY for dv::BODY_t {
    fn tag(&self) -> i32 {
        self.0
    }
}

impl dv::TOPOL for dv::BODY_t {}

impl dv::BODY_t {
    pub fn ask_faces(&self) -> dv::DVResult<dv::FaceArray> {
        let mut n_faces: i32 = 0;
        let mut faces: *mut dv::DV_FACE_t = std::ptr::null_mut();

        dv::common_::wrap_result(
            unsafe { DV_BODY_ask_faces(self.0, &mut n_faces, &mut faces) },
            || dv::array_::Array::new(faces, n_faces).into(),
        )
    }

    pub fn boolean(
        &self,
        tools: &[dv::BODY_t],
        options: &boolean_o_t,
    ) -> dv::DVResult<()> {
        let tool_array: dv::BodyArray = tools.into();

        dv::common_::wrap_result(
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
        basis_set: &dv::AXIS2_sf_t,
    ) -> dv::DVResult<dv::BODY_t> {
        let mut body = dv::entity::NULL;

        dv::common_::wrap_result(
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
