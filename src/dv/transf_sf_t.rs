use crate::dv;

impl Default for dv::TRANSF_sf_t {
    fn default() -> Self {
        Self {
            matrix: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }
}
