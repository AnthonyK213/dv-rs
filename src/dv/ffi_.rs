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

pub type ALGO_t = ffi::c_int;
pub type BCURVE_form_t = ffi::c_int;
pub type BSURF_form_t = ffi::c_int;
pub type CLASS_t = ffi::c_int;
pub type CODE_t = ffi::c_int;
pub type GEOM_copy_t = ffi::c_int;
pub type LOOP_type_t = ffi::c_int;
pub type boolean_function_t = ffi::c_int;
pub type check_geom_t = ffi::c_int;
pub type check_vx_on_cu_t = ffi::c_int;

/* DV_LOGICAL_t */

pub type LOGICAL_t = ffi::c_uchar;

/* DV_XY_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct XY_t {
    pub x: ffi::c_double,
    pub y: ffi::c_double,
}

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

/********************************* DV_MEMORY **********************************/

#[link(name = "differvoid")]
extern "C" {
    pub(crate) fn DV_MEMORY_alloc(
        nbytes: ffi::c_longlong,
        pointer: *mut *const ffi::c_void,
    ) -> CODE_t;

    pub(crate) fn DV_MEMORY_free(pointer: *const ffi::c_void) -> CODE_t;
}
