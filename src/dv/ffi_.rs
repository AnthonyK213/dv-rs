use ::std::ffi;

/********************************** TYPEDEFS **********************************/

pub type BCURVE_t = ffi::c_int;
pub type BODY_t = ffi::c_int;
pub type BSURF_t = ffi::c_int;
pub type CURVE_t = ffi::c_int;
pub type EDGE_t = ffi::c_int;
pub type FACE_t = ffi::c_int;
pub type FIN_t = ffi::c_int;
pub type GEOM_t = ffi::c_int;
pub type LINE_t = ffi::c_int;
pub type LOOP_t = ffi::c_int;
pub type MESH_t = ffi::c_int;
pub type OBJECT_t = ffi::c_int;
pub type PLANE_t = ffi::c_int;
pub type POINT_t = ffi::c_int;
pub type POLY_t = ffi::c_int;
pub type REGION_t = ffi::c_int;
pub type SHELL_t = ffi::c_int;
pub type SURF_t = ffi::c_int;
pub type TRANSF_t = ffi::c_int;
pub type VERTEX_t = ffi::c_int;

pub(crate) type DV_ALGO_t = ffi::c_int;
pub(crate) type DV_BCURVE_form_t = ffi::c_int;
pub(crate) type DV_BSURF_form_t = ffi::c_int;
pub(crate) type DV_CLASS_t = ffi::c_int;
pub(crate) type DV_ERROR_code_t = ffi::c_int;
pub(crate) type DV_GEOM_copy_t = ffi::c_int;
pub(crate) type DV_LOOP_type_t = ffi::c_int;
pub(crate) type DV_boolean_function_t = ffi::c_int;
pub(crate) type DV_check_geom_t = ffi::c_int;
pub(crate) type DV_check_vx_on_cu_t = ffi::c_int;
pub(crate) type DV_knot_type_t = ffi::c_int;

/********************************* DV_MEMORY **********************************/

#[link(name = "differvoid")]
extern "C" {
    pub(crate) fn DV_MEMORY_alloc(
        nbytes: ffi::c_longlong,
        pointer: *mut *mut ffi::c_void,
    ) -> DV_ERROR_code_t;

    pub(crate) fn DV_MEMORY_free(pointer: *const ffi::c_void) -> DV_ERROR_code_t;
}
