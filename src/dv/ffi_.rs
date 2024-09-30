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
pub(crate) type DV_CODE_t = ffi::c_int;
pub(crate) type DV_GEOM_copy_t = ffi::c_int;
pub(crate) type DV_LOOP_type_t = ffi::c_int;
pub(crate) type DV_boolean_function_t = ffi::c_int;
pub(crate) type DV_check_geom_t = ffi::c_int;
pub(crate) type DV_check_vx_on_cu_t = ffi::c_int;

/* DV_XYZ_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct XYZ_t {
    pub x: ffi::c_double,
    pub y: ffi::c_double,
    pub z: ffi::c_double,
}

/* DV_PNT2D_t */

pub type PNT2D_t = XY_t;

/* DV_VEC2D_t */

pub type VEC2D_t = XY_t;

/* DV_UV_t */

pub type UV_t = XY_t;

/* DV_PNT3D_t */

pub type PNT3D_t = XYZ_t;

/* DV_VEC3D_t */

pub type VEC3D_t = XYZ_t;

/* DV_INTERVAL_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct INTERVAL_t {
    pub t0: ffi::c_double,
    pub t1: ffi::c_double,
}

/* DV_TRIANGLE_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TRIANGLE_t {
    pub v0: ffi::c_int,
    pub v1: ffi::c_int,
    pub v2: ffi::c_int,
}

/******************************* STANDARD FORMS *******************************/

/* DV_AXIS1_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AXIS1_sf_t {
    location: PNT3D_t,
    axis: VEC3D_t,
}

/* DV_AXIS2_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AXIS2_sf_t {
    location: PNT3D_t,
    axis: VEC3D_t,
    ref_direction: VEC3D_t,
}

/* DV_CIRCLE_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CIRCLE_sf_t {
    basis_set: AXIS2_sf_t,
    radius: ffi::c_double,
}

/* DV_LINE_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LINE_sf_t {
    basis_set: AXIS1_sf_t,
}

/* DV_POINT_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct POINT_sf_t {
    position: PNT3D_t,
}

/* DV_PLANE_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PLANE_sf_t {
    basis_set: AXIS2_sf_t,
}

/* DV_POLY_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct POLY_sf_t {
    i_offset: ffi::c_int,
    n_vertices: ffi::c_int,
    n_triangles: ffi::c_int,
    vertex: *mut PNT3D_t,
    triangle: *mut TRIANGLE_t,
}

/* DV_TRANSF_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TRANSF_sf_t {
    matrix: [[ffi::c_double; 4]; 4],
}

/***************************** OPTION STRUCTURES ******************************/

/********************************* DV_MEMORY **********************************/

#[link(name = "differvoid")]
extern "C" {
    pub(crate) fn DV_MEMORY_alloc(
        nbytes: ffi::c_longlong,
        pointer: *mut *mut ffi::c_void,
    ) -> DV_CODE_t;

    pub(crate) fn DV_MEMORY_free(pointer: *const ffi::c_void) -> DV_CODE_t;
}
