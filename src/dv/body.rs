use super::{array_, axis2_sf_t, common_, ffi_, object};
use std::ffi;

/* DV_BODY_boolean_o_t */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct DV_BODY_boolean_o_t {
    o_t_version: ffi::c_int,
    function: ffi_::DV_boolean_function_t,
}

#[link(name = "differvoid")]
extern "C" {
    fn DV_BODY_ask_edges(
        body: ffi_::BODY_t,
        n_edges: *mut ffi::c_int,
        edges: *mut *mut ffi_::EDGE_t,
    ) -> ffi_::DV_CODE_t;

    fn DV_BODY_ask_faces(
        body: ffi_::BODY_t,
        n_faces: *mut ffi::c_int,
        faces: *mut *mut ffi_::FACE_t,
    ) -> ffi_::DV_CODE_t;

    fn DV_BODY_ask_fin(
        body: ffi_::BODY_t,
        n_fins: *mut ffi::c_int,
        fins: *mut *mut ffi_::FIN_t,
    ) -> ffi_::DV_CODE_t;

    fn DV_BODY_ask_loops(
        body: ffi_::BODY_t,
        n_loops: *mut ffi::c_int,
        loops: *mut *mut ffi_::LOOP_t,
    ) -> ffi_::DV_CODE_t;

    fn DV_BODY_ask_parent(body: ffi_::BODY_t, parent: *mut ffi_::BODY_t) -> ffi_::DV_CODE_t;

    fn DV_BODY_ask_regions(
        body: ffi_::BODY_t,
        n_regions: *mut ffi::c_int,
        regions: *mut *mut ffi_::REGION_t,
    ) -> ffi_::DV_CODE_t;

    fn DV_BODY_ask_shells(
        body: ffi_::BODY_t,
        n_shells: *mut ffi::c_int,
        shells: *mut *mut ffi_::SHELL_t,
    ) -> ffi_::DV_CODE_t;

    fn DV_BODY_ask_vertices(
        body: ffi_::BODY_t,
        n_vertices: *mut ffi::c_int,
        vertices: *mut *mut ffi_::VERTEX_t,
    ) -> ffi_::DV_CODE_t;

    fn DV_BODY_boolean(
        target: ffi_::BODY_t,
        n_tools: ffi::c_int,
        tools: *const ffi_::BODY_t,
        options: *const DV_BODY_boolean_o_t,
    ) -> ffi_::DV_CODE_t;

    fn DV_BODY_create_solid_block(
        x: ffi::c_double,
        y: ffi::c_double,
        z: ffi::c_double,
        basis_set: *const axis2_sf_t::AXIS2_sf_t,
        body: *mut ffi_::BODY_t,
    ) -> ffi_::DV_CODE_t;
}

pub fn ask_faces(body: ffi_::BODY_t) -> common_::DVResult<array_::Int32Array> {
    let mut n_faces: i32 = 0;
    let mut faces: *mut ffi_::FACE_t = std::ptr::null_mut();

    common_::wrap_result(
        unsafe { DV_BODY_ask_faces(body, &mut n_faces, &mut faces) },
        || array_::Array::new(faces, n_faces),
    )
}

pub fn create_solid_block(
    x: f64,
    y: f64,
    z: f64,
    basis_set: &axis2_sf_t::AXIS2_sf_t,
) -> common_::DVResult<ffi_::BODY_t> {
    let mut body = object::NULL;

    common_::wrap_result(
        unsafe { DV_BODY_create_solid_block(x, y, z, basis_set, &mut body) },
        || body,
    )
}

#[cfg(test)]
mod tests {
    use crate::dv;

    #[test]
    fn create_solid_block_test() {
        let solid_block =
            dv::body::create_solid_block(7., 8., 10., &dv::AXIS2_sf_t::default()).unwrap();

        let faces = dv::body::ask_faces(solid_block).unwrap();
        assert_eq!(6, faces.len());

        let loop_ = dv::face::ask_first_loop(faces[0]).unwrap();
        let fins = dv::loop_::ask_fins(loop_).unwrap();
        assert_eq!(4, fins.len());

        let edge = dv::fin::ask_edge(fins[0]).unwrap();
        let vertices = dv::edge::ask_vertices(edge).unwrap();
        assert_eq!(2, vertices.len());
    }
}
