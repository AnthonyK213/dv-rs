use num_enum::{IntoPrimitive, TryFromPrimitive};

/* DV_GEOM_copy_e */

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum copy_e {
    always_c = 24770,
    never_c,
    auto_c,
}
