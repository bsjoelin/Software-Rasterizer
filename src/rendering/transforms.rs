use crate::vector_math::vector::{Float2, Float3};

pub struct Transform {
    pub yaw: f64,
}

impl Transform {
    pub fn new(rotation_around_z: f64) -> Self {
        Self { yaw: rotation_around_z }
    }

    fn get_basis_vectors(&self) -> (Float3, Float3, Float3) {
        let ihat = Float3::new(self.yaw.cos(), 0.0, self.yaw.sin());
        let jhat = Float3::new(0.0, 1.0, 0.0);
        let khat = Float3::new(-self.yaw.sin(), 0.0, self.yaw.cos());
        (ihat, jhat, khat)
    }

    pub fn vertex_to_world(&self, vertex: &Float3) -> Float3 {
        let (ihat, jhat, khat) = self.get_basis_vectors();
        ihat * vertex.x + jhat * vertex.y + khat * vertex.z
    }
}

/// Transform vertex position into screen-space position [pixel coordinates]
pub fn vertex_to_screen(vertex: &Float3, tranform: &Transform, width: usize, height: usize) -> Float2 {
    let vertex_world = tranform.vertex_to_world(vertex);

    let world_screen_height = 5f64;
    let pixel_factor = height as f64 / world_screen_height;

    let mut pixel_offset = Float2::new(vertex_world.x, vertex_world.y) * pixel_factor;
    pixel_offset += Float2::new(width as f64 / 2f64, height as f64 / 2f64);
    pixel_offset
}