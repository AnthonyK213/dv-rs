use super::{array_, body, edge, entity, face, fin, loop_, triangle_t, vertex, xy_t, xyz_t};

pub type DoubleArray = array_::Array<f64>;
pub type Int32Array = array_::Array<i32>;
pub type TriangleArray = array_::Array<triangle_t::TRIANGLE_t>;
pub type XYArray = array_::Array<xy_t::XY_t>;
pub type XYZArray = array_::Array<xyz_t::XYZ_t>;

pub type BodyArray = array_::EntityArray<body::BODY_t>;
pub type EdgeArray = array_::EntityArray<edge::EDGE_t>;
pub type FaceArray = array_::EntityArray<face::FACE_t>;
pub type FinArray = array_::EntityArray<fin::FIN_t>;
pub type LoopArray = array_::EntityArray<loop_::LOOP_t>;
pub type EntityArray = array_::EntityArray<entity::ENTITY_t>;
pub type VertexArray = array_::EntityArray<vertex::VERTEX_t>;
