use super::{array_, body, edge, face, fin, loop_, object, triangle_t, vertex, xy_t, xyz_t};

pub type DoubleArray = array_::Array<f64>;
pub type Int32Array = array_::Array<i32>;
pub type TriangleArray = array_::Array<triangle_t::TRIANGLE_t>;
pub type XYArray = array_::Array<xy_t::XY_t>;
pub type XYZArray = array_::Array<xyz_t::XYZ_t>;

pub type BodyArray = array_::ObjectArray<body::BODY_t>;
pub type EdgeArray = array_::ObjectArray<edge::EDGE_t>;
pub type FaceArray = array_::ObjectArray<face::FACE_t>;
pub type FinArray = array_::ObjectArray<fin::FIN_t>;
pub type LoopArray = array_::ObjectArray<loop_::LOOP_t>;
pub type ObjectArray = array_::ObjectArray<object::OBJECT_t>;
pub type VertexArray = array_::ObjectArray<vertex::VERTEX_t>;
