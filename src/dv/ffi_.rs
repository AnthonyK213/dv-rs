use ::std::ffi;

/********************************** TYPEDEFS **********************************/

pub(crate) type DV_BCURVE_t = ffi::c_int;
pub(crate) type DV_BODY_t = ffi::c_int;
pub(crate) type DV_BSURF_t = ffi::c_int;
pub(crate) type DV_CURVE_t = ffi::c_int;
pub(crate) type DV_EDGE_t = ffi::c_int;
pub(crate) type DV_FACE_t = ffi::c_int;
pub(crate) type DV_FIN_t = ffi::c_int;
pub(crate) type DV_GEOM_t = ffi::c_int;
pub(crate) type DV_LINE_t = ffi::c_int;
pub(crate) type DV_LOOP_t = ffi::c_int;
pub(crate) type DV_MESH_t = ffi::c_int;
pub(crate) type DV_ENTITY_t = ffi::c_int;
pub(crate) type DV_PLANE_t = ffi::c_int;
pub(crate) type DV_POINT_t = ffi::c_int;
pub(crate) type DV_POLY_t = ffi::c_int;
pub(crate) type DV_REGION_t = ffi::c_int;
pub(crate) type DV_SHELL_t = ffi::c_int;
pub(crate) type DV_SURF_t = ffi::c_int;
pub(crate) type DV_TOPOL_t = ffi::c_int;
pub(crate) type DV_TRANSF_t = ffi::c_int;
pub(crate) type DV_VERTEX_t = ffi::c_int;

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
