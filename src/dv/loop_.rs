use num_enum::{IntoPrimitive, TryFromPrimitive};

/* DV_LOOP_type_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum type_e {
    vertex_c = 5410,
    outer_c,
    inner_c,
}
