use crate::vector_math::vector::Float3;
use crate::rendering::transforms::Transform;

pub struct Model {
    pub vertices: Vec<Float3>,
    pub triangle_colors: Vec<Float3>,
    pub transform: Transform,
}