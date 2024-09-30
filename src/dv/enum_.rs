use num_enum::{IntoPrimitive, TryFromPrimitive};

/* DV_ALGO_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum ALGO_e {
    quick_hull_c = 0,
    incremental_c,
    graham_scan_c,
    divide_and_conquer_c,
}

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

/* DV_CODE_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum CODE_e {
    ok = 0,
    err,
    not_implemented,
    initialized,
    null_arg_address,
    invalid_value,
    invalid_object,
    invalid_tag,
    still_referenced,
    no_intersection,
    points_are_collinear,
    points_are_coplanar,
    index_out_of_range,
    value_out_of_range,
    insufficient_points,
    insufficient_knots,
    bad_interval,
    zero_interval,
    periodic_open,
    periodic_not_smooth,
    cant_make_nurbs,
    weight_le_0,
    bad_dimension,
    bad_knots,
    poles_weights_not_match,
    knots_mults_not_match,
    invalid_mults,
    geom_not_needed,
    fin_not_in_loop,
    fins_not_distinct,
    edge_not_manifold,
    vertex_not_manifold,
    no_common_vertex,
    bad_vertex,
    cant_get_point,
    unset,
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
