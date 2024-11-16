#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::ffi;

/*********************************** ENUMS ************************************/

/* DV_CLASS_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum CLASS_e {
    null = 0,
    class,
    item,
    entity,
    geometry2d,
    point2d,
    cartesian_point2d,
    vector2d,
    curve2d,
    bounded_curve2d,
    nurbs_curve2d,
    trimmed_curve2d,
    conic2d,
    circle2d,
    ellipse2d,
    hyperbola2d,
    parabola2d,
    line2d,
    offset_curve2d,
    geometry,
    point3d,
    cartesian_point3d,
    vector3d,
    transform3d,
    curve,
    bounded_curve,
    nurbs_curve,
    trimmed_curve,
    conic,
    circle,
    ellipse,
    hyperbola,
    parabola,
    line,
    offset_curve,
    surface,
    bounded_surface,
    nurbs_surface,
    rectangular_trimmed_surface,
    elementary_surface,
    conical_surface,
    cylindrical_surface,
    spherical_surface,
    toroidal_surface,
    plane,
    offset_surface,
    spun_surface,
    swept_surface,
    mesh,
    triangulation,
    point_rep,
    point_on_curve,
    point_on_surface,
    curve_rep,
    curve_on_surface,
    topol,
    part,
    body,
    region,
    shell,
    face,
    loop_,
    fin,
    edge,
    vertex,
}

/* DV_ALGO_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum ALGO_e {
    quick_hull_c = 0,
    incremental_c,
    graham_scan_c,
    divide_and_conquer_c,
}

/* DV_boolean_function_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum boolean_function_e {
    intersect_c = 15901,
    subtract_c,
    unite_c,
}

/* DV_check_geom_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum check_geom_e {
    no_c = 18280,
    basic_c,
    lazy_c,
    full_c,
    yes_c,
}

/* DV_check_vx_on_cu_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum check_vx_on_cu_e {
    all_c = 24760,
    none_c,
    unbounded_c,
}

/* DV_knot_type_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum knot_type_e {
    unset_c = 8500,
    non_uniform_c,
    uniform_c,
    quasi_uniform_c,
    piecewise_bezier_c,
    bezier_ends_c,
    smooth_seam_c,
}

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

/* DV_LOGICAL_t */

pub(crate) type LOGICAL_t = ffi::c_uchar;

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

/******************************* STANDARD FORMS *******************************/

/* DV_AXIS1_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AXIS1_sf_t {
    pub location: PNT3D_t,
    pub axis: VEC3D_t,
}

/* DV_AXIS2_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AXIS2_sf_t {
    pub location: PNT3D_t,
    pub axis: VEC3D_t,
    pub ref_direction: VEC3D_t,
}

/* DV_CIRCLE_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CIRCLE_sf_t {
    pub basis_set: AXIS2_sf_t,
    pub radius: ffi::c_double,
}

/* DV_LINE_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct LINE_sf_t {
    pub basis_set: AXIS1_sf_t,
}

/* DV_BCURVE_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct DV_BCURVE_sf_t {
    pub degree: ffi::c_int,
    pub n_vertices: ffi::c_int,
    pub vertex_dim: ffi::c_int,
    pub is_rational: LOGICAL_t,
    pub vertex: *mut ffi::c_double,
    pub form: DV_BCURVE_form_t,
    pub n_knots: ffi::c_int,
    pub knot_mult: *mut ffi::c_int,
    pub knot: *mut ffi::c_double,
    pub knot_type: DV_knot_type_t,
    pub is_periodic: LOGICAL_t,
    pub is_closed: ffi::c_uchar,
}

/* DV_BSURF_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct DV_BSURF_sf_t {
    pub u_degree: ffi::c_int,
    pub v_degree: ffi::c_int,
    pub n_u_vertices: ffi::c_int,
    pub n_v_vertices: ffi::c_int,
    pub vertex_dim: ffi::c_int,
    pub is_rational: LOGICAL_t,
    pub vertex: *mut ffi::c_double,
    pub form: DV_BSURF_form_t,
    pub n_u_knots: ffi::c_int,
    pub n_v_knots: ffi::c_int,
    pub u_knot_mult: *mut ffi::c_int,
    pub v_knot_mult: *mut ffi::c_int,
    pub u_knot: *mut ffi::c_double,
    pub v_knot: *mut ffi::c_double,
    pub u_knot_type: DV_knot_type_t,
    pub v_knot_type: DV_knot_type_t,
    pub is_u_periodic: LOGICAL_t,
    pub is_v_periodic: LOGICAL_t,
    pub is_u_closed: LOGICAL_t,
    pub is_v_closed: LOGICAL_t,
}

/* DV_POINT_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct POINT_sf_t {
    pub position: PNT3D_t,
}

/* DV_PLANE_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PLANE_sf_t {
    pub basis_set: AXIS2_sf_t,
}

/* DV_POLY_sf_t */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct DV_POLY_sf_t {
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
    pub matrix: [[ffi::c_double; 4]; 4],
}

/***************************** OPTION STRUCTURES ******************************/

/* DV_BODY_boolean_o_t */

#[repr(C)]
#[derive(Debug)]
pub(crate) struct DV_BODY_boolean_o_t {
    o_t_version: ffi::c_int,
    function: DV_boolean_function_t,
}

/* DV_CURVE_make_wire_body_o_t */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct DV_CURVE_make_wire_body_o_t {
    o_t_version: ffi::c_int,
    tolerance: ffi::c_double,
    allow_disjoint: LOGICAL_t,
    allow_general: LOGICAL_t,
    check: LOGICAL_t,
    want_edges: LOGICAL_t,
    want_indices: LOGICAL_t,
}

/* DV_EDGE_attach_curves_o_t */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct DV_EDGE_attach_curves_o_t {
    o_t_version: ffi::c_int,
    n_intervals: ffi::c_int,
    intervals: *const INTERVAL_t,
    interval_map: *const ffi::c_int,
    have_senses: LOGICAL_t,
    senses: *const LOGICAL_t,
    copy_curves: DV_GEOM_copy_t,
    vx_checking: DV_check_vx_on_cu_t,
    geom_checking: DV_check_geom_t,
}

/********************************** TRAITS ************************************/

use curve::CURVE;
use entity::ENTITY;
use geom::GEOM;
use surf::SURF;
use topol::TOPOL;

/********************************** STRUCTS ***********************************/

#[derive(Debug, Default)]
pub struct BCURVE_sf_t {
    __data: DV_BCURVE_sf_t,
    __vertex: DoubleArray,
    __knot: DoubleArray,
    __knot_mult: Int32Array,
}

#[derive(Debug, Default)]
pub struct BSURF_sf_t {
    __data: DV_BSURF_sf_t,
    __vertex: DoubleArray,
    __u_knot_mult: Int32Array,
    __v_knot_mult: Int32Array,
    __u_knot: DoubleArray,
    __v_knot: DoubleArray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ENTITY_t(DV_ENTITY_t);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CLASS_t(CLASS_e);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GEOM_t(DV_GEOM_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct POINT_t(DV_POINT_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CURVE_t(DV_CURVE_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BCURVE_t(DV_BCURVE_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LINE_t(DV_LINE_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SURF_t(DV_SURF_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BSURF_t(DV_BSURF_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct POLY_t(DV_POLY_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TOPOL_t(DV_TOPOL_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BODY_t(DV_BODY_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FACE_t(DV_FACE_t);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LOOP_t(DV_LOOP_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EDGE_t(DV_EDGE_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FIN_t(DV_FIN_t);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VERTEX_t(DV_VERTEX_t);

/********************************* TYPE ALIAS *********************************/

pub type DVResult<U> = Result<U, error::code_e>;

pub type DoubleArray = array_::Array<f64>;
pub type Int32Array = array_::Array<i32>;
pub type TriangleArray = array_::Array<TRIANGLE_t>;
pub type XYArray = array_::Array<XY_t>;
pub type XYZArray = array_::Array<XYZ_t>;

pub type BodyArray = array_::EntityArray<BODY_t>;
pub type EdgeArray = array_::EntityArray<EDGE_t>;
pub type FaceArray = array_::EntityArray<FACE_t>;
pub type FinArray = array_::EntityArray<FIN_t>;
pub type LoopArray = array_::EntityArray<LOOP_t>;
pub type EntityArray = array_::EntityArray<ENTITY_t>;
pub type VertexArray = array_::EntityArray<VERTEX_t>;

/********************************** MODULES ***********************************/

mod array_;
mod common_;

mod axis1_sf_t;
mod axis2_sf_t;
mod bcurve_sf_t;
mod bsurf_sf_t;
mod circle_sf_t;
mod line_sf_t;
mod plane_sf_t;
mod point_sf_t;
mod poly_sf_t;
mod transf_sf_t;

mod face;
mod fin;
mod interval_t;
mod logical_t;
mod memory;
mod surf;
mod topol;
mod triangle_t;
mod vertex;
mod xy_t;
mod xyz_t;

pub mod bcurve;
pub mod body;
pub mod bsurf;
pub mod class;
pub mod curve;
pub mod edge;
pub mod entity;
pub mod error;
pub mod geom;
pub mod line;
pub mod loop_;
pub mod plane;
pub mod point;
pub mod poly;
pub mod util;
