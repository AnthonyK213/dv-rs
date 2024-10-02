use num_enum::{IntoPrimitive, TryFromPrimitive};

/* DV_CODE_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum code_e {
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
