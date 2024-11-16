use crate::dv;

impl Default for dv::AXIS2_sf_t {
    fn default() -> Self {
        Self {
            location: dv::PNT3D_t {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            axis: dv::VEC3D_t {
                x: 0.,
                y: 0.,
                z: 1.,
            },
            ref_direction: dv::VEC3D_t {
                x: 1.,
                y: 0.,
                z: 0.,
            },
        }
    }
}
