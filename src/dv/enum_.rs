use num_enum::{IntoPrimitive, TryFromPrimitive};

/* DV_CLASS_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum CLASS_e {
    null = 0,
    class_,
    object,
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
    mesh,
    triangulation,
    point_rep,
    point_on_curve,
    point_on_surface,
    curve_rep,
    curve_on_surface,
    topol,
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
