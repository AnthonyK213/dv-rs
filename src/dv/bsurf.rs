use num_enum::{IntoPrimitive, TryFromPrimitive};

/* DV_BSURF_form_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum form_e {
    unset_c = 8700,
    arbitrary_c,
    planar_c,
    cylindrical_c,
    conical_c,
    spherical_c,
    toroidal_c,
    revolved_c,
    ruled_c,
    gen_cone_c,
    quadric_c,
    swept_c,
}
